/// Monitor gear pump normally used for solution sampling.
///
///

// use async_std::prelude::*;
use async_std::io::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Furnace{
    async fn is_open(&self)   -> Result<bool>;
    async fn open(&mut self)  -> Result<()>;
    async fn close(&mut self) -> Result<()>;
    async fn ready(&mut self) -> Result<bool>;
}



// #[cfg(feature = "mosk")]
pub mod mosk {
    use async_std::prelude::*;
    use async_std::stream;
    use std::time::Duration;
    use async_trait::async_trait;
    use async_std::io::Result;

    pub struct Furnace {
        open: bool 


    }

    #[async_trait]
    impl super::Furnace for Furnace {
        async fn is_open(&self)   -> Result<bool> {
            Ok(self.open)
        }

        async fn open(&mut self)  -> Result<()> {
            if !self.open {
                let mut interval  = stream::interval(Duration::from_secs(2));
                interval.next().await;
                self.open = true;
            }
            Ok(())
        }
        async fn close(&mut self) -> Result<()> {
            if self.open {
                let mut interval  = stream::interval(Duration::from_secs(2));
                interval.next().await;
                self.open = false;
            }
            Ok(())
        }
        async fn ready(&mut self) -> Result<bool> {
           Ok(true)
        }
    }
}
