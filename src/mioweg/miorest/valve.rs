/// Monitor hardware caontrol system.
use async_std::prelude::*;
use async_std::fs;
// use async_std::path::{ Path, PathBuf};
use async_std::path::Path;
// use async_std::stream;
use async_std::io::Result;

// use serde_json::from_str;
// use serde::{Deserialize, Serialize};
use super::digital::{self, Digital};

pub struct Valve {
    // path: PathBuf,
    dout: Digital,
}

impl Valve {
    pub async fn is_open(&self) -> Result<bool> {
        self.dout.is_high().await
    }
    pub async fn is_close(&self) -> Result<bool> {
        self.dout.is_low().await
    }

    pub async fn open(&mut self ) -> Result<()> {
        self.dout.set_high().await
    }

    pub async fn close(&mut self) -> Result<()> {
        self.dout.set_low().await
    }
}

pub async fn check(path:&Path) -> bool {
   path.is_dir().await && path.join("valve").exists().await 
}
/// Valve model [ Y1,Y2]
pub async fn model(path: &Path) -> Result<String> {
     let model = fs::read_to_string(path.join("valve")).await?;
     Ok(model)
}

pub async fn connect(path : &Path) -> Result<Valve> {
    // let model = model(path).await?;
    let dout  = digital::connect(&path.join("dout")).await?;
    Ok(Valve{
        // path:path.to_path_buf(),
        dout: dout,
    })
}

pub async fn valves(path:&Path) -> Result<Vec<Valve>> {
 // let mut entries = fs::read_dir`(".").filter(|entry| future::ready(entry.file_name().as_bytes().starts_with(b"hwmon")))
    // let sensors: Vec<NdirSensor> = s.collect().await;
    // let mut entries = fs::read_dir(path).await.unwrap();
    // let sensors = entries.filter_map()
    let mut valves: Vec<Valve> = Vec::new();
    let mut dir = fs::read_dir(path).await?;

    while let Some(res) = dir.next().await {
        let path = res?.path();
        if path.is_dir().await {
            if  check(&path).await {
                valves.push(connect(&path).await?);
            }
        }
    }
    Ok(valves)
}


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
