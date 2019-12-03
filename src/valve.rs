/// Valve I/O
// use async_std::prelude::*;
// use async_std::prelude::*;
// use async_std::stream;
use async_std::io;
use async_std::path::{PathBuf,Path};
// use std::time::{Duration};
use super::digital::Digital;

// /// Single digital push-pull output pin
// #[async_trait]
// pub trait Valve {
//     /// Valve open
//     ///
//     /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
//     /// electrical sources
//     async fn open(&mut self) -> Result<()>;

//     /// Valve close
//     ///
//     /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
//     /// electrical sources
//     async fn close(&mut self) -> Result<()>;
// }


 /// Lamp simulation
 pub struct Valve {
    digital: Digital,
    path: PathBuf,
}

impl Valve {
    pub async fn select(path: &Path) -> io::Result<Valve> {
        Ok({
            Valve{
                digital:Digital::select(&path.join("driver")).await?,
                path: path.to_path_buf(),
            }
        })
    }
    pub async fn open(&mut self)  -> io::Result<()> {
        self.digital.set_high().await?;
        // if !self.open{
        //     let mut interval  = stream::interval(Duration::from_millis(250));
        //     interval.next().await;
        //     self.open = true;
        // }
        Ok(())
    }
    pub async fn close(&mut self) -> io::Result<()> {
        self.digital.set_low().await?;
        // if self.open {
        //     let mut interval  = stream::interval(Duration::from_millis(250));
        //     interval.next().await;
        //     self.open = false;
        // }
        Ok(())
    }
}
