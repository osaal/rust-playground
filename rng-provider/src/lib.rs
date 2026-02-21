//! Create random-number generator interfaces
//!
//! This library provides the [`RNGProvider`] trait for simple random number generators.
//! It also provides an example implementation, [`UnixDevRandom`], for interfacing with
//! [`/dev/urandom` and `/dev/random`](https://en.wikipedia.org/wiki//dev/random) on Unix-like systems.
//!
//! # Ready-made implementation
//!
//! If you are looking for a simple way to generate pseudo-random numbers on Unix,
//! simply use [`UnixDevRandom::try_get_bytes()`]:
//!
//! ```rust no_run
//! let rng = UnixDevRandom::try_get_bytes(16);
//! assert_eq!(rng.len(), 16);
//! ```
//!
//! # Custom implementations
//!
//! The [`RNGProvider`] trait exposes a fallible method for getting random raw bytes, `try_get_bytes`.
//! It also offers an asynchronous version if compiled with the `async` feature gate, called `try_get_bytes_async`.
//!
//! In both cases, the methods take a buffer length representing how much random data to read from the provider.
//! Note, that it is up to the implementation site to guarantee that a read of length `usize` is allowed and successful.
//!
//! The methods allow the implementer to define their own type of successful return data, called `RNGRawByteArray`.
//! This can be as simple as a byte array:
//!
//! ```rust no_run
//! struct MyImplementor {}
//!
//! impl RNGProvider for MyImplementor {
//!     type RNGRawByteArray = Vec<u8>;
//!     fn try_get_bytes(buflen: usize) -> Result<Self::RNGRawByteArray, std::io::Error> { }
//! }
//! ```
//!
//! Since interacting with an external RNG provider is an I/O operation, the methods are expected to return
//! [`std::io::Error`] types. In the future, this may be eased to a default type association instead.
//!
//! As an example, the `UnixDevRandom` impl returns an error with [`std::io::ErrorKind::InvalidInput`] if
//! the given `buflen` is inappropriate.

#[cfg(feature = "async")]
use std::future::Future;

use std::io::{Error, ErrorKind};

/// Trait for interacting with a random number generator provider
///
/// The default trait method `try_get_bytes` is synchronous, and is expected
/// to not block in any way.
///
/// For asynchronous interactions, implement `try_get_bytes_async` from the `async` feature.
pub trait RNGProvider {
    /// The raw byte return type for the implementing provider
    type RNGRawByteArray;

    /// Attempt to read bytes from the RNG provider
    ///
    /// The return propagates any IO errors or construct its own.
    fn try_get_bytes(buflen: usize) -> Result<Self::RNGRawByteArray, std::io::Error>;

    /// Asynchronously attempt to read bytes from the RNG provider
    ///
    /// The contained error type should represent a real failure to read from the provider,
    /// not a failure stemming from blocking.
    ///
    /// The function should await until its body has resolved completely before returning.
    #[cfg(feature = "async")]
    fn try_get_bytes_async(
        buflen: usize,
    ) -> impl Future<Output = Result<Self::RNGRawByteArray, std::io::Error>>;
}

/// Safely retrieve random numbers from the [`/dev/random`](https://en.wikipedia.org/wiki//dev/random) device on Unix-like systems
///
/// This interface uses the [`getrandom(2)`](https://man7.org/linux/man-pages/man2/getrandom.2.html) syscall, resulting in a safer alternative
/// than directly reading the file.
///
/// If compiled with the `async` feature, it also implements the asynchronous version
/// of the method, `try_get_bytes_async`.
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

    #[cfg(feature = "async")]
    fn try_get_bytes_async(
        buflen: usize,
    ) -> impl Future<Output = Result<Self::RNGRawByteArray, std::io::Error>> {
        async move {
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

            // No flags: allow getrandom() to block until sufficient entropy is available
            const GRND_NONE: u32 = 0;

            // SAFETY:
            // - buf.as_mut_ptr() is valid for buf.len() bytes because buf is an allocated Vec<u8>
            // - calling getrandom with flags == 0 is allowed and may block; caller awaits this async fn
            let ret = unsafe {
                unsafe extern "C" {
                    fn getrandom(buf: *mut u8, buflen: usize, flags: u32) -> isize;
                }
                getrandom(buf.as_mut_ptr(), buf.len(), GRND_NONE)
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
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn basic_udr() {
        let res = UnixDevRandom::try_get_bytes(16);
        assert!(res.is_ok());
    }

    #[test]
    fn large_udr() {
        let res = UnixDevRandom::try_get_bytes(1_000_000_000);
        assert!(res.is_ok());
    }

    #[test]
    fn overflow_udr() {
        let res = UnixDevRandom::try_get_bytes((isize::MAX as usize) + 1);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::InvalidInput);
    }

    #[test]
    fn zero_sized_udr() {
        let res = UnixDevRandom::try_get_bytes(0);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::InvalidInput);
    }

    #[tokio::test]
    async fn basic_udr_async() {
        let res = UnixDevRandom::try_get_bytes_async(16).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn large_udr_async() {
        let res = UnixDevRandom::try_get_bytes_async(1_000_000_000).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn overflow_udr_async() {
        let res = UnixDevRandom::try_get_bytes_async((isize::MAX as usize) + 1).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::InvalidInput);
    }

    #[tokio::test]
    async fn zero_sized_udr_async() {
        let res = UnixDevRandom::try_get_bytes_async(0).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), ErrorKind::InvalidInput);
    }
}
