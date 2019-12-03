/// Autosampler
///
///
///
use nb;

pub trait Autosampler{
    type Error;
    type Position;
    type Speed;

    fn hold(&mut self) -> nb::Result<Self::Position,Self::Error>;
    fn position(&mut self, pos:Self::Position) -> nb::Result<Self::Position,Self::Error>;
    fn take(&mut self, par:Self::Speed) -> nb::Result<(),Self::Error>;
    fn push(&mut self, par:Self::Speed) -> nb::Result<(),Self::Error>;
}

use serde_derive::{Deserialize, Serialize};

use crate::components::{
    axis::Velocity,
    injection::Injection,
    vessel::Vessel,
    dilution::Dilution,
};

/// AxisConfig
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AxisConfig {
    pub hold: u32,
    pub max:u32,
    pub power: u32,
}

impl AxisConfig {
    pub fn new(hold:u32,max:u32) -> Self {
        let power= 1200;
        Self {
            hold: hold,
            max: max,
            power:1200
        }
    }
}

impl Default for AxisConfig {
    fn default() -> Self{
        Self{
            hold: 10,
            max:1800,
            power:1200,
        }
    }
}


/// 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AutosamplerConfig {
    pub x:         AxisConfig,
    pub y:         AxisConfig,
    pub inj:       AxisConfig,
    pub injection: Injection,
    #[serde(default)]
    pub vessels:   Vec<Vessel>,
    pub dilution:  Dilution,
}


impl Default for AutosamplerConfig {
 fn default() -> Self {
       Self {
           x:AxisConfig::new(40,2400),
           y:AxisConfig::new(80,2000),
           inj:AxisConfig::new(10,1800),
           injection:Injection::default(),
           vessels: vec![
               Vessel::new(40,600,1450),
               Vessel::new(1100,480,1250),
               Vessel::new(1200,700,1250),
               Vessel::new(1500,700,1250),
               Vessel::new(1700,700,1250),
               Vessel::new(1800,700,1250),
               Vessel::new(2000,700,1250),
               Vessel::new(2300,700,1250),
           ],
            dilution:Dilution::default(),
       }
   }
}


impl AutosamplerConfig {
    pub fn furnace(&self) -> Vessel {
        if let Some(furnace) = self.vessels.get(0) {
            furnace.clone()
        } else {
            Vessel::new(40,600,1450)
        }
    }
}


// #[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
// pub enum AutosamplerMutation {
//     XAxis(Axis),
//     YAxis(Axis),
//     InjAxis(Axis),
//     Injection(Injection),
//     Vessel(usize,Vessel),
//     Dilution(Dilution),
// }



// #[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
// pub enum XYAction {
//     SetupX(Axis),
//     SetupY(Axis),
//     SetupZ(Axis),
//     MoveX(AxisMove),
//     MoveY(AxisMove),
//     MoveZ(AxisMove),
//     SensorX(),
//     SensorY(),
//     SensorZ(),
// } 
