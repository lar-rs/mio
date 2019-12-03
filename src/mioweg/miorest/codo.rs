// use std::fmt;
// pub use uom::si::f32::{Ratio,MassRate,ElectricCurrent};
use async_std::prelude::*;
use async_std::fs;
use async_std::path::{ Path, PathBuf};
use async_std::stream;
use async_std::io::Result;

use serde_json::from_str;
use serde::{Deserialize, Serialize};
use futures_timer::Delay;

use std::str::FromStr;

pub struct CodSensor {
    sensor: Sensor,
    // pub unit:     String,
    // pub label:    String,
    // pub address:  String,
    // pub path:     PathBuf,
    // pub current:  f32,
}

pub async fn model(path: &Path) -> Result<String> {
    let model = fs::read_to_string(path.join("codo")).await?;
    Ok(model)
}

pub async fn select(path: &Path) -> Result<CodSensor> {
    let model = model(path).async?;
    let sensor = super::select(path).await?;
    Ok(NdirSensor{sensor}) 

}

// use crate::sys;
// use lazy_static::lazy_static;

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub enum State {
//     Brocket,
//     Signal,
//     Average,
//     Integration,
// }

// #[derive(Serialize,Deserialize, Clone, Debug)]
// pub struct Signal {
// /// time in mili seconds
//     pub timestamp:   u64,
// /// Value
//     pub value:  f64,
// }

// impl Signal {
//     pub fn random() -> Signal {
//         Signal{
//             time:Utc::now().timestamp_millis() as u64,

//         }
//     }
// }
// use rand::Rng;

// pub struc
/// Hardware airflow sensor.


// impl NdirSensor {
//     /// Returns sensor unit name.
//     pub fn unit(&self) -> &str {
//         &self.unit
//     }

//     /// Returns sensor label.
//     pub fn label(&self) -> &str {
//         &self.label
//     }

//     /// Returns current presure reported by sensor.
//     pub fn current(&self) -> f32 {
//         self.current
//     }

    // pub fn signal(&mut self) -> impl Stream<Item = Result<f32>> {
    //     let path = self.path.join("current");
    //     let s = stream::repeat_with(|| async {
    //         let mut rng = rand::thread_rng();

    //         // let signal  = fs::read_to_string(&path.join("current")).await?;
    //         // let current:Signal = from_str(&signal)?;
    //         // current;
    //         Ok(rng.gen::<f32>())

    //      });
    //     pin_utils::pin_mut!(s);
    //     s 
    // }
// }

/// current 
// pub async fn (path: &Path) -> Result<f32> {
    // let path = path.join("signal");
    // let current = f32::from_str(&fs::read_to_string(path).await?).unwrap();
    // Ok(current)

// }



// pub async fn set_addres(pa)

// pub async fn simulation(path: PathBuf) {
// }

// pub fn fsr(sensor: NdirSensor) -> impl Stream<Item = Result<f32>>
