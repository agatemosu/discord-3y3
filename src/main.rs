mod app;
mod util;

use app::App;

fn main() {
	yew::Renderer::<App>::new().render();
}
