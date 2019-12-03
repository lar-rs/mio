/// Valve I/O
// use async_std::prelude::*;
use async_std::io::Result;
use async_trait::async_trait;

/// Single digital push-pull output pin
#[async_trait]
pub trait Valve {
    /// Valve open
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
    /// electrical sources
    async fn open(&mut self) -> Result<()>;

    /// Valve close 
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
    /// electrical sources
    async fn close(&mut self) -> Result<()>;
}





// #[cfg(feature = "mosk")]
pub mod mosk {
    use async_std::prelude::*;
    use async_std::stream;
    use async_std::io::Result;
    use std::time::{Duration};
    use async_trait::async_trait;

    /// Lamp simulation
    pub struct Valve {
        open: bool,
    }

    #[async_trait]
    impl super::Valve for Valve {
        async fn open(&mut self)  -> Result<()> {
            if !self.open{
                let mut interval  = stream::interval(Duration::from_millis(250));
                interval.next().await;
                self.open = true;
            }
            Ok(())
        }
        async fn close(&mut self) -> Result<()> {
            if self.open {
                let mut interval  = stream::interval(Duration::from_millis(250));
                interval.next().await;
                self.open = false;
            }
            Ok(())
        }
    }
}
