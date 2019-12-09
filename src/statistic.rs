/// Interface statictic
///
// use std::prelude::*;
use std::fs;
// use std::task;
// use sysfs_gpio::{Direction,Pin};
use std::path::{ PathBuf};
// use std::fmt;

// use async_trait::async_trait;
use super::*;

// pub const LABEL: &'static str = "label";
pub const ERRORS_COUTN: &'static str = "errors_count";
pub const LAST_ERROR: &'static str = "last_error";
// pub const DESCRIPTION: &'static str = "description";
// pub const MODEL: &'static str = "model";
// pub const ERROR: &'static str = "error";
// pub const TYPE: &'static str = "type";
// pub const SIMULATE: &'static str = "simulate";

use std::convert::TryFrom;

pub struct Statistic{
    path: PathBuf,
}

impl TryFrom<Interface> for Statistic {
    type Error = Error;

    fn try_from(iface: Interface) -> Result<Self> {
        let stats = iface.path.join("statistic");
        if stats.is_dir() {
            fs::create_dir(&stats)?;
        }
        Ok(Statistic{
            path:stats,
        })
    }
}


impl Statistic {
    pub fn errors(&self) -> Result<u32> {
        let count = fs::read_to_string(self.path.join(ERRORS_COUTN))?.parse::<u32>()?;
        Ok(count)
    }
    pub fn report_error(&self,err:Error)->Result<()> {
        fs::write(self.path.join(ERRORS_COUTN),format!("{}",self.errors()?+1).as_bytes())?;
        fs::write(self.path.join(LAST_ERROR),format!("{}",err).as_bytes())?;

        Ok(())
    }
}
