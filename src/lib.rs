pub mod error;
pub mod driver;
pub mod analog;
pub mod airflow;
pub mod digital;
pub mod furnace;
pub mod sensor;
pub mod pump;
pub mod lamp;
pub mod fluid;
pub mod relay;
pub mod valve;
pub mod axis;
pub mod tmpfs;
pub mod uv;

pub use self::error::MioError;
pub use self::driver::{Driver,HID};
pub use self::analog::ADC;
pub use self::airflow::Airflow;
pub use self::furnace::Furnace;
pub use self::pump::{GearPump,ImpulsePump};
pub use self::lamp::Lamp;
pub use self::fluid::Fluid;
pub use self::relay::Relay;
pub use self::valve::Valve;
pub use self::sensor::Sensor;
pub use self::axis::Axis;
pub use self::uv::Uv;

use async_std::io;
use async_std::fs;
use async_std::path::PathBuf;
// use std::env;
// 

pub async fn miofs() -> io::Result<PathBuf> {
    let path = PathBuf::from("/pwa/mio");
    if !path.is_dir().await {
        fs::create_dir_all(&path).await?;
        tmpfs::mount(&path).await?;
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

