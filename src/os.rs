pub use self::errno::Errno;
use crate::components::memory::{Address, MemoryReadAccess, MemoryWriteAccess};

use std::fs::File;
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use std::ops::Range;

mod errno;
pub mod loader;

/// Checks if `fd` is non-negative, or returns [`errno::EBADF`].
macro_rules! assert_fd_valid {
    ($fd:expr) => {
        if $fd < 0 {
            return Err((-1, errno::EBADF));
        }
    };
}

pub type Result = std::result::Result<OsFnResult, (OsFnResult, Errno)>;

/// Exit code returned by emulated system calls.
pub type OsFnResult = i32;

/// File descriptor
pub type Fd = i32;

/// Emulated Operating System, used for initial setup of the simulation
/// and when performing an [environment call](crate::instr::Ecall).
pub struct Os<I, O, E> {
    errno_addr: Option<Address>,
    _end_addr: Address,
    heap_end: Address,
    stdin: I,
    stdout: O,
    stderr: E,
    file_table: Vec<Option<File>>,
}

impl<I: Read, O: Write, E: Write> Os<I, O, E> {
    /// Creates a new OS emulation with the provided standard streams.
    pub fn new(stdin: I, stdout: O, stderr: E) -> Self {
        Self {
            errno_addr: None,
            _end_addr: 0,
            heap_end: 0,
            stdin,
            stdout,
            stderr,
            file_table: vec![None, None, None],
        }
    }
}

impl<I, O, E> Os<I, O, E> {
    /// Standard input stream.
    pub fn stdin(&self) -> &I {
        &self.stdin
    }

    /// Mutable access to the standard input stream.
    pub fn stdin_mut(&mut self) -> &mut I {
        &mut self.stdin
    }

    /// Standard output stream.
    pub fn stdout(&self) -> &O {
        &self.stdout
    }

    /// Standard error stream.
    pub fn stderr(&self) -> &E {
        &self.stderr
    }

    /// Saves the address of the global `errno` variable.
    fn set_errno_addr(&mut self, addr: Option<Address>) {
        self.errno_addr = addr;
    }

    /// Sets the `_end_addr` variable, used by the [`sbrk`](Self::sbrk) system call.
    #[allow(non_snake_case)]
    fn set__end_addr(&mut self, addr: Address) {
        self._end_addr = addr;
    }

    /// Sets the global `errno` variable if its location has been previously defined.
    pub fn set_errno(&mut self, mut mem: impl MemoryWriteAccess, errno: Errno) {
        if let Some(addr) = self.errno_addr {
            mem.write(addr, errno);
        }
    }

    /// The `close` Linux system call.
    pub fn close(&mut self, file: Fd) -> Result {
        assert_fd_valid!(file);
        if matches!(file, 0..=2) {
            return Err((-1, errno::EBADF));
        }

        self.file_table
            .get_mut(file as usize)
            .and_then(Option::take)
            .ok_or((-1, errno::EBADF))?
            .sync_all()
            .map(|_| 0)
            .map_err(|err| {
                (
                    -1,
                    match err.kind() {
                        ErrorKind::StorageFull => errno::ENOSPC,
                        // ErrorKind::FilesystemQuotaExceeded => errno::EDQUOT,
                        ErrorKind::Interrupted => errno::EINTR,
                        _ => errno::EIO,
                    },
                )
            })
    }

    /// The `fstat` Linux system call.
    pub fn fstat<M>(&mut self, mut mem: M, file: Fd, statbuf: Address) -> Result
    where
        M: MemoryReadAccess + MemoryWriteAccess,
    {
        assert_fd_valid!(file);
        let is_open = |file| {
            self.file_table
                .get(file as usize)
                .map(Option::is_some)
                .unwrap_or(false)
        };

        if file <= 2 || is_open(file) {
            let addr_range = statbuf..statbuf + size_of::<libc::stat>() as Address;
            let mut bytes = read_bytes(&mut mem, addr_range.clone());
            let ptr = bytes.as_mut_ptr() as *mut libc::stat;
            let mut st = unsafe { std::ptr::read(ptr) };
            st.st_mode = libc::S_IFCHR as _;
            unsafe { std::ptr::write(ptr, st) };
            write_bytes(mem, addr_range, &bytes);

            Ok(0)
        } else {
            Err((-1, errno::EBADF))
        }
    }

