pub mod error;
pub mod iface;
pub mod channel;
pub mod method;
pub mod statistic;
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
pub mod stirrer;
pub mod uv;
pub mod ws;
pub mod cli;
pub mod state;

pub use self::error::MioError;
pub use self::ws::{Workspace};
pub use self::method::Method;
pub use self::channel::Channel;
pub use self::statistic::Statistic;
pub use self::driver::Driver;
pub use self::digital::{DigIN,DigOUT};
pub use self::iface::{Interface,IType,IClass};
pub use self::analog::Analog;
pub use self::airflow::Airflow;
pub use self::stirrer::Stirrer;
pub use self::furnace::Furnace;
pub use self::pump::{GearPump,ImpulsePump};
pub use self::lamp::Lamp;
pub use self::fluid::Fluid;
pub use self::relay::Relay;
pub use self::valve::Valve;
pub use self::sensor::Sensor;
pub use self::axis::Axis;
pub use self::uv::Uv;

// use std::env;
//

pub type Error = error::MioError;
pub type Result<T> = std::result::Result<T, Error>;

