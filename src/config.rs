use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitorIdentifier {
    pub make: String,
    pub model: String,
    pub serial: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitorSettings {
    pub resolution: MonitorResolution,
    pub position: MonitorPosition,
    pub transform: MonitorTransform,
    pub scale: f32,
    pub bit_depth: u32,
    pub vrr: MonitorVrrMode,
    pub mirror: Option<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum MonitorResolution {
    Preferred,
    Manual {
        width: usize,
        height: usize,
        refresh: Option<f32>,
    },
    HighResolution,
    HighRefresh,
    Modeline(String),
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MonitorTransform {
    Normal = 0,
    Degrees90 = 1,
    Degrees180 = 2,
    Degrees270 = 3,
    Flipped = 4,
    Degrees90Flipped = 5,
    Degrees180Flipped = 6,
    Degrees270Flipped = 7,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum MonitorPosition {
    Automatic,
    Manual { x: isize, y: isize },
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MonitorVrrMode {
    Default = 255,
    Off = 0,
    On = 1,
    Fullscreen = 2,
}
