use tao::{
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use wry::{WebView, WebViewBuilder};

pub struct App {
    title: String,
    width: u32,
    height: u32,
    event_loop: EventLoop<()>,
    window: Window,
    web_view: WebView,
    web_view_url: String,
}

impl App {
    pub fn new(title: &str, width: u32, height: u32, web_view_url: &str) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(tao::dpi::LogicalSize::new(width, height))
            .with_transparent(true)
            .with_resizable(true)
            .build(&event_loop)
            .expect("Failed to create window");

        // Taken from Lemoncord
        #[cfg(target_os = "macos")]
        let user_agent: String = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15".to_string();

        let web_view = WebViewBuilder::new()
            .with_url(web_view_url)
            .with_user_agent(user_agent)
            .build(&window)
            .expect("Failed to build web view");

        Self {
            title: title.to_string(),
            width,
            height,
            event_loop,
            window,
            web_view,
            web_view_url: web_view_url.to_string(),
        }
    }

    pub fn run(self) {
        let Self {
            event_loop,
            window,
            mut web_view,
            ..
        } = self;

        event_loop.run(move |event, _, control_flow| match event {
            tao::event::Event::WindowEvent { event, .. } => {
                if event == tao::event::WindowEvent::CloseRequested {
                    *control_flow = ControlFlow::Exit;
                }
            }
            tao::event::Event::RedrawRequested(_) => {
                web_view.reload().unwrap();
            }
            _ => (),
        });
    }
}
