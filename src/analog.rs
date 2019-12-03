///
/// Analog IN OUT
///
///
///
use async_std::io::Result;
// use async_std::prelude::*;
use async_trait::async_trait;

/// Analog input
#[async_trait]
pub trait Input{
    type Value;
    async fn get(&mut self)->Result<Self::Value>;
    // pub async fn (&mut self)->nb::Result<Self::Value,Self::Error>;
}


/// Analog input
#[async_trait]
pub trait Output{
    type Value;
    async fn set(&mut self,v:Self::Value)->Result<()>;
    // pub async fn (&mut self)->nb::Result<Self::Value,Self::Error>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnalogIn{
    value: f32,
}


#[async_trait]
impl Input for AnalogIn {
    type Value = f32;
    async fn get(&mut self)->Result<Self::Value> {
        Ok(self.value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnalogOut{
    value: f32,
}

#[async_trait]
impl Input for AnalogOut {
    type Value = f32;
    async fn get(&mut self)->Result<Self::Value> {
        Ok(self.value)
    }
}

#[async_trait]
impl Output for AnalogOut {
    type Value = f32;
    async fn set(&mut self,value:Self::Value)->Result<()> {
        self.value = value;
        Ok(())
    }
}

