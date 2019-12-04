/// Driver information 
/// 
// use async_std::prelude::*;
use async_std::fs;
// use async_std::task;
// use sysfs_gpio::{Direction,Pin};
use async_std::path::{ Path,PathBuf};
use std::fmt;

// use async_trait::async_trait;
use super::MioError;

pub const LABEL: &'static str = "label";
pub const UNIT: &'static str = "unit";
pub const DESCRIPTION: &'static str = "description";
pub const MODEL: &'static str = "model";
pub const ERROR: &'static str = "error";
pub const TYPE: &'static str = "type";
pub const SIMULATE: &'static str = "simulate";


pub enum HID {
    DigitalOut,
    DigitalIn,
    ADC,
    UART,
    Relay,
    Valve,
    Fluid,
    Lamp,
    GearPump,
    Furnace,
    ImpulsPump,
    Temperatur,
    Humidity,
    Airflow,
    Pressure,
    NDir,
    Zirox,
    NO5,
    Axis,
    Injection,
    Autosampler,
    Stirrer,
    Unknown
}


impl From<u8> for HID {
    fn from(value: u8) -> Self {
        match value {
            1 => HID::DigitalIn,
            2 => HID::DigitalOut,
            3 => HID::ADC,
            4 => HID::UART,
            5 => HID::Relay,
            6 => HID::Valve,
            7 => HID::Fluid,
            8 => HID::Lamp,
            9 => HID::GearPump,
           10 => HID::ImpulsPump,
           11 => HID::Furnace,
           12 => HID::Airflow,
           13 => HID::Pressure,
           14 => HID::Temperatur,
           15 => HID::Humidity,
           21 => HID::NDir,
           22 => HID::Zirox,
           23 => HID::NO5,
           41 => HID::Axis,
           42 => HID::Injection,
           51 => HID::Stirrer,
           110 => HID::Autosampler,
           _ =>   HID::Unknown
        }
    }
}

impl From<HID> for u8 {
    fn from(hid:HID) -> u8 {
        hid.into()
    }
}

impl From<&str> for HID {
    fn from(value: &str) -> Self {
        match value {
            "din"           => HID::DigitalIn,
            "dout"          => HID::DigitalOut,
            "adc"           => HID::ADC,
            "uart"          => HID::UART,
            "relay"         => HID::Relay,
            "valve"         => HID::Valve,
            "fluid"         => HID::Fluid,
            "lamp"          => HID::Lamp,
            "gearpump"      => HID::GearPump,
            "impulspump"    => HID::ImpulsPump,
            "furnace"       => HID::Furnace,
            "airflow"       => HID::Airflow,
            "pressure"      => HID::Pressure,
            "temperatur"    => HID::Temperatur,
            "humidity"      => HID::Humidity,
            "ndir"          => HID::NDir,
            "zirox"         => HID::Zirox,
            "no5"           => HID::NO5,
            "axis"          => HID::Axis,
            "injection"     => HID::Injection,
            "stirrer"       => HID::Stirrer,
            "autosampler"   => HID::Autosampler,
            _               => HID::Unknown,
        }
    }
}

impl From<String> for HID {
    fn from(value: String) -> Self {
        HID::from(value.as_str())
    }
}

impl fmt::Display for HID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HID::DigitalIn  => return write!(f,"din"),          
            HID::DigitalOut => return write!(f,"dout"),           
            HID::ADC        => return write!(f,"adc"),    
            HID::UART       => return write!(f,"uart"),     
            HID::Relay      => return write!(f,"relay"),      
            HID::Valve      => return write!(f,"valve"),      
            HID::Fluid      => return write!(f,"fluid"),      
            HID::Lamp       => return write!(f,"lamp"),     
            HID::GearPump   => return write!(f,"gearpump"),         
            HID::ImpulsPump => return write!(f,"impulspump"),           
            HID::Furnace    => return write!(f,"furnace"),        
            HID::Airflow    => return write!(f,"airflow"),        
            HID::Pressure   => return write!(f,"pressure"),         
            HID::Temperatur => return write!(f,"temperatur"),           
            HID::Humidity   => return write!(f,"humidity"),         
            HID::NDir       => return write!(f,"ndir"),     
            HID::Zirox      => return write!(f,"zirox"),      
            HID::NO5        => return write!(f,"no5"),    
            HID::Axis       => return write!(f,"axis"),     
            HID::Injection  => return write!(f,"injection"),          
            HID::Stirrer    => return write!(f,"stirrer"),        
            HID::Autosampler=> return write!(f,"autosamler"),            
            HID::Unknown    => return write!(f,"unknown"),        
        }
    }
}

