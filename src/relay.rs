/// Relay
// use async_std::prelude::*;
use async_std::io::Result;
use async_trait::async_trait;

/// Single digital push-pull output pin
#[async_trait]
pub trait Relay {

    /// Relay check is open
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
    /// electrical sources
    async fn is_open(&self) -> Result<bool>;
 
    /// Relay open
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
    /// electrical sources
    async fn open(&mut self) -> Result<()>;

    /// Drives the off
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
    /// electrical sources
    async fn close(&mut self) -> Result<()>;
    // Toggle reley
    //
    //
    // electrical sources
    // async fn toggle(&mut self) -> Result<()> {
    //     if self.is_open().await? {
    //         self.close().await
    //     }else {
    //         self.open().await
    //     }
    // }
}




// #[cfg(feature = "mosk")]
pub mod mosk {
    use async_std::io::Result;
    use async_trait::async_trait;
    use async_std::prelude::*;
    use async_std::stream;
    use std::time::{Duration};

    /// Lamp simulation
    pub struct Relay {
        open: bool,
    }
    #[async_trait]
    impl super::Relay for Relay {
        async fn is_open(&self) -> Result<bool> {
            Ok(self.open)
        }
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
