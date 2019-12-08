/// Valve I/O
/// 
use std::io;
use std::fs;
use std::path::{PathBuf};
// use std::time::{Duration};
use super::*;


// /// Single digital push-pull output pin
// #[async_trait]
// pub trait Valve {
//     /// Valve open
//     ///
//     /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
//     /// electrical sources
//    fnopen(&mut self) -> Result<()>;

//     /// Valve close
//     ///
//     /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
//     /// electrical sources
//    fnclose(&mut self) -> Result<()>;
// }

use std::convert::TryFrom;
impl TryFrom<Interface> for Valve {
    type Error = Error;
    fn try_from(iface: Interface) -> Result<Self> {
        iface.set_itype(IType::Valve)?;
        Ok(Self{
            path:iface.path,
        })
    }
}


 /// Lamp simulation
 pub struct Valve {
    path: PathBuf,
}

impl Valve {
    pub fn open(&mut self) -> io::Result<()> {
        fs::write(self.path.join("value"), b"1")?;
        Ok(())
    }
    pub fn close(&mut self) -> io::Result<()> {
        fs::write(self.path.join("value"), b"0")?;
        Ok(())
    }
}

// pub fn create(path:&Path) -> Result<Valve> {
//     let dev = device::create(path,IType::Valve)?;
//     fs::create_dir_all(path)?;
//     fs::write(path.join("value"), b"0")?;
//     Ok(dev.into())
// }
