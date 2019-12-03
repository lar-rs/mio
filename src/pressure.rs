use std::fmt;
// pub use uom::si::f32::{Ratio, Presure};
use async_std::prelude::*;
use async_std::stream;


/// Hardware presure sensor.
pub struct Pressure {
    path: PathBuf,
    pub unit: String,
    pub label: Option<String>,
    pub current:u32 ,
    pub brocken: bool,
    pub critical:bool,
}

impl Presure{
    pub async fn select(path:&Path) -> io::Result<Pressure> {

    }
    /// Returns sensor unit name.
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// Returns sensor label.
    pub fn label(&self) -> Option<&str> {
        self.label.as_ref().map(|s| s.as_str())
    }

    /// Returns current presure reported by sensor.
    pub fn current(&self) -> Presure {
        self.current
    }

    /// Returns high trip point for sensor if available.
    pub fn critical(&self) -> bool {
        self.critical
    }

    /// Returns critical trip point for sensor if available.
    pub fn brocken(&self) -> bool {
        self.critical
    }
}


// pub async fn check(sensor:Sensor) -> io::Result<bool> {

// }

pub async fn pressure(path: &str) -> io::Result<Presure> {
    let sensor = sensor(path.as_path()).await?;
    Ok(sensor)
}

