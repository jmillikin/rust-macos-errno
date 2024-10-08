// Copyright (c) 2024 John Millikin <john@john-millikin.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
// AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
// LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
// OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.
//
// SPDX-License-Identifier: 0BSD

//! This library defines an [Error] struct that represents error numbers
//! returned from macOS system calls.

#![no_std]

use core::{fmt, num};
use core::convert::TryFrom;

/// Type for error numbers returned from macOS system calls.
///
/// The `Error` type implements `PartialEq` for many integer types, and
/// (optionally) with the POSIX error numbers defined in the [`posix-errno`]
/// library.
///
/// [`posix-errno`]: https://crates.io/crates/posix-errno
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error(num::NonZeroI32);

impl Error {
	/// Create a new error from a raw error number. If `errno` is zero,
	/// returns `None`.
	pub const fn new(errno: i32) -> Option<Error> {
		match num::NonZeroI32::new(errno) {
			Some(n) => Some(Self(n)),
			None => errno_out_of_range(),
		}
	}

	/// Unsafely create a new error from a raw error number.
	///
	/// # Safety
	///
	/// The caller must ensure that `errno` is non-zero.
	#[inline]
	pub const unsafe fn new_unchecked(errno: i32) -> Error {
		Error(num::NonZeroI32::new_unchecked(errno))
	}

	/// Returns the error number as a primitive `i32`.
	#[inline]
	pub const fn get(&self) -> i32 {
		self.0.get()
	}

	/// Returns the error number as a [`NonZeroI32`](num::NonZeroI32).
	#[inline]
	pub const fn get_nonzero(&self) -> num::NonZeroI32 {
		self.0
	}
}

#[cold]
#[inline]
const fn errno_out_of_range() -> Option<Error> {
	None
}

impl From<Error> for i32 {
	#[inline]
	fn from(err: Error) -> i32 {
		err.0.get()
	}
}

impl From<Error> for num::NonZeroI32 {
	#[inline]
	fn from(err: Error) -> num::NonZeroI32 {
		err.0
	}
}

impl From<Error> for i64 {
	#[inline]
	fn from(err: Error) -> i64 {
		err.0.get().into()
	}
}

impl From<Error> for num::NonZeroI64 {
	#[inline]
	fn from(err: Error) -> num::NonZeroI64 {
		err.0.into()
	}
}

impl fmt::Binary for Error {
	#[inline]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(fmt)
	}
}

impl fmt::LowerHex for Error {
	#[inline]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(fmt)
	}
}

impl fmt::UpperHex for Error {
	#[inline]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(fmt)
	}
}

impl PartialEq<u64> for Error {
	#[inline]
	fn eq(&self, other: &u64) -> bool {
		u64::try_from(self.0.get()).map_or(false, |x| x == *other)
	}
}

impl PartialEq<Error> for u64 {
	#[inline]
	fn eq(&self, other: &Error) -> bool {
		u64::try_from(other.0.get()).map_or(false, |x| x == *self)
	}
}

impl PartialEq<usize> for Error {
	#[inline]
	fn eq(&self, other: &usize) -> bool {
		usize::try_from(self.0.get()).map_or(false, |x| x == *other)
	}
}

impl PartialEq<Error> for usize {
	#[inline]
	fn eq(&self, other: &Error) -> bool {
		usize::try_from(other.0.get()).map_or(false, |x| x == *self)
	}
}

impl PartialEq<isize> for Error {
	#[inline]
	fn eq(&self, other: &isize) -> bool {
		isize::try_from(self.0.get()).map_or(false, |x| x == *other)
	}
}

impl PartialEq<Error> for isize {
	#[inline]
	fn eq(&self, other: &Error) -> bool {
		isize::try_from(other.0.get()).map_or(false, |x| x == *self)
	}
}

