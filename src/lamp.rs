/// Lamp UV
use async_std::prelude::*;
use async_std::io;
use async_std::path::{Path,PathBuf};
// use async_trait::async_trait;
// use std::time::{Duration,Instant};
use super::digital::{Digital};

// /// Single digital push-pull output pin
// #[async_trait]
// pub trait Lamp {
//     /// Error type
//     ///
//     /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
//     /// electrical sources
//     async fn set_on(&mut self) -> Result<()>;

//     /// Drives the off
//     ///
//     /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
//     /// electrical sources
//     async fn set_off(&mut self) -> Result<()>;
// }


 /// Lamp simulation
 pub struct Lamp {
    path: PathBuf,
    digital:Digital,
}

impl Lamp {
    pub async fn select(path: &Path) -> io::Result<Lamp> {
        Ok({
            Lamp{
                digital:Digital::select(&path.join("driver")).await?,
                path: path.to_path_buf(),
            }
        })
    }
    pub async fn set_on(&mut self)  -> io::Result<()> {
        self.digital.set_high().await?;
        // if !self.on{
        //     let mut interval  = stream::interval(Duration::from_secs(1));
        //     interval.next().await;
        //     self.on = true;
        //     self.ontime = Instant::now();
        // }
        Ok(())
    }
    pub async fn set_off(&mut self) -> io::Result<()> {
        self.digital.set_low().await?;
        // if self.on {
        //     let mut interval  = stream::interval(Duration::from_secs(2));
        //     interval.next().await;
        //     self.on = false;
        //     self.runtime = self.ontime.elapsed().as_secs();
        // }
        Ok(())
    }
}