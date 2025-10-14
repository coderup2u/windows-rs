use super::*;

/// An error or status code value returned by some operating system functions.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[must_use]
pub struct WIN32_ERROR(pub u32);

impl WIN32_ERROR {
    /// Returns [`true`] if `self` is a success code.
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0 == 0
    }

    /// Returns [`true`] if `self` is a failure code.
    #[inline]
    pub const fn is_err(self) -> bool {
        !self.is_ok()
    }

    /// Maps a Win32 error code to an HRESULT value.
    #[inline]
    pub const fn to_hresult(self) -> HRESULT {
        HRESULT(if self.0 as i32 <= 0 {
            self.0
        } else {
            (self.0 & 0x0000_FFFF) | (7 << 16) | 0x8000_0000
        } as i32)
    }

    #[inline]
    pub fn from_error(error: &Error) -> Option<Self> {
        let hresult = error.code().0 as u32;

        if ((hresult >> 16) & 0x7FF) == 7 {
            Some(Self(hresult & 0xFFFF))
        } else {
            None
        }
    }

    #[inline]
    pub fn ok(self) -> Result<()> {
        self.to_hresult().ok()
    }

    /// Creates a new `WIN32_ERROR` from the Win32 error code returned by `GetLastError()`.
    pub fn from_thread() -> Self {
        Self(unsafe { GetLastError() })
    }
}

impl From<WIN32_ERROR> for HRESULT {
    fn from(value: WIN32_ERROR) -> Self {
        value.to_hresult()
    }
}

impl From<WIN32_ERROR> for Error {
    fn from(value: WIN32_ERROR) -> Self {
        value.to_hresult().into()
    }
}

impl core::fmt::Display for WIN32_ERROR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl core::fmt::Debug for WIN32_ERROR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_fmt(format_args!("WIN32_ERROR({self})"))
    }
}
