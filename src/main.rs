use tao::{event_loop::EventLoop, window::WindowBuilder};

use wry::WebViewBuilder;

mod window;

const TITLE: &str = "Leto";
const WEB_VIEW_URL: &str = "https://discord.com/app";

fn main() {
    let app = window::App::new(TITLE, 800, 600, WEB_VIEW_URL);
    app.run();
}
