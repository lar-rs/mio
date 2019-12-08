#![allow(unused_variables)]

use failure::{Fail};
use std::io;
use std::io::{Error, ErrorKind};
use std::num::ParseFloatError;
use std::num::ParseIntError;
// use std::string::FromUtf8Error;

// use jsonrpc_core::Error as RpcError;
// use mut_guard::*;

#[derive(Fail, Debug)]
pub enum MioError {
    #[fail(display = "io error - {}",err)]
    IOError {err: Error },
    #[fail(display = "async error - {}",err)]
    AsyncError {err: std::io::Error },
    #[fail(display = "mount miofs error - {}",msg)]
    Mount {msg: String},
    #[fail(display = "data error - {}",msg)]
    DataErr {msg: String},
    #[fail(display = "device timeout - {}",msg)]
    DriverTimeout {msg: String},
    #[fail(display = "convert int error - {}", err)]
    ConvertInt{ err: std::num::ParseIntError },
    #[fail(display = "convert float error - {}", err)]
    ConvertFloat{ err:ParseFloatError },
}



pub fn wrong_data(msg:String) -> MioError {
    MioError::DataErr{msg}
}

pub fn driver_timeout(msg:String) -> MioError {
    MioError::DataErr{msg}
}

impl From<Error> for MioError {
    fn from(kind:io::Error) -> MioError {
        MioError::IOError{err: kind}
    }
}
impl From<MioError> for Error {
    fn from(larerr:MioError) -> Error {
        Error::new(ErrorKind::Other, format!("can error - {}",larerr))
    }
}
impl From<ParseIntError> for MioError {
    fn from(err: ParseIntError) -> MioError {
        MioError::ConvertInt{err}
    }
}

impl From<ParseFloatError> for MioError {
    fn from(err: ParseFloatError) -> MioError {
        MioError::ConvertFloat{err}
    }
}