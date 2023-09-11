pub mod config;

use config::monitors::MonitorSettings;

fn main() {
    println!("{}", MonitorSettings::default().to_keyword_string("eDP-1"));
}
