// use std::io;


/// Errors that may occur during mocking.
#[derive(PartialEq, Clone, Debug)]
pub enum Error {
    /// An Lamp Error occurred
    UvLamp,
    Furnace,


}

// impl From<io::Error> for Error {
    // fn from(e: io::Error) -> Self {
        // MockError::Io(e.kind())
    // }
// }
//
