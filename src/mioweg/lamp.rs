/// Lamp UV
// use async_std::prelude::*;
use async_std::io::Result;
use async_trait::async_trait;

/// Single digital push-pull output pin
#[async_trait]
pub trait Lamp {
    /// Error type
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
    /// electrical sources
    async fn set_on(&mut self) -> Result<()>;

    /// Drives the off
    ///
    /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
    /// electrical sources
    async fn set_off(&mut self) -> Result<()>;
}





// #[cfg(feature = "mosk")]
pub mod mosk {
    use async_std::prelude::*;
    use async_trait::async_trait;
    use async_std::stream;
    use std::time::{Duration,Instant};
    use async_std::io::Result;

    /// Lamp simulation
    pub struct Lamp {
        on: bool,
        runtime: u64,
        ontime: Instant,
    }

    #[async_trait]
    impl super::Lamp for Lamp {
        async fn set_on(&mut self)  -> Result<()> {
            if !self.on{
                let mut interval  = stream::interval(Duration::from_secs(1));
                interval.next().await;
                self.on = true;
                self.ontime = Instant::now();
            }
            Ok(())
        }
        async fn set_off(&mut self) -> Result<()> {
            if self.on {
                let mut interval  = stream::interval(Duration::from_secs(2));
                interval.next().await;
                self.on = false;
                self.runtime = self.ontime.elapsed().as_secs();
            }
            Ok(())
        }
    }
}
