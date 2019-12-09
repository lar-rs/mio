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
use std::collections::BTreeSet;




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
            fs::create_dir_all(path.join("interfaces"))?;
            let class = path.join("class");
            fs::create_dir_all(&class)?;
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
        }else{
            log::info!("interface {} in {} alreadey exist",name ,self.path.as_path().display());
        }
        Ok(Interface::from(path))
    }

    pub fn get_interfaces(&self) -> Result<Vec<Interface>> {
        let path = self.path.join("interfaces");
        let mut interfaces:Vec<Interface> = Vec::new();
        if path.is_dir() {
            for entry in fs::read_dir(&path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let iface = Interface::create(&path)?;
                    interfaces.push(iface);
                }
            }
        }
        Ok(interfaces)
    }
    pub fn get_classified_interfaces(&self,class:IClass)  -> Result<Vec<Interface>> {
        let path = self.path.join("class").join(format!("{}",class));
        let mut interfaces:Vec<Interface> = Vec::new();
        if path.is_dir() {
            for entry in fs::read_dir(&path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let iface = Interface::create(&path)?;
                    interfaces.push(iface);
                }
            }
        }
        Ok(interfaces)
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
    pub fn get_interface_dir(mio:&Mio)  -> PathBuf {
        mio.path.join("interfaces")
    }
    pub fn get_class_dir(mio:&Mio)  -> PathBuf {
        mio.path.join("class")
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

