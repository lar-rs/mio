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
pub struct Workspace {
    pub path : PathBuf,
    confdir: PathBuf,
}
 
impl Workspace {
    /// mount new mio fs
    pub fn setup() -> Result<Workspace> {
        let rundir  =  dirs::runtime_dir().unwrap_or(PathBuf::from(".")).join("mio");
        let config  =  dirs::config_dir().unwrap_or(PathBuf::from(".config")).join("mio");
        if  !rundir.is_dir(){
            log::info!("setup mio working space in {}", rundir.display());
            fs::create_dir_all(&rundir)?;
            fs::create_dir_all(rundir.join("interfaces"))?;
            let class = rundir.join("class");
            fs::create_dir_all(&class)?;
        }
        if !config.is_dir(){
            fs::create_dir_all(&config)?;
        }
        Ok(Workspace{
            path:rundir,
            confdir:config,
        })
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
    pub fn get_interface_dir(&self)  -> PathBuf {
        self.path.join("interfaces")
    }
    pub fn get_class_dir(&self)  -> PathBuf {
        self.path.join("class")
    }
}


impl Drop for Workspace {
    fn drop(&mut self) {
        log::info!("unmount {:?} miofs", self.path);
    }
}