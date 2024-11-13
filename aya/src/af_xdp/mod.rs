//! WIP

pub mod xsk;
pub use xsk::{
    BufIdx, DeviceQueue, IfInfo, ReadComplete, ReadRx, RingCons, RingProd, RingRx, RingTx, Socket,
    SocketConfig, Umem, UmemChunk, UmemConfig, User, WriteFill, WriteTx,
};
pub(crate) struct LastErrno;

/// An error that has been read from `errno`.
//
// `Default` is a bit misleading even though there is a constructor without any parameters. In
// hindsight it may have been better to provide a descriptive name.
#[allow(clippy::new_without_default)]
pub struct Errno(libc::c_int);

impl From<LastErrno> for Errno {
    fn from(LastErrno: LastErrno) -> Self {
        Self::last_os_error()
    }
}

impl Errno {
    /// Create an error from the latest `errno`.
    #[deprecated = "use the more descriptive name `Errno::last_os_error`"]
    #[doc(hidden)]
    pub fn new() -> Self {
        Self::last_os_error()
    }

    /// Create an error from the latest `errno`.
    pub fn last_os_error() -> Self {
        Self(unsafe { *libc::__errno_location() })
    }

    /// Get the actual `errno` value.
    pub fn get_raw(&self) -> libc::c_int {
        self.0
    }
}

impl core::fmt::Display for Errno {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let st = unsafe { libc::strerror(self.0) };
        let cstr = unsafe { core::ffi::CStr::from_ptr(st) };
        write!(f, "{}", cstr.to_string_lossy())
    }
}

impl core::fmt::Debug for Errno {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Errno({}: {})", self.0, self)
    }
}
