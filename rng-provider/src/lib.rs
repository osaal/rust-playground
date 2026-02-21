use std::io::{Error, ErrorKind};

/// Synchronous RNG provider
///
/// Objects implementing this provider are expected to not block the main thread.
pub trait RNGProvider {
    /// The raw byte return type for the implementing provider
    type RNGRawByteArray;

    /// Attempt to read bytes from the RNG provider
    ///
    /// The return propagates any IO errors or construct its own.
    fn try_get_bytes(buflen: usize) -> Result<Self::RNGRawByteArray, std::io::Error>;
}

/// Safely retrieve random numbers from the `/dev/random` device on Unix-like systems
///
/// This interface uses the `getrandom(2)` syscall, resulting in a safer alternative
/// than directly reading the file.
///
/// However, it is **not async safe**:
///
/// > If the `getrandom(2)` syscall would have blocked due to issues with the byte or entropy pool,
/// the interface will return an error. See the `DESCRIPTION` section of `man 2 getrandom` for
/// more information.
pub struct UnixDevRandom {}

impl RNGProvider for UnixDevRandom {
    type RNGRawByteArray = Vec<u8>;

    fn try_get_bytes(buflen: usize) -> Result<Self::RNGRawByteArray, std::io::Error> {
        match buflen {
            val if val >= isize::MAX as usize => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "`buflen` must be less than `isize::MAX`",
                ));
            }
            val if val == 0 => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "`buflen` must be non-zero",
                ));
            }
            _ => (),
        }

        let mut buf = vec![0u8; buflen];

        // GRND_NONBLOCK flag ensures non-blocking behavior
        const GRND_NONBLOCK: u32 = 0x0001;

        // SAFETY:
        // - GRND_NONBLOCK ensures getrandom() returns synchronously without blocking
        // - buf.as_mut_ptr() is valid for buf.len() bytes because buf is an allocated Vec<u8>
        let ret = unsafe {
            unsafe extern "C" {
                unsafe fn getrandom(buf: *mut u8, buflen: usize, flags: u32) -> isize;
            }
            getrandom(buf.as_mut_ptr(), buf.len(), GRND_NONBLOCK)
        };

        // getrandom returns -1 on error
        if ret < 0 {
            return Err(std::io::Error::last_os_error());
        }

        // The return code should match the length of the buffer
        if ret as usize != buf.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "getrandom() returned fewer bytes than requested",
            ));
        }

        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn unix_dev_random_safe() {
        let res = UnixDevRandom::try_get_bytes(16);
        assert!(res.is_ok());
    }

    #[test]
    fn overflow_udrs() {
        let res = UnixDevRandom::try_get_bytes((isize::MAX as usize) + 1);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::InvalidInput);
    }

    #[test]
    fn zero_sized_udrs() {
        let res = UnixDevRandom::try_get_bytes(0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::InvalidInput);
    }
}
