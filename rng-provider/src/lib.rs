use std::{
    fs::File,
    io::{Error, Read},
};

pub trait RNGProvider {
    /// The raw byte return type for the implementing provider
    type RNGRawByteArray;

    /// Attempt to read bytes from the PRNG provider
    ///
    /// The return should propagate any IO errors or construct its own.
    fn try_get_bytes() -> Result<Self::RNGRawByteArray, std::io::Error>;
}

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