impl PartialEq<num::NonZeroU64> for Error {
	#[inline]
	fn eq(&self, other: &num::NonZeroU64) -> bool {
		u64::try_from(self.0.get()).map_or(false, |x| x == other.get())
	}
}

impl PartialEq<Error> for num::NonZeroU64 {
	#[inline]
	fn eq(&self, other: &Error) -> bool {
		u64::try_from(other.0.get()).map_or(false, |x| x == self.get())
	}
}

impl PartialEq<num::NonZeroUsize> for Error {
	#[inline]
	fn eq(&self, other: &num::NonZeroUsize) -> bool {
		usize::try_from(self.0.get()).map_or(false, |x| x == other.get())
	}
}

impl PartialEq<Error> for num::NonZeroUsize {
	#[inline]
	fn eq(&self, other: &Error) -> bool {
		usize::try_from(other.0.get()).map_or(false, |x| x == self.get())
	}
}

impl PartialEq<num::NonZeroIsize> for Error {
	#[inline]
	fn eq(&self, other: &num::NonZeroIsize) -> bool {
		isize::try_from(self.0.get()).map_or(false, |x| x == other.get())
	}
}

impl PartialEq<Error> for num::NonZeroIsize {
	#[inline]
	fn eq(&self, other: &Error) -> bool {
		isize::try_from(other.0.get()).map_or(false, |x| x == self.get())
	}
}

macro_rules! impl_partial_eq {
	($t:ty, $via_t:ty) => {
		impl PartialEq<$t> for Error {
			#[inline]
			fn eq(&self, other: &$t) -> bool {
				<$via_t>::from(self.0.get()) == <$via_t>::from(*other)
			}
		}

		impl PartialEq<Error> for $t {
			#[inline]
			fn eq(&self, other: &Error) -> bool {
				<$via_t>::from(other.0.get()) == <$via_t>::from(*self)
			}
		}
	};
}

macro_rules! impl_partial_eq_nonzero {
	($t:ty, $via_t:ty) => {
		impl PartialEq<$t> for Error {
			#[inline]
			fn eq(&self, other: &$t) -> bool {
				<$via_t>::from(self.0.get()) == <$via_t>::from(other.get())
			}
		}

		impl PartialEq<Error> for $t {
			#[inline]
			fn eq(&self, other: &Error) -> bool {
				<$via_t>::from(other.0.get()) == <$via_t>::from(self.get())
			}
		}
	};
}

impl_partial_eq!(i16, i32);
impl_partial_eq!(i32, i32);
impl_partial_eq!(i64, i64);
impl_partial_eq!(u16, i32);
impl_partial_eq!(u32, i64);

impl_partial_eq_nonzero!(num::NonZeroI16, i32);
impl_partial_eq_nonzero!(num::NonZeroI32, i32);
impl_partial_eq_nonzero!(num::NonZeroI64, i64);
impl_partial_eq_nonzero!(num::NonZeroU16, i32);
impl_partial_eq_nonzero!(num::NonZeroU32, i64);

macro_rules! errno_constants {
	( $( $(#[$meta:meta])* $name:ident = $value:literal , )+ ) => {
		$(
			$(#[$meta])*
			pub const $name: $crate::Error = unsafe {
				$crate::Error::new_unchecked($value)
			};
		)*

		#[inline]
		const fn err_name(err: $crate::Error) -> Option<&'static str> {
			match err.0.get() {
			$(
				$value => Some(stringify!($name)),
			)*
				_ => None,
			}
		}
	}
}

impl fmt::Debug for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match err_name(*self) {
			Some(name) => f.write_str(name),
			_ => f.debug_tuple("Error").field(&self.0.get()).finish(),
		}
	}
}