    /// The `lseek` Linux system call.
    pub fn lseek(&mut self, file: Fd, offset: i32, whence: i32) -> Result {
        const SEEK_SET: i32 = 0;
        const SEEK_CUR: i32 = 1;
        const SEEK_END: i32 = 2;

        assert_fd_valid!(file);
        if matches!(file, 0..=2) {
            return Err((-1, errno::ESPIPE));
        }

        self.file_table
            .get_mut(file as usize)
            .and_then(Option::as_mut)
            .ok_or((-1, errno::EBADF))?
            .seek(match whence {
                SEEK_SET => SeekFrom::Start(offset as _),
                SEEK_CUR => SeekFrom::Current(offset as _),
                SEEK_END => SeekFrom::End(offset as _),
                _ => return Err((-1, errno::EINVAL)),
            })
            .map(|offset| offset as u32 as _)
            .map_err(|err| {
                (
                    -1,
                    match err.kind() {
                        ErrorKind::NotSeekable => errno::ESPIPE,
                        _ => errno::EOVERFLOW,
                    },
                )
            })
    }
}

impl<I: Read, O, E> Os<I, O, E> {
    /// The `read` Linux system call.
    pub fn read<M>(&mut self, mut mem: M, file: Fd, buf: Address, count: u32) -> Result
    where
        M: MemoryReadAccess + MemoryWriteAccess,
    {
        assert_fd_valid!(file);
        let addr_range = buf..buf + count as Address;
        let mut buf = read_bytes(&mut mem, addr_range.clone());
        match file {
            0 => self.stdin.read(&mut buf),
            1 | 2 => return Err((-1, errno::EBADF)),
            fd => self
                .file_table
                .get_mut(fd as usize)
                .and_then(Option::as_mut)
                .ok_or((-1, errno::EBADF))?
                .read(&mut buf),
        }
        .map(|byte_count| {
            write_bytes(mem, addr_range, &buf);
            byte_count as _
        })
        .map_err(|err| {
            (
                -1,
                match err.kind() {
                    ErrorKind::AddrNotAvailable => errno::EFAULT,
                    ErrorKind::WouldBlock => errno::EAGAIN,
                    ErrorKind::IsADirectory => errno::EISDIR,
                    ErrorKind::InvalidInput => errno::EINVAL,
                    ErrorKind::Interrupted => errno::EINTR,
                    _ => errno::EIO,
                },
            )
        })
    }

    /// The `sbrk` Linux system call.
    pub fn sbrk(&mut self, stack_ptr: Address, incr: i32) -> Result {
        if self.heap_end == 0 {
            self.heap_end = self._end_addr;
        }

        let prev_heap_end = self.heap_end;
        self.heap_end = (self.heap_end as i32 + incr) as _;
        if self.heap_end > stack_ptr {
            // TODO Make this an exception
            eprintln!(
                "Heap and stack collision: {:05x}, {:05x}",
                self.heap_end, stack_ptr
            );
            return Err((-1, errno::ENOMEM));
        }

        Ok(prev_heap_end as _)
    }
}

impl<I, O: Write, E: Write> Os<I, O, E> {
    /// The `write` Linux system call.
    pub fn write<M>(&mut self, mut mem: M, file: Fd, buf: Address, count: u32) -> Result
    where
        M: MemoryReadAccess,
    {
        assert_fd_valid!(file);
        let addr_range = buf..buf + count as Address;
        let buf = read_bytes(&mut mem, addr_range.clone());
        match file {
            0 => return Err((-1, errno::EBADF)),
            1 => self.stdout.write(&buf),
            2 => self.stderr.write(&buf),
            fd => self
                .file_table
                .get_mut(fd as usize)
                .and_then(Option::as_mut)
                .ok_or((-1, errno::EBADF))?
                .write(&buf),
        }
        .map(|byte_count| byte_count as _)
        .map_err(|err| {
            (
                -1,
                match err.kind() {
                    ErrorKind::PermissionDenied => errno::EPERM,
                    ErrorKind::AddrNotAvailable => errno::EFAULT,
                    ErrorKind::BrokenPipe => errno::EPIPE,
                    ErrorKind::WouldBlock => errno::EAGAIN,
                    ErrorKind::ReadOnlyFilesystem => errno::EINVAL,
                    ErrorKind::StorageFull => errno::ENOSPC,
                    // ErrorKind::FilesystemQuotaExceeded => errno::EDQUOT,
                    ErrorKind::FileTooLarge => errno::EFBIG,
                    ErrorKind::Interrupted => errno::EINTR,
                    _ => errno::EIO,
                },
            )
        })
    }
}

/// Reads bytes from `mem` at the specified `addr_range`
fn read_bytes<M: MemoryReadAccess>(mut mem: M, addr_range: Range<Address>) -> Vec<u8> {
    let mut bytes = Vec::<u8>::with_capacity((addr_range.end - addr_range.start) as usize);
    for addr in addr_range {
        bytes.push(mem.read(addr))
    }
    bytes
}

/// Writes `data` to `mem` at the specified `addr_range`
fn write_bytes<M: MemoryWriteAccess>(mut mem: M, addr_range: Range<Address>, data: &[u8]) {
    for (addr, byte) in addr_range.zip(data) {
        mem.write(addr, *byte);
    }
}
