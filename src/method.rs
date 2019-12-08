/// Methode
/// 
/// 
use std::fs;
// use sysfs_gpio::{Direction,Pin};
use std::path::{PathBuf};
// use std::fmt;

// use async_trait::async_trait;
use super::*;

pub const NAME: &'static str = "name";
pub const UNIT: &'static str = "unit";
pub const DESCRIPTION: &'static str = "description";
pub const ERROR: &'static str = "error";

use std::convert::TryFrom;

impl TryFrom<Interface> for Method {
    type Error = Error;
    fn try_from(iface: Interface) -> Result<Self> {
        iface.set_itype(IType::Method)?;
        Ok(Self{
            path:iface.path,
        })
    }
}

/// Interface 
/// 
pub struct Method{
    pub path: PathBuf,
}


impl Method {
    pub fn name(&self) -> Result<String> {
        let label = fs::read_to_string(self.path.join(NAME))?;
        Ok(label)
    }
    pub fn unit(&self) -> Result<String> {
        let unit= fs::read_to_string(self.path.join(UNIT))?;
        Ok(unit)
    }
    pub fn description(&self)-> Result<String> {
        let desc= fs::read_to_string(self.path.join(DESCRIPTION))?;
        Ok(desc) 
    }
    pub fn is_error(&self) -> Result<bool> {
        if self.path.join(ERROR).is_file() {
            Ok(true)
        }else {
            Ok(false)
        }
    }
}

