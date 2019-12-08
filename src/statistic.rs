/// Interface statictic 
/// 
// use std::prelude::*;
use std::fs;
// use std::task;
// use sysfs_gpio::{Direction,Pin};
use std::path::{ PathBuf};
use std::fmt;

// use async_trait::async_trait;
use super::*;

// pub const LABEL: &'static str = "label";
// pub const UNIT: &'static str = "unit";
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

}