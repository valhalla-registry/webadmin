
use dioxus_logger::tracing::{info, Level};
use webadmin::app;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    dioxus::launch(app);
}