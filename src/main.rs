use log::{info};
use chrono::Local;


fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();
    info!("{:<20} {}", "starting up", Local::now());

    info!("{:<20} {}", "application stop", Local::now());
}
