// use std::fmt;
// pub use uom::si::f32::{Ratio,MassRate,ElectricCurrent};
use super::setting;
use super::signal;
use async_std::prelude::*;
use async_std::fs;
use async_std::path::{ Path, PathBuf};
use async_std::stream;

use serde_json::from_str;
use serde::{Deserialize, Serialize};



/// Integration
///
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Integration {
    pub justification: u32,
    pub start_treshold: f64,
    pub stop_treshold: f64,
    pub min_start: u32,
    pub min_stop: u32,
    pub max_stop: u32,
}


impl Default for Integration {
    fn default() -> Self {
        Self {
            justification: 15,
            start_treshold: 0.002,
            stop_treshold: 0.003,
            min_start: 10,
            min_stop: 60,
            max_stop: 210,
        }
    }
}



/// Hardware airflow sensor.
pub struct NdirSensor {
    pub unit:     String,
    pub label:    String,
    pub path:     PathBuf,
    pub current:  f32,
    pub model:    String,
}

impl NdirSensor {
    pub async fn select(path: &Path) -> Result<NdirSensor> {
        let model    = model(path).await?;
        let unit     = setting::unit(path).await?;
        let label    = setting::label(path).await?;
        let current  = signal::current(path).await?;
        Ok(NdirSensor{unit,label,address,path,current})
    }
    /// Returns sensor unit name.
    pub fn unit(&self) -> &str {
        self.unit.as_ref().map(|s| s.as_str())
    }

    /// Returns sensor label.
    pub fn label(&self) -> Option<&str> {
        &self.label
    }

    /// Returns current presure reported by sensor.
    pub fn current(&self) -> f32 {
        self.current
    }
}


pub async fn model(path: &Path) -> Result<String> {
    let model = fs::read_to_string(path.join("ndir")).await?;
    Ok(model)
}




// pub async
// pub async fn set_addres(pa)

// pub async fn simulation(path: PathBuf) {
// }

