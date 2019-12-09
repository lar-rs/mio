/// Driver interface 
/// 
use std::fs;
// use sysfs_gpio::{Direction,Pin};
use std::path::{PathBuf};

// use async_trait::async_trait;
use super::*;



/// Simulate.
pub struct Simulate {
    pub path : PathBuf,
}


impl Simulate {
    pub fn create(mio:&Mio,name:&str) -> Result<Simulate> {
        let path = mio.path.join("driver/").join(name);
        if !path.is_dir() {
            fs::create_dir_all(&path)?;
        }
        Ok(Simulate{path})
    }
}


