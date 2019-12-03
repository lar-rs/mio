/// Sensor HAL
/// NDir1,NDir2 Sauerstoff
///
///

// use async_std::prelude::*;
use async_std::io::Result;
use async_trait::async_trait;
// use async_std::stream;
// use std::time::{Duration,Instant};



use serde::{Deserialize, Serialize};
// use core::ops::Range;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Record {
    pub value: f32,
    pub state: u8,
    pub time:  u64,
}


impl Record {
    pub fn new(value: f32,state:u8)-> Record {
        Record {
            value: value,
            state: state,
            time: 0,
        }
    }
}

#[async_trait]
pub trait Sensor {
    async fn signal(&mut self) -> Result<f32>;
    async fn fsr   (&mut self) -> Result<f32>;
    async fn scale (&self) -> Result<f32>;
    async fn interval (&self) -> Result<u64>;
    async fn set_scale (&mut self,scale:f32) -> Result<()>;
    async fn set_interval(&mut self, millis: u64) -> Result<()>;
    // async fn average(&mut self,count: usize) -> Result<Vec<Record>> {
        // let mut interval  = stream::interval(Duration::from_millis(self.interval().await?));
        // let mut average:Vec<Record> =  Vec::new();
        // while let Some(_) = interval.next().await {
            // count -=1;
            // average.push(Record::new(self.signal().await?,0));
            // if count <= 0 {
                // break;
            // }
        // }
        // Ok(average)
    // }
}




// #[cfg(feature = "mosk")]
pub mod mosk {
    use async_trait::async_trait;
    use super::{Record,Sensor};
    use async_std::io::Result;

    #[derive(Clone, Debug, PartialEq)]
    pub struct MoskSensor{
        data: Vec<Record>,
        value: f32,
        scale: f32,
        interval: u64,
    }

    #[async_trait]
    impl Sensor for MoskSensor {
        async fn signal(&mut self)->Result<f32> {
            Ok(self.value)
        }
        async fn fsr   (&mut self) -> Result<f32> {
            Ok(self.value*self.scale)
        }
        async fn set_scale(&mut self, scale: f32)->Result<()> {
            self.scale = scale;
            Ok(())
        }
        async fn scale (&self) -> Result<f32> {
            Ok(self.scale)
        }
        async fn interval (&self) -> Result<u64> {
            Ok(self.interval)
        }
        async fn set_interval(&mut self, interval: u64) -> Result<()> {
            self.interval = interval;
            Ok(())
        }
    }
    // impl Stream for Sensor {
    //     type Item = f32;
    // // poll_next() is the only required method
    //     fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    //         Poll::Ready(Some(self.signal()))
    //     }
    // }
}

