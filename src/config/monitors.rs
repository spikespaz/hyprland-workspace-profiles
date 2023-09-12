use std::fmt;

use serde::{Deserialize, Serialize};

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
    Modeline(MonitorMode),
}

/// Copied from Mesa's [`xf86drmMode.h#L90-100`] and
/// cross-referenced with Hyprland's [`ConfigManager.cpp#L530-L538`].
///
/// For a high-level overview, see Wikipedia:
/// <https://en.wikipedia.org/wiki/XFree86_Modeline>
///
/// There is also a calculator (and explanation) by Paul Lutus:
/// <https://arachnoid.com/modelines/>
///
/// [`xf86drmMode.h#L90-100`]: https://gitlab.freedesktop.org/mesa/drm/-/blob/8d0fb9b3f225183fb3276a0e4ae1f8354a3519e8/xf86drmMode.h#L90-100
/// [`ConfigManager.cpp#L530-L538`]: https://github.com/hyprwm/Hyprland/blob/ed51fe7bac76248933c71d3958fdb9a973066dfd/src/config/ConfigManager.cpp#L530-L538
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MonitorMode {
    pub clock: f32,
    pub hdisplay: u16,
    pub hsync_start: u16,
    pub hsync_end: u16,
    pub htotal: u16,
    pub vdisplay: u16,
    pub vsync_start: u16,
    pub vsync_end: u16,
    pub vtotal: u16,
    pub flags: Vec<MonitorModeFlag>,
}

/// Copied from Mesa's [`drm_mode.h#L71-84`] and
/// cross-referenced with Hyprland's [`ConfigManager.cpp#L542-L545`].
///
/// [`drm_mode.h#L71-84`]: https://gitlab.freedesktop.org/mesa/drm/-/blob/8d0fb9b3f225183fb3276a0e4ae1f8354a3519e8/include/drm/drm_mode.h#L71-84
/// [`ConfigManager.cpp#L542-L545`]: https://github.com/hyprwm/Hyprland/blob/ed51fe7bac76248933c71d3958fdb9a973066dfd/src/config/ConfigManager.cpp#L542-L545
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MonitorModeFlag {
    PHSync,
    NHSync,
    PVSync,
    NVSync,
    // Hyprland doesn't support the following flags.
    // Interlace,
    // DoubleScan,
    // CSync,
    // PCSync,
    // NCSync,
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

impl Default for MonitorSettings {
    fn default() -> Self {
        Self {
            resolution: MonitorResolution::Preferred,
            position: MonitorPosition::Automatic,
            transform: MonitorTransform::Normal,
            scale: 1.0,
            bit_depth: 8,
            vrr: MonitorVrrMode::Default,
            mirror: None,
        }
    }
}

impl MonitorSettings {
    pub fn to_keyword_string<N: AsRef<str>>(&self, name: N) -> String {
        use std::fmt::Write;

        let mut buf = String::new();

        write!(buf, "{}", name.as_ref()).unwrap();

        buf.push(',');
        use MonitorResolution as Mr;
        match &self.resolution {
            Mr::Preferred => write!(buf, "preferred"),
            Mr::Manual {
                width,
                height,
                refresh: None,
            } => write!(buf, "{width}x{height}"),
            Mr::Manual {
                width,
                height,
                refresh: Some(refresh),
            } => write!(buf, "{width}x{height}@{refresh}"),
            Mr::HighResolution => write!(buf, "highres"),
            Mr::HighRefresh => write!(buf, "highrr"),
            Mr::Modeline(mode) => write!(buf, "modeline {}", mode.to_string()),
        }
        .unwrap();

        buf.push(',');
        use MonitorPosition as Mp;
        match &self.position {
            Mp::Automatic => write!(buf, "auto"),
            Mp::Manual { x, y } => write!(buf, "{x}x{y}"),
        }
        .unwrap();

        buf.push(',');
        write!(buf, "{}", self.scale).unwrap();

        buf.push(',');
        write!(buf, "transform,{}", self.transform as u32).unwrap();

        buf.push(',');
        write!(buf, "bitdepth,{}", self.bit_depth).unwrap();

        if self.vrr != MonitorVrrMode::Default {
            buf.push(',');
            write!(buf, "vrr,{}", self.vrr as u32).unwrap();
        }

        if let Some(name) = &self.mirror {
            buf.push(',');
            write!(buf, "mirror,{}", name).unwrap()
        }

        buf
    }
}

// Not `Display` because that may warrant a different representation.
impl ToString for MonitorMode {
    fn to_string(&self) -> String {
        use std::fmt::Write;

        let mut buf = String::new();

        write!(
            buf,
            "{} {} {} {} {} {} {} {} {}",
            self.clock,
            self.hdisplay,
            self.hsync_start,
            self.hsync_end,
            self.htotal,
            self.vdisplay,
            self.vsync_start,
            self.vsync_end,
            self.vtotal,
        )
        .unwrap();

        for flag in &self.flags {
            write!(buf, " {}", flag).unwrap();
        }

        buf
    }
}

impl fmt::Display for MonitorModeFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PHSync => formatter.write_str("+hsync"),
            Self::NHSync => formatter.write_str("-hsync"),
            Self::PVSync => formatter.write_str("+vsync"),
            Self::NVSync => formatter.write_str("-vsync"),
        }
    }
}
