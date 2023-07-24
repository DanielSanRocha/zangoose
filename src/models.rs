use druid::{Data, Lens};

#[derive(Clone,Data,Lens)]
pub struct AppData {
    pub denoise: bool,
    pub frac: f64,
    pub volume: f64,
    pub drive: f64
}
