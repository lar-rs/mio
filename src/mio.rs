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
    owner: bool,
}
 
impl Mio {
    /// mount new mio fs
    pub fn mount(path:&Path,name:&str) -> Result<Mio> {
        if !path.is_dir() {
            let (tx, rx) = std::sync::mpsc::channel();
            fs::create_dir_all(path)?;
            fs::write(path.join("name"),name)?;

            let miofs = path.to_path_buf();
            // Command::new("mount").arg("-t").arg("tmpfs").arg("-t").arg("-o").arg("user,size=100M").arg("tmpfs").arg(path).spawn()?;
            let _ = std::thread::spawn(move || {
                let tx = tx.clone();
                let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();
                watcher.watch(&miofs, RecursiveMode::Recursive).unwrap();
                for event in rx {
                    log::info!("changed:{:?}", event);
                };
            });
            Ok(Mio{
                path:path.join("mio"),
                owner: true,
            })
        }else {
            Err(Error::Mount{msg:format!(" {} in {} alredy exist",path.display(),name)})
        }
    }
    pub fn umount(mio:&Mio) {
        
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
}


impl Drop for Mio {
    fn drop(&mut self) {
        if self.owner {
            Command::new("umount").arg(self.path.as_path()).spawn().expect("umount miofs error");
            log::info!("unmount {:?} miofs", self.path);
        }
    }
}