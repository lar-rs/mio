/// Monitor gear pump normally used for solution sampling.
///
///
use async_std::path::{PathBuf,Path};
use async_std::io;
use async_std::fs;
use std::fmt;

use super::{MioError,HID,Driver,driver};
use super::digital;

 /// Lamp simulation
 pub struct GearPump {
    path: PathBuf,
}
impl From<Driver> for GearPump {
    fn from(drv:Driver) -> GearPump {
        GearPump{
            path: drv.path
        }
    }
}

impl GearPump {
    pub async fn select(path: &Path) -> io::Result<GearPump> {
        Ok({
            GearPump{
                path: path.to_path_buf(),
            }
        })
    }
    pub async fn start(&mut self)  -> io::Result<()> {
        digital::set_high(self.path.as_path()).await?;
        Ok(())
    }
    pub async fn stop(&mut self) -> io::Result<()> {
        digital::set_low(self.path.as_path()).await?;
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Stop,
    Start,
    Wait,
}


impl From<u8> for State {
    fn from(value: u8) -> Self {
        match value {
            0 => State::Stop,
            1 => State::Start,
            2 => State::Wait,
            _ => State::Stop,
        }
    }
}

impl From<State> for u8 {
    fn from(state:State) -> u8 {
        state.into()
    }
}

impl From<&str> for State {
    fn from(value: &str) -> Self {
        match value {
            "stop"  =>  State::Stop,
            "start" =>  State::Start,
            "wait"  =>  State::Wait,
            _       =>  State::Stop
        }
    }
}

impl From<String> for State {
    fn from(value: String) -> Self {
        State::from(value.as_str())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Stop  =>  return write!(f,"stop"),
            State::Start =>  return write!(f,"start"),
            State::Wait  =>  return write!(f,"wait"),
        }
    }
}
pub struct ImpulsePump {
    path: PathBuf,
}

impl ImpulsePump {
    pub async fn select(path: &Path) -> Result<ImpulsePump,MioError> {
        Ok({
            ImpulsePump{
                path: path.to_path_buf(),
            }
        })
    }
    pub async fn start(&mut self)  -> io::Result<()> {
        
        digital::set_high(self.path.as_path()).await?;
        Ok(())
    }
    pub async fn wait(&mut self)  -> io::Result<()> {
        digital::set_high(self.path.as_path()).await?;
        Ok(())
    }
    pub async fn stop(&mut self) -> io::Result<()> {
        digital::set_low(self.path.as_path()).await?;
        Ok(())
    }
}

pub async fn gearpump(path:&Path) -> io::Result<GearPump> {
    driver::create(path,HID::GearPump).await?;
    fs::write(path.join("value"), b"0").await?;
    fs::write(path.join("pump"), b"simulation").await?;
  
    let gp = GearPump::select(path).await?;
    Ok(gp)
}