use std::collections::HashMap;

use hlwsp::config::monitors::{
    MonitorMode, MonitorModeFlag, MonitorPosition, MonitorResolution, MonitorSettings,
    MonitorTransform,
};
use hlwsp::config::{ConfigProfile, MonitorIdentifier};

fn main() {
    let profile = ConfigProfile {
        profile_name: "Desk With 3 Monitors".to_owned(),
        monitors: {
            let mut map = HashMap::new();
            map.insert(
                "eDP-1".to_owned(),
                MonitorIdentifier {
                    make: "AU Optronics".to_owned(),
                    model: "0x403D".to_owned(),
                    serial: "".to_owned(),
                },
            );
            map.insert(
                "DP-1".to_owned(),
                MonitorIdentifier {
                    make: "GWD".to_owned(),
                    model: "ARZOPA".to_owned(),
                    serial: "000000000000".to_owned(),
                },
            );
            map.insert(
                "HDMI-A-1".to_owned(),
                MonitorIdentifier {
                    make: "Ancor Communications Inc".to_owned(),
                    model: "ASUS VH238".to_owned(),
                    serial: "B6LMIZ000302".to_owned(),
                },
            );
            map
        },
        monitor_settings: {
            let mut map = HashMap::new();
            map.insert(
                "eDP-1".to_owned(),
                MonitorSettings {
                    resolution: MonitorResolution::HighRefresh,
                    position: MonitorPosition::Manual { x: 1920, y: 1080 },
                    ..Default::default()
                },
            );
            map.insert(
                "DP-1".to_owned(),
                MonitorSettings {
                    resolution: MonitorResolution::Manual {
                        width: 1920,
                        height: 1080,
                        refresh: None,
                    },
                    position: MonitorPosition::Manual { x: 1920, y: 0 },
                    transform: MonitorTransform::Degrees90Flipped,
                    ..Default::default()
                },
            );
            map.insert(
                "HDMI-A-1".to_owned(),
                MonitorSettings {
                    resolution: MonitorResolution::Modeline(MonitorMode {
                        clock: 172.80,
                        hdisplay: 1920,
                        hsync_start: 2040,
                        hsync_end: 2248,
                        htotal: 2576,
                        vdisplay: 1080,
                        vsync_start: 1081,
                        vsync_end: 1084,
                        vtotal: 1118,
                        flags: vec![MonitorModeFlag::NHSync, MonitorModeFlag::PVSync],
                    }),
                    position: MonitorPosition::Manual { x: 0, y: 1080 },
                    ..Default::default()
                },
            );
            map
        },
    };

    let profile_json = serde_json::to_string_pretty(&profile).unwrap();
    std::fs::write("out/profile-0.json", profile_json).unwrap();
}
