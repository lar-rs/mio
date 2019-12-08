use serde::{Deserialize, Serialize};
use super::*;
use std::fs;
use std::path::{PathBuf};

pub const INPUT:  &'static str     = "input";
pub const OUTPUT: &'static str     = "output";
pub const CORELATION: &'static str = "corelation";
pub const REFERENCE: &'static str  = "reference";
pub const DEVIATION: &'static str  = "deviation";

/// für 0..60:   0.230197;
// static AC:[f32;7] = [0.003836617, -0.06027397,0.3727283,-1.1430475,1.83842,-1.4032,0.39159];
// für 0..60:  -3.616438;
// static A5:f32 =
// für 0..60:  22.36370;
// static A4:f32 = 
// für 0..60: -68.58285;
// static A3:f32 = ;
// für 0..60: 110.3052;
// static A2:f32 = ;
// für 0..60: -84.19201;
// static A1:f32 = ;
// für 0..60:  23.49542;
// static A0:f32 = ;


#[derive(Serialize,Default, Deserialize, Clone, Debug)]
pub struct Config {
    pub correlation:          f32,
    pub soll_value:           f32,
    pub input_deviation:      f32,
    pub output_deviation:     f32,
    pub monitorin_interval:   u64,
}

impl From<&Interface> for Airflow {
    #[inline]
    fn from(device:&Interface) -> Airflow {
        Airflow{
            path: device.path.to_path_buf()
        }
    }
}

use std::convert::TryFrom;
/// interface transfer
impl TryFrom<Interface> for Airflow {
    type Error = Error;
    fn try_from(iface: Interface) -> Result<Self> {
        iface.set_itype(IType::Airflow)?;
        Ok(Self{
            path:iface.path,
        })
    }
}


/// Hardware airflow sensor.
pub struct Airflow {
    pub path : PathBuf,
}

impl Airflow {
    pub fn create(iface:Interface) -> Result<Airflow> {
        //     fs::write(dev.path.join(INPUT), b"0")?;
        //     fs::write(dev.path.join(OUTPUT), b"0")?;
        //     fs::write(dev.path.join(CORELATION), b"1.0")?;
        //     fs::write(dev.path.join(REFERENCE), b"30.0")?;
        //     fs::write(dev.path.join(DEVIATION), b"5.0")?;
        Ok(Airflow::try_from(iface)?)
    }

    /// Returns volume rate on input reported by sensor.
    pub fn input(&self) -> Result<f32> {
        let value = fs::read_to_string(self.path.join(INPUT))?.parse::<f32>()?;
        Ok(value)
    }
    /// Returns volume rate on input reported by sensor.
    pub fn output(&self) -> Result<f32> {
        let value = fs::read_to_string(self.path.join(OUTPUT))?.parse::<f32>()?;
        Ok(value)
    }
    /// corelation value
    pub fn corelation(&self) -> Result<f32> {
        let value = fs::read_to_string(self.path.join(CORELATION))?.parse::<f32>()?;
        Ok(value)
    }
    /// corelation value
    pub fn reference(&self) -> Result<f32> {
        let value = fs::read_to_string(self.path.join(REFERENCE))?.parse::<f32>()?;
        Ok(value)
    }
    /// maximale erlaubte abweichung in procent 
    pub fn deviation(&self) -> Result<f32> {
        let value = fs::read_to_string(self.path.join(DEVIATION))?.parse::<f32>()?;
        Ok(value)
    }
}


// pub fn airflow() -> Result<Airflow> {
    // let path = super::miofs()?.join("airflow");
    // Ok(Airflow{path}) 
// }



//