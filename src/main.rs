mod app;

use app::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

fn main() {
	dioxus_logger::init(Level::INFO).expect("failed to init logger");
	dioxus::launch(|| {
		rsx! {
			Router::<Route> {}
		}
	});
}
