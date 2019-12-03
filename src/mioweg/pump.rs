/// Monitor gear pump normally used for solution sampling.
///
///
use async_std::prelude::*;
use async_std::io::Result;
use async_trait::async_trait;
use std::time::{Duration};
use async_std::stream;

#[async_trait]
pub trait Pump {
    async fn start(&mut self)  -> Result<()>;
    async fn stop(&mut self) -> Result<()>;
}

  /// Lamp simulation
pub struct Simulate {
    on: bool,
}


#[async_trait]
impl Pump for Simulate {
    async fn start(&mut self)  -> Result<()> {
        if !self.on{
            let mut interval  = stream::interval(Duration::from_millis(250));
            interval.next().await;
            self.on = true;
        }
        Ok(())
    }
    async fn stop(&mut self) -> Result<()> {
        if self.on {
            let mut interval  = stream::interval(Duration::from_millis(250));
            interval.next().await;
            self.on = false;
        }
        Ok(())
    }
}

