
/// Autosampler
///
///
///
// use async_std::prelude::*;
use async_std::io::Result;
use async_trait::async_trait;
// use async_std::stream::Stream;
// use async_std::prelude::*;
// use async_std::stream;
// use std::time::Duration;

pub struct Position {
    pub sensor: bool,
    pub pos: usize,
}


/// Axis interface..
#[async_trait]
pub trait Axis {
    // type Velocity;
    async fn position     ( &self ) -> Result<Position>;
    async fn move_to      ( &mut self, pos: Position) -> Result<Position>;
    async fn to_sensor    ( &mut self) -> Result<Position>;
    async fn is_sensor    ( &self) -> Result<bool>;
    async fn set_range    ( &mut self, max:Position) -> Result<()>;
    async fn set_velocity ( &mut self, velocity:u32) -> Result<()>;
}




#[cfg(feature = "mosk")]
pub mod mosk {
    use crate::error::MockError;
    use crate::common::Generic;

    pub struct MoskPos {
        pub pos : u32, 
        pub max : u32,
    }
    impl Stream for MoskPos {
        type Item = u32;
    } 
    pub struct MoskAxis {
        pos      : u32, 
        max      : u32,
        velosity : u64,
    }

    #[async_trait]
    impl Axis for MoskAxis {
        type Position = u32;
        type Velosity = u64;
        async fn position(&self ) -> Result<u32> {
            Ok(self.pos)
        }
        async fn moveto(&mut self , pus: u32) -> Result<u32> {
            let mut pos = if pos > self.max {
                self.max
            }else {
                pos
            };
            let mut interval  = stream::interval(Duration::from_millis(self.velocity));
            while let Some(_) = interval.next().await {
                if pos < self.pos {
                    pos += 1;
                }else if pos > self.pos {
                    pos -= 1;
                }else {
                    break;
                }
            } 
            self.pos = pos;
            Ok(pos)
        }
        async fn tosensor(&mut self , pus: u32) -> Result<Self::Position> {
            let mut interval  = stream::interval(Duration::from_millis(self.velocity));
            while let Some(_) = interval.next().await {
                if 0 == self.pos {
                    break;
                }
                self.pos -= 1;
            } 
            Ok(self.pos)
        }
        async fn issensor ( &mut self ) -> Result<Self::Position> {
            Ok(self.pos == 0)
        }
        async fn set_range( &mut self , max: u32 ) -> Result<()> {
            self.max = max;
            Ok(())
        }
        async fn set_velocity( &mut self , velocity: u64 ) -> Result<()> {
            self.velocity = velocity;
        }
    }


// /// Analog Out mock
// ///
// /// Models an Analog read or write
// #[derive(Clone, Debug, PartialEq)]
// pub struct MoskAnalogOut{
//     value: f32;
// }


// impl Analog for MoskAnalogOut {
//     type Error = MockError;
//     type Value = f32;

//     fn set_value (&mut self, v : Value) ->Result<(), Self::Error> {
//         self.value = v;
//         Ok(())
//     }
}