pub struct Driver {
    pub path: PathBuf,
}
impl From<&Path> for Driver {
    fn from(path:&Path) -> Driver {
        Driver{path:path.to_path_buf()}
    }
}
impl From<PathBuf> for Driver {
    fn from(path:PathBuf) -> Driver {
        Driver{path:path.to_path_buf()}
    }
}
impl From<Driver> for PathBuf {
    fn from(driver:Driver) -> PathBuf {
        driver.path
    }
}
impl AsRef<Driver> for Driver {
    fn as_ref(&self) -> &Driver {
        self
    }
}
impl Driver {
    pub async fn label(&self) -> Result<String,MioError> {
        let label = fs::read_to_string(self.path.join(LABEL)).await?;
        Ok(label)
    }
    pub async fn unit(&self) -> Result<String,MioError> {
        let unit= fs::read_to_string(self.path.join(UNIT)).await?;
        Ok(unit)
    }
    pub async fn description(&self)-> Result<String,MioError> {
        let desc= fs::read_to_string(self.path.join(DESCRIPTION)).await?;
        Ok(desc) 
    }
    pub async fn model(&self) -> Result<String,MioError> {
        let unit= fs::read_to_string(self.path.join(MODEL)).await?;
        Ok(unit)
    }
    pub async fn hid(&self) -> Result<HID,MioError> {
        let hid = fs::read_to_string(self.path.join(TYPE)).await?;
        Ok(hid.into())
    }
    pub async fn change_hid(&self,hid:HID) -> Result<(),MioError> {
        fs::write(self.path.join(TYPE),format!("{}",hid).as_bytes()).await?;
        Ok(())
    }
    pub async fn is_error(&self) -> Result<bool,MioError> {
        if self.path.join(ERROR).is_file().await {
            Ok(true)
        }else {
            Ok(false)
        }
    }
    pub async fn is_simulate(&self) -> Result<bool,MioError> {
        if self.path.join(SIMULATE).is_file().await {
            Ok(true)
        }else {
            Ok(false)
        }
    }
    pub async fn error(&self) -> Result<String,MioError> {
        let error = fs::read_to_string(self.path.join(ERROR)).await.unwrap_or(String::from(""));
        Ok(error)
    }    
}

pub async fn label(path:&Path) -> Result<String,MioError> {
    let label = fs::read_to_string(path.join(LABEL)).await?;
    Ok(label)
}
pub async fn unit(path:&Path) -> Result<String,MioError> {
    let unit= fs::read_to_string(path.join(UNIT)).await?;
    Ok(unit)
}
pub async fn description(path:&Path)-> Result<String,MioError> {
    let desc= fs::read_to_string(path.join(DESCRIPTION)).await?;
    Ok(desc) 
}
pub async fn model(path:&Path) -> Result<String,MioError> {
    let unit= fs::read_to_string(path.join(MODEL)).await?;
    Ok(unit)
}

pub async fn is_error(path:&Path) -> Result<bool,MioError> {
    if path.join(ERROR).is_file().await {
        Ok(true)
    }else {
        Ok(false)
    }
}
pub async fn error(path:&Path) -> Result<String,MioError> {
    let error = fs::read_to_string(path.join(ERROR)).await.unwrap_or(String::from(""));
    Ok(error)
}

pub async fn create(path:&Path,hid:HID) -> Result<Driver,MioError> {
    fs::create_dir_all(path).await?;
    fs::write(path.join(TYPE),format!("{}",hid).as_bytes()).await?;
    fs::write(path.join(SIMULATE),format!("{}:simulation",hid).as_bytes()).await?;
    fs::write(path.join(MODEL),format!("{}:simulation",hid).as_bytes()).await?;
    fs::write(path.join(LABEL),format!("{}:simulation-label",hid).as_bytes()).await?;
    fs::write(path.join(UNIT), b"--").await?;
    Ok(Driver::from(path))
}