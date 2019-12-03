/// Monitor gear pump normally used for solution sampling.
///
///
// use async_std::prelude::*;
// use async_std::prelude::*;
use async_std::path::{PathBuf,Path};
// use async_std::stream;
use async_std::io;
use super::digital::Digital;
// #[async_trait]
// pub trait Pump {
//     async fn start(&mut self)  -> Result<()>;
//     async fn stop(&mut self) -> Result<()>;
// }

 /// Lamp simulation
 pub struct Pump {
    digital:Digital,
    path: PathBuf,
}

impl Pump {
    pub async fn select(path: &Path) -> io::Result<Pump> {
        Ok({
            Pump{
                digital:Digital::select(&path.join("driver")).await?,
                path: path.to_path_buf(),
            }
        })
    }
    pub async fn start(&mut self)  -> io::Result<()> {
        self.digital.set_high().await?;
        // if !self.on{
        //     let mut interval  = stream::interval(Duration::from_millis(250));
        //     interval.next().await;
        //     self.on = true;
        // }
        Ok(())
    }
    pub async fn stop(&mut self) -> io::Result<()> {
        self.digital.set_low().await?;

        // if self.on {
        //     let mut interval  = stream::interval(Duration::from_millis(250));
        //     interval.next().await;
        //     self.on = false;
        // }
        Ok(())
    }
}