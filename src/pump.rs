/// Monitor gear pump normally used for solution sampling.
///
///
use std::path::{PathBuf,Path};
use std::io;
use std::fs;
use std::fmt;
use serde::{Serialize,Deserialize};

use super::*;


pub struct State{
    mode: Mode,
}


pub enum Message{
    SetState(Mode),
}

use std::convert::TryFrom;
impl TryFrom<Interface> for GearPump {
    type Error = Error;
    fn try_from(iface: Interface) -> Result<Self> {
        iface.set_itype(IType::GearPump)?;
        iface.set_iclass(IClass::Pump)?;
        Ok(Self{
            path:iface.path,
        })
    }
}

 /// GearPump
 pub struct GearPump {
    path: PathBuf,
}

impl GearPump {
    pub fn open(path:&Path) -> io::Result<GearPump> {
        let path = path.to_path_buf();
        Ok(GearPump{
            path:path,
        })
    }
    pub fn start(&mut self)  -> io::Result<()> {
        fs::write(self.path.join("value"),format!("{}",Mode::Start).as_bytes())?;
        Ok(())
    }
    pub fn stop(&mut self) -> io::Result<()> {
        fs::write(self.path.join("value"),format!("{}",Mode::Stop).as_bytes())?;
        Ok(())
    }
}
/// [6210sub1] ParameterName=0=Stop, 1=Start 2=Wait ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=- PDOMapping=0
/// [6210sub2] ParameterName=0=Normal, 1=Spulen,2=Intervall ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=- PDOMapping=0
/// [6210sub3] ParameterName=Drehrichtung rechts / links ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
/// [6210sub4] ParameterName=Speed soll ObjectType=0x7 DataType=0x0002 AccessType=rw DefaultValue=0 PDOMapping=0
/// [6210sub5] ParameterName=Interwall Pos-Impulse ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
/// [6210sub6] ParameterName=Interwall Time / sec ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
/// [6210sub7] ParameterName=Position ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=0 PDOMapping=0
/// [6210sub8] ParameterName=Delay PW ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=0 PDOMapping=0
/// [6211] ParameterName=Konstanten 2 ObjectType=0x8 SubNumber=4
/// [6211sub0] ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=3 PDOMapping=0
/// [6211sub1] ParameterName=K-p ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=6 PDOMapping=0
/// [6211sub2] ParameterName=K-d ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=1 PDOMapping=0
/// [6211sub3] ParameterName=K-i ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=1 PDOMapping=0
///
#[derive(Clone, Copy,Serialize,Deserialize, Debug, PartialEq, Eq)]
pub enum Mode {
    Stop,
    Start,
    Wait,
}


impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        match value {
            0 => Mode::Stop,
            1 => Mode::Start,
            2 => Mode::Wait,
            _ => Mode::Stop,
        }
    }
}

impl From<Mode> for u8 {
    fn from(state:Mode) -> u8 {
        state.into()
    }
}

impl From<&str> for Mode {
    fn from(value: &str) -> Self {
        match value {
            "stop"  =>  Mode::Stop,
            "start" =>  Mode::Start,
            "wait"  =>  Mode::Wait,
            _       =>  Mode::Stop
        }
    }
}

impl From<String> for Mode {
    fn from(value: String) -> Self {
        Mode::from(value.as_str())
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Mode::Stop  =>  return write!(f,"stop"),
            Mode::Start =>  return write!(f,"start"),
            Mode::Wait  =>  return write!(f,"wait"),
        }
    }
}
impl TryFrom<Interface> for ImpulsePump {
    type Error = Error;
    fn try_from(iface: Interface) -> Result<Self> {
        iface.set_itype(IType::ImpulsPump)?;
        iface.set_iclass(IClass::Pump)?;
        Ok(Self{
            path:iface.path,
        })
    }
}

pub struct ImpulsePump {
    path: PathBuf,
}

impl ImpulsePump {
    pub fn select(path: &Path) -> Result<ImpulsePump> {
        Ok({
            ImpulsePump{
                path: path.to_path_buf(),
            }
        })
    }
    pub fn start(&mut self)  -> io::Result<()> {
        fs::write(self.path.join("value"),format!("{}",Mode::Start).as_bytes())?;
        // digital::set_high(self.path.as_path())?;
        Ok(())
    }
    pub fn wait(&mut self)  -> io::Result<()> {
        fs::write(self.path.join("value"),format!("{}",Mode::Wait).as_bytes())?;
        Ok(())
    }
    pub fn stop(&mut self) -> io::Result<()> {
        fs::write(self.path.join("value"),format!("{}",Mode::Stop).as_bytes())?;
        Ok(())
    }
}



// pub fn gear_pump(path:&Path) -> Result<GearPump> {
//     // let dev =  device::create(path,IType::GearPump)?;
//     fs::write(path.join("value"), b"0")?;
//     Ok(GearPump::from(dev))
// }
