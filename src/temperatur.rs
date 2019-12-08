/// Temperatur sensor
/// Anschlus:  `Analog:TEMP`
///
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TemperaturSetting {
    pub monitorin_interval:   u64,
    // pub injection_threshold:  f32,
}



impl Default for TemperaturSetting {
    fn default() -> Self {
        Self {
            monitorin_interval:   0,
        }
    }
}



use std::convert::TryFrom;
impl TryFrom<Interface> for Temperatur {
    type Error = Error;
    fn try_from(iface: Interface) -> Result<Self> {
        iface.set_itype(IType::Temperatur)?;
        Ok(Self{
            path:iface.path,
        })
    }
}

/// Temperature
///
/// fsr - full scale range
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Temperatur {
    pub path : PathBuf,
}

impl Temperatur {
    pub fn from_analog16(value : u16) -> Temperatur {
        let signal =  value as f32 / 10.0;
        let broken = value>1000;

        Temperatur{
            value: signal,
            broken: broken,
        }
    }
}


// pub fn setup() -> Result<()> {
    // Ok(())
// }
//
// pub fn Temperatur_value() -> Result<Temperatur, MioError> {
    // let analog_value  = io::analog_input16(0x4)?;
    // Ok(Temperatur::from_analog16(analog_value))
// }
//
