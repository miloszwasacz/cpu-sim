pub use self::errno::Errno;
pub use self::loader::Loader;
use crate::components::memory::{Address, Memory, MemoryAccess};

use std::fs::File;
use std::io::{self, ErrorKind, Read, Seek, SeekFrom, Write};

mod errno;
mod loader;

macro_rules! assert_fd_valid {
    ($fd:expr) => {
        if $fd < 0 {
            return Err((-1, errno::EBADF));
        }
    };
}

pub type Result = std::result::Result<OsFnResult, (OsFnResult, Errno)>;
pub type OsFnResult = i32;
pub type Fd = i32;

pub struct Os {
    errno_addr: Option<Address>,
    _end_addr: Address,
    heap_end: Address,
    file_table: Vec<Option<File>>,
}

impl Os {
    pub fn new() -> Self {
        Self {
            errno_addr: None,
            _end_addr: 0,
            heap_end: 0,
            file_table: vec![None, None, None],
        }
    }

    fn set_errno_addr(&mut self, addr: Option<Address>) {
        self.errno_addr = addr;
    }

    #[allow(non_snake_case)]
    fn set__end_addr(&mut self, addr: Address) {
        self._end_addr = addr;
    }

    pub fn set_errno(&mut self, mem: &mut Memory, errno: Errno) {
        if let Some(addr) = self.errno_addr {
            mem.set(addr, errno);
        }
    }

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

    pub fn fstat(&mut self, mem: &mut Memory, file: Fd, statbuf: Address) -> Result {
        assert_fd_valid!(file);
        let is_open = |file| {
            self.file_table
                .get(file as usize)
                .map(Option::is_some)
                .unwrap_or(false)
        };

        if file <= 2 || is_open(file) {
            let bytes = &mut mem[statbuf..statbuf + size_of::<libc::stat>() as Address];
            let ptr = bytes.as_mut_ptr() as *mut libc::stat;
            let mut st = unsafe { std::ptr::read(ptr) };
            st.st_mode = libc::S_IFCHR as _;
            unsafe { std::ptr::write(ptr, st) };

            Ok(0)
        } else {
            Err((-1, errno::EBADF))
        }
    }

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

    pub fn read(&mut self, mem: &mut Memory, file: Fd, buf: Address, count: u32) -> Result {
        assert_fd_valid!(file);
        let buf = &mut mem[buf..buf + count as Address];
        match file {
            0 => io::stdin().read(buf),
            1 | 2 => return Err((-1, errno::EBADF)),
            fd => self
                .file_table
                .get_mut(fd as usize)
                .and_then(Option::as_mut)
                .ok_or((-1, errno::EBADF))?
                .read(buf),
        }
        .map(|byte_count| byte_count as _)
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

    pub fn write(&mut self, mem: &Memory, file: Fd, buf: Address, count: u32) -> Result {
        assert_fd_valid!(file);
        let buf = &mem[buf..buf + count as Address];
        match file {
            0 => return Err((-1, errno::EBADF)),
            1 => io::stdout().write(buf),
            2 => io::stderr().write(buf),
            fd => self
                .file_table
                .get_mut(fd as usize)
                .and_then(Option::as_mut)
                .ok_or((-1, errno::EBADF))?
                .write(buf),
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

impl Default for Os {
    fn default() -> Self {
        Self::new()
    }
}