#[cfg(feature = "posix-traits")]
const fn from_posix(err: posix_errno::Error) -> Option<Error> {
	use posix_errno::Error as P;
	match err {
		P::E2BIG           => Some(E2BIG),
		P::EACCES          => Some(EACCES),
		P::EADDRINUSE      => Some(EADDRINUSE),
		P::EADDRNOTAVAIL   => Some(EADDRNOTAVAIL),
		P::EAFNOSUPPORT    => Some(EAFNOSUPPORT),
		P::EAGAIN          => Some(EAGAIN),
		P::EALREADY        => Some(EALREADY),
		P::EBADF           => Some(EBADF),
		P::EBADMSG         => Some(EBADMSG),
		P::EBUSY           => Some(EBUSY),
		P::ECANCELED       => Some(ECANCELED),
		P::ECHILD          => Some(ECHILD),
		P::ECONNABORTED    => Some(ECONNABORTED),
		P::ECONNREFUSED    => Some(ECONNREFUSED),
		P::ECONNRESET      => Some(ECONNRESET),
		P::EDEADLK         => Some(EDEADLK),
		P::EDESTADDRREQ    => Some(EDESTADDRREQ),
		P::EDOM            => Some(EDOM),
		P::EDQUOT          => Some(EDQUOT),
		P::EEXIST          => Some(EEXIST),
		P::EFAULT          => Some(EFAULT),
		P::EFBIG           => Some(EFBIG),
		P::EHOSTUNREACH    => Some(EHOSTUNREACH),
		P::EIDRM           => Some(EIDRM),
		P::EILSEQ          => Some(EILSEQ),
		P::EINPROGRESS     => Some(EINPROGRESS),
		P::EINTR           => Some(EINTR),
		P::EINVAL          => Some(EINVAL),
		P::EIO             => Some(EIO),
		P::EISCONN         => Some(EISCONN),
		P::EISDIR          => Some(EISDIR),
		P::ELOOP           => Some(ELOOP),
		P::EMFILE          => Some(EMFILE),
		P::EMLINK          => Some(EMLINK),
		P::EMSGSIZE        => Some(EMSGSIZE),
		P::EMULTIHOP       => Some(EMULTIHOP),
		P::ENAMETOOLONG    => Some(ENAMETOOLONG),
		P::ENETDOWN        => Some(ENETDOWN),
		P::ENETRESET       => Some(ENETRESET),
		P::ENETUNREACH     => Some(ENETUNREACH),
		P::ENFILE          => Some(ENFILE),
		P::ENOBUFS         => Some(ENOBUFS),
		P::ENODATA         => Some(ENODATA),
		P::ENODEV          => Some(ENODEV),
		P::ENOENT          => Some(ENOENT),
		P::ENOEXEC         => Some(ENOEXEC),
		P::ENOLCK          => Some(ENOLCK),
		P::ENOLINK         => Some(ENOLINK),
		P::ENOMEM          => Some(ENOMEM),
		P::ENOMSG          => Some(ENOMSG),
		P::ENOPROTOOPT     => Some(ENOPROTOOPT),
		P::ENOSPC          => Some(ENOSPC),
		P::ENOSR           => Some(ENOSR),
		P::ENOSTR          => Some(ENOSTR),
		P::ENOSYS          => Some(ENOSYS),
		P::ENOTCONN        => Some(ENOTCONN),
		P::ENOTDIR         => Some(ENOTDIR),
		P::ENOTEMPTY       => Some(ENOTEMPTY),
		P::ENOTRECOVERABLE => Some(ENOTRECOVERABLE),
		P::ENOTSOCK        => Some(ENOTSOCK),
		P::ENOTSUP         => Some(ENOTSUP),
		P::ENOTTY          => Some(ENOTTY),
		P::ENXIO           => Some(ENXIO),
		P::EOPNOTSUPP      => Some(EOPNOTSUPP),
		P::EOVERFLOW       => Some(EOVERFLOW),
		P::EOWNERDEAD      => Some(EOWNERDEAD),
		P::EPERM           => Some(EPERM),
		P::EPIPE           => Some(EPIPE),
		P::EPROTO          => Some(EPROTO),
		P::EPROTONOSUPPORT => Some(EPROTONOSUPPORT),
		P::EPROTOTYPE      => Some(EPROTOTYPE),
		P::ERANGE          => Some(ERANGE),
		P::EROFS           => Some(EROFS),
		P::ESPIPE          => Some(ESPIPE),
		P::ESRCH           => Some(ESRCH),
		P::ESTALE          => Some(ESTALE),
		P::ETIME           => Some(ETIME),
		P::ETIMEDOUT       => Some(ETIMEDOUT),
		P::ETXTBSY         => Some(ETXTBSY),
		P::EWOULDBLOCK     => Some(EWOULDBLOCK),
		P::EXDEV           => Some(EXDEV),
		_ => None,
	}
}

