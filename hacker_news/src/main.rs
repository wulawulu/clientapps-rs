use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use hacker_news::App;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}
