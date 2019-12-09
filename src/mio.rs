use std::path::{Path,PathBuf};

// use std::io;
use std::fs;
use std::process::Command;

use super::*;
// use lazy_static::lazy_static;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::time::Duration;
// use async_std::sync::channel;
// use crossbeam::channel::{Sender,Receiver};
// use async_std::task;
// use crossbeam::channel as channel;





/// Create mio 
pub struct Mio {
    pub path : PathBuf,
}
 
impl Mio {
    /// mount new mio fs
    pub fn mount(path:&Path) -> Result<Mio> {
        let path = path.join("mio");
        if  !path.is_dir(){
            log::info!("creatte new mio space in {}", path.display());
            fs::create_dir_all(&path)?;
        }
        Ok(Mio{
            path:path,
        })
    }
    pub fn simulation_driver(&self) -> Result<Simulate> {
        Ok(Simulate::create(self,"simulate")?)
    }

    pub fn create_interface(&mut self,name:&str) -> Result<Interface> {
        let path = self.path.join(name);
        if !path.is_dir() {
            log::info!("mio create new device {} in {}",name,path.display());
            fs::create_dir_all(&path)?;
        }
        Ok(Interface::from(path))
    }

    pub fn watch(&self ) -> Result<()>{

        let (tx, rx) = std::sync::mpsc::channel();
            // Command::new("mount").arg("-t").arg("tmpfs").arg("-t").arg("-o").arg("user,size=100M").arg("tmpfs").arg(path).spawn()?;
        let tx = tx.clone();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(200)).unwrap();
        watcher.watch(&self.path, RecursiveMode::Recursive).unwrap();
        for event in rx {
                log::info!("changed:{:?}", event);
        };
        Ok(())
    }
}


impl Drop for Mio {
    fn drop(&mut self) {
        log::info!("unmount {:?} miofs", self.path);
    }
}

pub struct Workspace {
   pub mio :Mio,
}

impl Workspace {
    pub fn create() -> Result<Workspace>  {
        let rundir = PathBuf::from("/run/user/1000");
        let io = Mio::mount(&rundir)?;
        Ok(Workspace{
            mio: io,
        })
    }
}

