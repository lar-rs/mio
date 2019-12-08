/// Monitor gear pump normally used for solution sampling.
///
///
// use std::prelude::*;
use std::io::Result;
use async_trait::async_trait;
// use bytes::{Buf, Bytes, IntoBuf};
use serde::{Serialize};


#[async_trait]
pub trait CanMsg {
    type Value:Serialize;
    fn read ( &mut self ,node:u32, index:u16, sub:u8 ) -> Result<Self::Value> ;
    fn write( &mut self ,node:u32, index:u16, sub:u8 ,value:Self::Value)->Result<Self::Value>;
}


