use std::{
    fs::File,
    io::{Error, Read},
};

pub trait RNGProvider {
    /// Attempt to read bytes from the PRNG provider
    ///
    /// The return should propagate any IO errors or construct its own.
    fn try_get_bytes(&self, buf: &mut [u8]) -> Result<(), std::io::Error>;
}

pub struct OdDevRandom {}

impl OdDevRandom {
    pub fn new() -> Self {
        OdDevRandom {}
    }
}

impl RNGProvider for OdDevRandom {
    fn try_get_bytes(&self, buf: &mut [u8]) -> Result<(), Error> {
        let mut handle = File::open("/dev/random")?;
        handle.read_exact(buf)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn returns_something() {
        let provider = OdDevRandom::new();
        let mut buf = [0u8; 16];
        let res = provider.try_get_bytes(&mut buf);
        assert!(res.is_ok())
    }
}
