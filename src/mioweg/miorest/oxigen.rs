// use std::fmt;
// pub use uom::si::f32::{Ratio,MassRate,ElectricCurrent};
// use std::collections::VecDeque;    
use async_std::prelude::*;
use async_std::fs;
use async_std::path::{ Path, PathBuf};
// use async_std::stream;
// use async_std::stream::FromStream;
use async_std::io::Result;
// use serde_json::from_str;
// use serde::{Deserialize, Serialize};
// use futures_timer::Delay;
// use std::str::FromStr;

/// Hardware ndir sensor.
// #[derive(Clone, Debug, PartialEq, Eq)]
pub struct Oxigen {
    path :PathBuf,

}
impl NdirSensor {
    pub async fn fsr(&self) -> Result<f32> {
        Ok(super::value(&self.path).await?)
    }
}

// pub async fn model(path: &Path) -> Result<String> {
//     let model = fs::read_to_string(path.join("ndir")).await?;
//     Ok(model)
// }
// pub async fn check(path:&Path) -> bool {
//    path.is_dir().await && path.join("ndir").exists().await 
// }


// pub async fn connect(path: &Path) -> Result<NdirSensor> {
//     let _model = model(path).await?;
//     // let sensor = super::select(path).await?;
//     let path = path.to_path_buf();
//     Ok(NdirSensor{
//         path:path})

// }
// pub async fn get_fsr ( ndir : &mut NdirSensor ) -> Result<f32> {
    // Ok(super::value(&ndir.path).await?)
    // let s = stream::repeat_with(|| async { 0.1 }).fuse();
    // pin_utils::pin_mut!(s);
    // s
// }
// pub async fn signal(ndir : &mut NdirSensor) -> Result<impl Stream<Item = f32>> {

// }

pub async fn sensors(path:&Path) -> Result<Vec<NdirSensor>> {
    // let mut entries = fs::read_dir`(".").filter(|entry| future::ready(entry.file_name().as_bytes().starts_with(b"hwmon")))
    // let sensors: Vec<NdirSensor> = s.collect().await;
    // let mut entries = fs::read_dir(path).await.unwrap();
    // let sensors = entries.filter_map()
    let mut sensors: Vec<NdirSensor> = Vec::new();
    let mut dir = fs::read_dir(path).await?;
    while let Some(res) = dir.next().await {
        let path = res?.path();
        if path.is_dir().await {
            if  path.join("ndir").exists().await {
                sensors.push(connect(&path).await?);
            }
        }
    }
    Ok(sensors)
} 




    // s.filter_map(|a| a.parse::<u32>().ok());

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

// current 
// pub async fn (path: &Path) -> Result<f32> {
    // let path = path.join("signal");
    // let current = f32::from_str(&fs::read_to_string(path).await?).unwrap();
    // Ok(current)

// }



// pub async fn set_addres(pa)

// pub async fn simulation(path: PathBuf) {
// }

// pub fn fsr(sensor: NdirSensor) -> impl Stream<Item = Result<f32>>
