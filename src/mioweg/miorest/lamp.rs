/// Monitor hardware caontrol system.
// use super::setting;
// use async_std::prelude::*;
use async_std::fs;
use async_std::path::{ Path, PathBuf};
// use async_std::stream;
use async_std::io::Result;

// use serde_json::from_str;
// use serde::{Deserialize, Serialize};
use super::hal;
use super::digital::{self, Digital};


pub struct Lamp {
    // dout: Digital 
    // uptime: SystemTime,
    // lifetime: Duration,,
    // model: String,
    path: PathBuf,
}



impl Lamp {
    pub async fn is_on(&self) -> Result<bool> {
        self.uptime = SystemTime::now();
        self.dout.is_high().await
    }

    pub async fn set_on(&mut self ) -> Result<()> {
        self.dout.set_high().await
    }

    pub async fn off(&mut self) -> Result<()> {
        self.dout.set_low().await
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
}
pub async fn model(path: &Path) -> Result<String> {
    let model = fs::read_to_string(path.join("lamp")).await?;
    Ok(model)
}

pub async fn lamp(path:&Path)  -> Result<Lamp> {
    model(path).await?;

}

pub async fn simulate(path:&Path) -> Result<()> {

}

// pub async fn check(path:&Path) -> bool {
//    path.is_dir().await && path.join("lamp").exists().await 
// }


// pub async get() -> Result<Lamp>  {
//     let model = model(path).await?;
//     let dout  = digital::connect(&path.join("dout")).await?;
//     Ok(Lamp{
//         path:path.to_path_buf(),
//         dout: dout,
//         model: model,
//     })
// }


// pub async fn connect(path : &Path) -> Result<Lamp>


// #[derive(Serialize,Deserialize, Clone, Debug)]
// pub enum Command {
//     TurnOn,
//     TurnOff,
// }
// pub struct State {
//     pub lifetime: u64,
//     pub on: bool,
// }
// impl Default for Lamp {
//     fn default() -> Self {
//         Self {
//             uptime:SystemTime::now(),
//             on: false,
//             lifetime: Duration::from_secs(1),
//         }
//     }
// }


    // pub fn new(automata :automata) -> Lamp {
//         automata.mio()
//         Lamp{

//         }
//     }
//     pub fn turn_on( &mut self) -> State{
//          State{
//             on:true,
//             lifetime:0,

//         }
//     }
//     pub fn turn_off(&mut self) -> State{
//         State{
//             on:true,
//             lifetime:0,

//         }
//     }
//     pub fn update_lifitime(&mut self){
//         let now     = SystemTime::now();
//         match now.duration_since(self.uptime) {
//             Ok(uptime) if self.on => {
//                 self.lifetime += uptime;
//                 self.uptime    = SystemTime::now();
//             },
//             Ok(_)  => info!("UV Lamp turn off"),
//             Err(e) => warn!("UV Lamp uptime:{:}",e),
//         }
//     }
//     pub fn setup(&mut self,lifetime: u64) {
//         self.lifetime = Duration::from_secs(lifetime);
//     }
// }

// pub async fn turn_on() -> Result<(),automataError>{
//     UVLAMP.write().unwrap().turn_on();
//     Ok(())
// }

// pub async fn turn_off() -> Result<(),automataError>{
//     UVLAMP.write().unwrap().turn_off();
//     Ok(())
// }



// async fn get_lamp() -> EndpointResult {
    // let id: usize = cx.param("id").client_err()?;

    // if let Some(msg) = cx.app_data().messages().get(id) {
        // Ok(response::json(msg))
    // } else {
        // Err(StatusCode::NOT_FOUND)?
    // }
// }
