pub mod config;

fn main() {
    println!("{}", config::MonitorSettings::default().to_keyword_string("eDP-1"));
}
