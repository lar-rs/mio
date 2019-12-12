/// Driver interface 
/// 
// use sysfs_gpio::{Direction,Pin};
use std::path::{PathBuf,Path};

// use async_trait::async_trait;
use super::*;
use std::process;
use std::os::unix::fs;

/// Simulate.
pub struct Driver {
    pub path : PathBuf,
    pub pid : u32,
}


impl Driver {
    pub fn open(path:&Path,pid:u32) -> Result<Driver> {
        let path = path.to_path_buf();
        if !path.is_dir() {
            Err(Error::Interface{msg: format!("{} driver not run",path.as_path().display())})
        }else {
            Ok(Driver{path,pid})
        }
    }
}

impl Drop for Driver {
    fn drop(&mut self) {
        if self.pid == process::id(){
            let path = self.path.join(format!("{}",self.pid));
            if let Ok(_) =  std::fs::symlink_metadata(&path) {
                log::info!("driver {} shutdown",path.as_path().display());
                std::fs::remove_file(path).expect_err("remove driver link error");
            }
        }
    }
}

pub fn start(path: &Path) -> Result<Driver> {
    let pid = process::id();
    log::info!("new driver {} start",pid);
    let src = PathBuf::from(format!("/proc/{}",pid));
    let dst = path.join(format!("{}",pid));
    fs::symlink(src,dst)?;
    Driver::open(path,pid)
}