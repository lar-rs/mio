/// Monitor hardware caontrol system.
use async_std::prelude::*;
use async_std::fs;
use async_std::path::{ Path, PathBuf};
use async_std::stream;
use async_std::io::Result;

use serde_json::from_str;
use serde::{Deserialize, Serialize};
use super::digital::{self, Digital};


// #[derive(Serialize,Deserialize, Clone, Debug)]
// pub enum Command {
//     TurnOn,
//     TurnOff,
// }
// pub struct State {
//     pub lifetime: u64,
//     pub on: bool,
// }


pub struct Pump {
    path: PathBuf,
    dout: Digital, 
}

impl Pump{
    pub async fn is_run(&self) -> Result<bool> {
        self.dout.is_high().await
    }

    pub async fn start(&mut self ) -> Result<()> {
        self.dout.set_high().await
    }

    pub async fn stop(&mut self) -> Result<()> {
        self.dout.set_low().await
    }
}

pub async fn model(path: &Path) -> Result<String> {
    let model = fs::read_to_string(path.join("lamp")).await?;
    Ok(model)
}

pub async fn gearpump(path:&Path)  -> Result<Lamp> {
    model(path).await?;

}

pub async fn simulate(path:&Path) -> Result<()> {
    Ok(())
}