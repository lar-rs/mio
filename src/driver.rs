/// Driver interface 
/// 
// use sysfs_gpio::{Direction,Pin};
use std::path::{PathBuf,Path};

// use async_trait::async_trait;
use super::*;



/// Simulate.
pub struct Driver {
    pub path : PathBuf,
}


impl Driver {
    pub fn open(path:&Path) -> Result<Driver> {
        let path = path.to_path_buf();
        if !path.is_dir() {
            Err(Error::Interface{msg: format!("{} driver not run",path.as_path().display())})
        }else {
            Ok(Driver{path})
        }
    }
}
