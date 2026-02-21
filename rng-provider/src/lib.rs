use std::{
    fs::File,
    io::{Error, Read},
};

/// Synchronous RNG provider
///
/// Objects implementing this provider are expected to not block the main thread.
pub trait RNGProvider {
    /// The raw byte return type for the implementing provider
    type RNGRawByteArray;

    /// Attempt to read bytes from the RNG provider
    ///
    /// The return propagates any IO errors or construct its own.
    fn try_get_bytes() -> Result<Self::RNGRawByteArray, std::io::Error>;
}

/// Retrieve random numbers from the `/dev/random` device on Unix-like systems
pub struct UnixDevRandom {}

impl RNGProvider for UnixDevRandom {
    type RNGRawByteArray = Vec<u8>;
    fn try_get_bytes() -> Result<Vec<u8>, Error> {
        // TODO: This should probably be async since /dev/random blocks until it can return...
        let mut handle = File::open("/dev/random")?;
        let mut buf = vec![0; 16];
        match handle.read_exact(&mut buf) {
            Ok(_) => Ok(buf),
            Err(e) => Err(e),
        }
    }
}

/// Safely retrieve random numbers from the `/dev/random` device on Unix-like systems
///
/// This interface uses the `getrandom(2)` syscall, resulting in a safer alternative
/// than directly reading the file.
pub struct UnixDevRandomSafe {}

impl RNGProvider for UnixDevRandomSafe {
    type RNGRawByteArray = Vec<u8>;

    fn try_get_bytes() -> Result<Self::RNGRawByteArray, std::io::Error> {
        let mut buf = vec![0u8; 16];

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
    fn returns_something() {
        let res = UnixDevRandom::try_get_bytes();
        assert!(res.is_ok());
        println!("{:?}", res.unwrap())
    }
}
