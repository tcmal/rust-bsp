pub mod quake3;

#[derive(Debug)]
pub enum Error<'a> {
    BadMagic {
        expected: &'static [u8],
        actual: &'a [u8]
    },
    BadSize {
        req: u32
    }
}

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;