// pub use uom::si::f32::{Ratio};


// use crate::sys;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Level{
    Normal,
    Warning,
    Critical,
    Brocket
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Settings {
    pub warn_level: f32,
    pub crit_level: f32,
    pub interval: u64,
    pub scale: f32,
}
use std::convert::TryFrom;
/// interface transfer

impl TryFrom<Interface> for Humidity {
    type Error = Error;
    fn try_from(iface: Interface) -> Result<Self> {
        iface.set_itype(IType::Humidity)?;
        Ok(Self{
            path:iface.path,
        })
    }
}
/// Hardware airflow sensor.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Humidity {
    path: PathBuf
}


impl Humidity {
    pub fn settings(&self) -> io::Result<Settings> {
       let tomlstr = fs::read_to_string(self.path.join("settings.toml"))?; 

    }
    pub fn select(path:&Path) ->  io::Result<Humidity> {
        let value = fs::read_to_string(path.join("value"))?.parse::<f32>().unwrap_or(0.0);
        let critical = fs::read_to_string(path.join("level"))?.parse::<f32>().unwrap_or(0.0);


    }
    pub fn from_analog16(value: u16) -> Humidity {
        let signal =  value as f32 / 4096.0 * 5.0;
        Humidity::from_voltage(signal)
    }
    pub fn from_voltage(voltage:f32) -> Humidity {
        let broken = voltage < 0.8 * 4.0 / 5.0;
        let value  = ((voltage - 0.8)  / (3.6 - 0.8))*100.0;
        Humidity {
            updated:   Utc::now().timestamp_millis() as u64,
            value:     value,
            broken:    broken,
        }
    }
}

pub fn select() -> PathBuf {
    let path = super::workdir().join("/humidity/");
    // if !path.exists() {
        // fs::DirBuilder::new()
            // .recursive(true)
            // .create(path.as_path())
            // ?;
        // info!("{:} new creat", Paint::cyan("MIO:airflow"));
    //
    path
}

pub fn read_config() -> io::Result<Config>{
    let path = workdir().join("config.toml");
    let mut file = fs::File::open(path.as_path())?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let config: Config = from_slice(buf.as_slice())?;
    Ok(config)
}

pub fn write_config(config:&Config) -> io::Result<()> {
    let path = workdir().join("config");
    let mut file = fs::File::create(path.as_path())?;

    // airflow.write(path)?;
    Ok(())
}


pub fn signal(config:&Config) -> io::Result<Humidity> {
    let path = workdir().join("signal");
    let mut file = fs::File::open(path.as_path())?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let humidity:Humidity = from_slice(buf.as_slice())?;
    Ok(humidity)
}
