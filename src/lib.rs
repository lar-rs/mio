pub mod analog;
pub mod digital;
pub mod canmsg;
pub mod furnace;
pub mod sensor;
pub mod pump;
pub mod lamp;
pub mod relay;
pub mod valve;
pub mod xysys;
pub mod tmpfs;



pub use self::analog::{Input,Output};
pub use self::canmsg::{CanMsg};
pub use self::furnace::{Furnace};
pub use self::pump::Pump;
pub use self::lamp::Lamp;
pub use self::relay::Relay;
pub use self::valve::Valve;
pub use self::xysys::Axis;
use async_std::io;
use async_std::fs;
use async_std::path::PathBuf;
use std::env;
// 

pub async fn miofs() -> io::Result<PathBuf> {
    let path = PathBuf::from(env::var("HOME").unwrap_or("./".to_owned())).join(".pwa/mio");
    if !path.is_dir().await {
        fs::create_dir_all(&path).await?;
        let mio = path.join("mio");
        fs::create_dir_all(&mio).await?;
        tmpfs::mount(&mio).await?;
    }
    Ok(path)
}

// pub mod mosk {
    // pub use self::analog::mosk::{AnalogIn,AnalogOut};
    // pub use self::furnace::mosk::{MoskFurnace};
    // pub use self::pump::mosk::MoskPump;
    // pub use self::lamp::mosk::MoskLamp;
    // pub use self::relay::mosk::MoskRelay;
    // pub use self::sensor::mosk::MoskSensor;
    // pub use self::valve::mosk::MoskValve;
    // pub use self::xysys::mosk::MoskAxis;
// }

