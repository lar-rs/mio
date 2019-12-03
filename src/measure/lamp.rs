/// Monitor hardware caontrol system.
use serde_derive::{Deserialize, Serialize};
use std::time::{Duration,SystemTime};
use log::{info,warn};
use lazy_static::lazy_static;
use crate::WqaError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]



lazy_static!{
      static ref UVLAMP: RwLock<Lamp> = RwLock::new(Lamp::default());
}

pub struct Lamp {
    pub uptime: SystemTime,
    pub lifetime: Duration,
    pub on: bool,
}


impl Default for Lamp {
    fn default() -> Self {
        Self {
            uptime:SystemTime::now(),
            on: false,
            lifetime: Duration::from_secs(1),
        }
    }
}

impl Lamp {

    pub fn turn_on( &mut self) {
        self.on     = true;
        self.uptime = SystemTime::now();
    }
    pub fn turn_off(&mut self) {
        self.update_lifitime();
        self.on     = false;
    }
    pub fn update_lifitime(&mut self){
        let now     = SystemTime::now();
        match now.duration_since(self.uptime) {
            Ok(uptime) if self.on => {
                self.lifetime += uptime;
                self.uptime    = SystemTime::now();
            },
            Ok(_)  => info!("UV Lamp turn off"),
            Err(e) => warn!("UV Lamp uptime:{:}",e),
        }
    }
    pub fn setup(&mut self,lifetime: u64) {
        self.lifetime = Duration::from_secs(lifetime);
    }
}

pub async fn set_lifetime(lifetime: u64) {
    UVLAMP.write().unwrap().set_lifetime(lifetime);
}

async fn status() -> Result<Lamp,WqaError> {
    Ok(Lamp::default())
}

pub async fn lamp_status() -> Result<Lamp,WqaError> {
    Ok(UVLAMP.read().unwrap().clone())
}

pub async fn lamp_turn_on() -> Result<(),WqaError>{
    UVLAMP.write().unwrap().turn_on();
    Ok(())
}

pub async fn lamp_turn_off() -> Result<(),WqaError>{
    UVLAMP.write().unwrap().turn_off();
    Ok(())
}



// async fn get_lamp() -> EndpointResult {
    // let id: usize = cx.param("id").client_err()?;

    // if let Some(msg) = cx.app_data().messages().get(id) {
        // Ok(response::json(msg))
    // } else {
        // Err(StatusCode::NOT_FOUND)?
    // }
// }
