/// Analog [IN:OUT]
///  * input12
///  * output32
///

use std::path::{PathBuf};
use super::*;
use std::fs;
use std::convert::TryFrom;





pub struct Analog {
    pub path: PathBuf,
}


impl TryFrom<Interface> for Analog {
    type Error = Error;
    fn try_from(iface: Interface) -> Result<Self> {
        iface.set_itype(IType::ADC)?;
        Ok(Self{
            path:iface.path,
        })
    }
}

impl Analog {
    pub fn read_value(&self) -> Result<u32> {
        let value = fs::read_to_string(self.path.join("value"))?.parse::<u32>()?;
        Ok(value)
    }
    pub fn write_value(&self,value:u32) -> Result<u32> {
        fs::write(self.path.join("value"),format!("{}",value).as_bytes())?;
        Ok(value)
    }
}
