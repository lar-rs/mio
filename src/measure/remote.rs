/// Remote signal
///
#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct RemoteState {
    pub signal: bool,
    pub remote: bool
}
/// Analog
#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct Analog {
    pub enable : bool,
    pub curent: u32,
}


//


