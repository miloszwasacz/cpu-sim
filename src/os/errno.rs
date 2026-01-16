/*
 * Based on <sys/errno.h> in Newlib
 *
 * https://sourceware.org/git/?p=newlib-cygwin.git;a=blob;f=newlib/libc/include/sys/errno.h;h=7199db0d2e352ce85bb3955ed5c7ab8c3d8d0b42;hb=59891375d74e8e08a89ffe2e0dcd8145e18c7eb5
 */
//! Constants for error codes returned by system calls.
#![allow(unused)]

/// Error code.
pub type Errno = i32;

/// Not owner
pub const EPERM: Errno = 1;
/// No such file or directory
pub const ENOENT: Errno = 2;
/// No such process
pub const ESRCH: Errno = 3;
/// Interrupted system call
pub const EINTR: Errno = 4;
/// I/O error
pub const EIO: Errno = 5;
/// No such device or address
pub const ENXIO: Errno = 6;
/// Arg list too long
pub const E2BIG: Errno = 7;
/// Exec format error
pub const ENOEXEC: Errno = 8;
/// Bad file number
pub const EBADF: Errno = 9;
/// No children
pub const ECHILD: Errno = 10;
/// No more processes
pub const EAGAIN: Errno = 11;
/// Not enough space
pub const ENOMEM: Errno = 12;
/// Permission denied
pub const EACCES: Errno = 13;
/// Bad address
pub const EFAULT: Errno = 14;
/// Device or resource busy
pub const EBUSY: Errno = 16;
/// File exists
pub const EEXIST: Errno = 17;
/// Cross-device link
pub const EXDEV: Errno = 18;
/// No such device
pub const ENODEV: Errno = 19;
/// Not a directory
pub const ENOTDIR: Errno = 20;
/// Is a directory
pub const EISDIR: Errno = 21;
/// Invalid argument
pub const EINVAL: Errno = 22;
/// Too many open files in system
pub const ENFILE: Errno = 23;
/// File descriptor value too large
pub const EMFILE: Errno = 24;
/// Not a character device
pub const ENOTTY: Errno = 25;
/// Text file busy
pub const ETXTBSY: Errno = 26;
/// File too large
pub const EFBIG: Errno = 27;
/// No space left on device
pub const ENOSPC: Errno = 28;
/// Illegal seek
pub const ESPIPE: Errno = 29;
/// Read-only file system
pub const EROFS: Errno = 30;
/// Too many links
pub const EMLINK: Errno = 31;
/// Broken pipe
pub const EPIPE: Errno = 32;
/// Mathematics argument out of domain of function
pub const EDOM: Errno = 33;
/// Result too large
pub const ERANGE: Errno = 34;
/// No message of desired type
pub const ENOMSG: Errno = 35;
/// Identifier removed
pub const EIDRM: Errno = 36;
/// Deadlock
pub const EDEADLK: Errno = 45;
/// No lock
pub const ENOLCK: Errno = 46;
/// Not a stream
pub const ENOSTR: Errno = 60;
/// No data (for no delay io)
pub const ENODATA: Errno = 61;
/// Stream ioctl timeout
pub const ETIME: Errno = 62;
/// No stream resources
pub const ENOSR: Errno = 63;
/// Virtual circuit is gone
pub const ENOLINK: Errno = 67;
/// Protocol error
pub const EPROTO: Errno = 71;
/// Multihop attempted
pub const EMULTIHOP: Errno = 74;
/// Bad message
pub const EBADMSG: Errno = 77;
/// Inappropriate file type or format
pub const EFTYPE: Errno = 79;
/// Function not implemented
pub const ENOSYS: Errno = 88;
/// Directory not empty
pub const ENOTEMPTY: Errno = 90;
/// File or path name too long
pub const ENAMETOOLONG: Errno = 91;
/// Too many symbolic links
pub const ELOOP: Errno = 92;
/// Operation not supported on socket
pub const EOPNOTSUPP: Errno = 95;
/// Protocol family not supported
pub const EPFNOSUPPORT: Errno = 96;
/// Connection reset by peer
pub const ECONNRESET: Errno = 104;
/// No buffer space available
pub const ENOBUFS: Errno = 105;
/// Address family not supported by protocol family
pub const EAFNOSUPPORT: Errno = 106;
/// Protocol wrong type for socket
pub const EPROTOTYPE: Errno = 107;
/// Socket operation on non-socket
pub const ENOTSOCK: Errno = 108;
/// Protocol not available
pub const ENOPROTOOPT: Errno = 109;
/// Connection refused
pub const ECONNREFUSED: Errno = 111;
/// Address already in use
pub const EADDRINUSE: Errno = 112;
/// Software caused connection abort
pub const ECONNABORTED: Errno = 113;
/// Network is unreachable
pub const ENETUNREACH: Errno = 114;
/// Network interface is not configured
pub const ENETDOWN: Errno = 115;
/// Connection timed out
pub const ETIMEDOUT: Errno = 116;
/// Host is down
pub const EHOSTDOWN: Errno = 117;
/// Host is unreachable
pub const EHOSTUNREACH: Errno = 118;
/// Connection already in progress
pub const EINPROGRESS: Errno = 119;
/// Socket already connected
pub const EALREADY: Errno = 120;
/// Destination address required
pub const EDESTADDRREQ: Errno = 121;
/// Message too long
pub const EMSGSIZE: Errno = 122;
/// Unknown protocol
pub const EPROTONOSUPPORT: Errno = 123;
/// Address not available
pub const EADDRNOTAVAIL: Errno = 125;
/// Connection aborted by network
pub const ENETRESET: Errno = 126;
/// Socket is already connected
pub const EISCONN: Errno = 127;
/// Socket is not connected
pub const ENOTCONN: Errno = 128;
pub const ETOOMANYREFS: Errno = 129;
pub const EDQUOT: Errno = 132;
pub const ESTALE: Errno = 133;
/// Not supported
pub const ENOTSUP: Errno = 134;
/// Illegal byte sequence
pub const EILSEQ: Errno = 138;
/// Value too large for defined data type
pub const EOVERFLOW: Errno = 139;
/// Operation canceled
pub const ECANCELED: Errno = 140;
/// State not recoverable
pub const ENOTRECOVERABLE: Errno = 141;
/// Previous owner died
pub const EOWNERDEAD: Errno = 142;
/// Operation would block
pub const EWOULDBLOCK: Errno = EAGAIN;
/// Users can add values starting here
pub const __ELASTERROR: Errno = 2000;
