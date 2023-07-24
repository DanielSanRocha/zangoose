use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

#[derive(Clone,Data,Lens,Deserialize,Serialize)]
pub struct AppData {
    pub denoise: bool,
    pub volume: f64
}