#[cfg(any(feature = "posix-traits", doc))]
impl PartialEq<posix_errno::Error> for Error {
	#[inline]
	fn eq(&self, other: &posix_errno::Error) -> bool {
		from_posix(*other) == Some(*self)
	}
}

#[cfg(any(feature = "posix-traits", doc))]
impl PartialEq<Error> for posix_errno::Error {
	#[inline]
	fn eq(&self, other: &Error) -> bool {
		from_posix(*self) == Some(*other)
	}
}

errno_constants! {
	// https://github.com/apple-oss-distributions/xnu/blob/xnu-11215.1.10/bsd/sys/errno.h

	/// Operation not permitted
	EPERM = 1,
	/// No such file or directory
	ENOENT = 2,
	/// No such process
	ESRCH = 3,
	/// Interrupted system call
	EINTR = 4,
	/// Input/output error
	EIO = 5,
	/// Device not configured
	ENXIO = 6,
	/// Argument list too long
	E2BIG = 7,
	/// Exec format error
	ENOEXEC = 8,
	/// Bad file descriptor
	EBADF = 9,
	/// No child processes
	ECHILD = 10,
	/// Resource deadlock avoided
	EDEADLK = 11,
	/// Cannot allocate memory
	ENOMEM = 12,
	/// Permission denied
	EACCES = 13,
	/// Bad address
	EFAULT = 14,
	/// Block device required
	ENOTBLK = 15,
	/// Device / Resource busy
	EBUSY = 16,
	/// File exists
	EEXIST = 17,
	/// Cross-device link
	EXDEV = 18,
	/// Operation not supported by device
	ENODEV = 19,
	/// Not a directory
	ENOTDIR = 20,
	/// Is a directory
	EISDIR = 21,
	/// Invalid argument
	EINVAL = 22,
	/// Too many open files in system
	ENFILE = 23,
	/// Too many open files
	EMFILE = 24,
	/// Inappropriate ioctl for device
	ENOTTY = 25,
	/// Text file busy
	ETXTBSY = 26,
	/// File too large
	EFBIG = 27,
	/// No space left on device
	ENOSPC = 28,
	/// Illegal seek
	ESPIPE = 29,
	/// Read-only file system
	EROFS = 30,
	/// Too many links
	EMLINK = 31,
	/// Broken pipe
	EPIPE = 32,

	/// Numerical argument out of domain
	EDOM = 33,
	/// Result too large
	ERANGE = 34,

	/// Resource temporarily unavailable
	EAGAIN = 35,
	/// Operation now in progress
	EINPROGRESS = 36,
	/// Operation already in progress
	EALREADY = 37,

	/// Socket operation on non-socket
	ENOTSOCK = 38,
	/// Destination address required
	EDESTADDRREQ = 39,
	/// Message too long
	EMSGSIZE = 40,
	/// Protocol wrong type for socket
	EPROTOTYPE = 41,
	/// Protocol not available
	ENOPROTOOPT = 42,
	/// Protocol not supported
	EPROTONOSUPPORT = 43,
	/// Socket type not supported
	ESOCKTNOSUPPORT = 44,
	/// Operation not supported
	ENOTSUP = 45,

	/// Protocol family not supported
	EPFNOSUPPORT = 46,
	/// Address family not supported by protocol family
	EAFNOSUPPORT = 47,
	/// Address already in use
	EADDRINUSE = 48,
	/// Can't assign requested address
	EADDRNOTAVAIL = 49,

	/// Network is down
	ENETDOWN = 50,
	/// Network is unreachable
	ENETUNREACH = 51,
	/// Network dropped connection on reset
	ENETRESET = 52,
	/// Software caused connection abort
	ECONNABORTED = 53,
	/// Connection reset by peer
	ECONNRESET = 54,
	/// No buffer space available
	ENOBUFS = 55,
	/// Socket is already connected
	EISCONN = 56,
	/// Socket is not connected
	ENOTCONN = 57,
	/// Can't send after socket shutdown
	ESHUTDOWN = 58,
	/// Too many references: can't splice
	ETOOMANYREFS = 59,
	/// Operation timed out
	ETIMEDOUT = 60,
	/// Connection refused
	ECONNREFUSED = 61,
	/// Too many levels of symbolic links
	ELOOP = 62,
	/// File name too long
	ENAMETOOLONG = 63,

	/// Host is down
	EHOSTDOWN = 64,
	/// No route to host
	EHOSTUNREACH = 65,
	/// Directory not empty
	ENOTEMPTY = 66,

	/// Too many processes
	EPROCLIM = 67,
	/// Too many users
	EUSERS = 68,
	/// Disc quota exceeded
	EDQUOT = 69,

	/// Stale NFS file handle
	ESTALE = 70,
	/// Too many levels of remote in path
	EREMOTE = 71,
	/// RPC struct is bad
	EBADRPC = 72,
	/// RPC version wrong
	ERPCMISMATCH = 73,
	/// RPC prog. not avail
	EPROGUNAVAIL = 74,
	/// Program version wrong
	EPROGMISMATCH = 75,
	/// Bad procedure for program
	EPROCUNAVAIL = 76,

	/// No locks available
	ENOLCK = 77,
	/// Function not implemented
	ENOSYS = 78,

	/// Inappropriate file type or format
	EFTYPE = 79,
	/// Authentication error
	EAUTH = 80,
	/// Need authenticator
	ENEEDAUTH = 81,

	/// Device power is off
	EPWROFF = 82,
	/// Device error, e.g. paper out
	EDEVERR = 83,

	/// Value too large to be stored in data type
	EOVERFLOW = 84,

	/// Bad executable
	EBADEXEC = 85,
	/// Bad CPU type in executable
	EBADARCH = 86,
	/// Shared library version mismatch
	ESHLIBVERS = 87,
	/// Malformed Macho file
	EBADMACHO = 88,

	/// Operation canceled
	ECANCELED = 89,

	/// Identifier removed
	EIDRM = 90,
	/// No message of desired type
	ENOMSG = 91,
	/// Illegal byte sequence
	EILSEQ = 92,
	/// Attribute not found
	ENOATTR = 93,

	/// Bad message
	EBADMSG = 94,
	/// Reserved
	EMULTIHOP = 95,
	/// No message available on STREAM
	ENODATA = 96,
	/// Reserved
	ENOLINK = 97,
	/// No STREAM resources
	ENOSR = 98,
	/// Not a STREAM
	ENOSTR = 99,
	/// Protocol error
	EPROTO = 100,
	/// STREAM ioctl timeout
	ETIME = 101,

	/// Operation not supported on socket
	EOPNOTSUPP = 102,
	/// No such policy registered
	ENOPOLICY = 103,

	/// State not recoverable
	ENOTRECOVERABLE = 104,
	/// Previous owner died
	EOWNERDEAD = 105,

	/// Interface output queue is full
	EQFULL = 106,
}

/// Operation would block (alias for [`EAGAIN`])
pub const EWOULDBLOCK: Error = EAGAIN;
