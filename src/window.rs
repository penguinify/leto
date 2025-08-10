use tao::{
    event_loop::{ControlFlow, EventLoop},
    platform::macos::WindowBuilderExtMacOS,
    window::{Window, WindowBuilder},
};

use muda::accelerator::{Accelerator, Code, Modifiers};
use muda::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use wry::{WebView, WebViewBuilder};

const ZOOM_FACTOR: f64 = 0.9; // Adjust zoom factor as needed

pub struct App {
    _title: String,
    _width: u32,
    _height: u32,
    event_loop: EventLoop<()>,
    window: Window,
    web_view: WebView,
    _web_view_url: String,
}

impl App {
    pub fn new(title: &str, width: u32, height: u32, web_view_url: &str) -> Self {
        let event_loop = EventLoop::new();
        let event_proxy = event_loop.create_proxy();
        #[cfg(target_os = "macos")]
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(tao::dpi::LogicalSize::new(width, height))
            .with_transparent(false)
            .with_resizable(true)
            .with_fullsize_content_view(true)
            .with_titlebar_transparent(true)
            .with_title_hidden(true)
            .with_background_color(tao::window::RGBA::from((40, 43, 48, 255)))
            .with_min_inner_size(tao::dpi::LogicalSize::new(1200.0, 720.0))
            .build(&event_loop)
            .expect("Failed to create window");

        window
            .set_ignore_cursor_events(false)
            .expect("Failed to set ignore cursor events");

        // Taken from Lemoncord
        #[cfg(target_os = "macos")]
        let user_agent: String = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15".to_string();

        #[cfg(target_os = "macos")]
        let web_view = WebViewBuilder::new()
            .with_url(web_view_url)
            .with_user_agent(user_agent)
            .with_background_color(tao::window::RGBA::from((40, 43, 48, 255)))
            .with_ipc_handler(move |message| {
                println!("Received IPC message: {}", message.body());
                match message.body().as_str() {
                    "drag_window" => {
                        if let Err(e) = event_proxy.send_event(()) {
                            eprintln!("Failed to send drag event: {}", e);
                        }
                    }
                    _ => {
                        println!("Received unknown IPC message: {}", message.body());
                    }
                }
            })
            .with_devtools(true)
            .build(&window)
            .expect("Failed to build web view");

        // realistic zoom level, magic number woooo
        web_view.zoom(ZOOM_FACTOR).unwrap();

        App::add_menubar_items();

        Self {
            _title: title.to_string(),
            _width: width,
            _height: height,
            event_loop,
            window,
            web_view,
            _web_view_url: web_view_url.to_string(),
        }
    }

    // very scary function
    pub fn evaluate_script(&mut self, script: &str) -> Result<(), wry::Error> {
        self.web_view.evaluate_script(script)
    }

    pub fn add_menubar_items() {
        //TODO: add developer tools menu item and update the event loop for custom events

        let menu = Menu::new();

        let about_m = Submenu::new("leto", true);
        menu.append(&about_m).unwrap();
        about_m
            .append_items(&[
                &PredefinedMenuItem::hide(None),
                &PredefinedMenuItem::hide_others(None),
                &PredefinedMenuItem::show_all(None),
                &PredefinedMenuItem::separator(),
                &PredefinedMenuItem::quit(None),
            ])
            .unwrap();

        #[cfg(target_os = "windows")]
        unsafe {
            about_m.init_for_hwnd(window_hwnd)
        };
        #[cfg(target_os = "linux")]
        about_m.init_for_gtk_window(&gtk_window, Some(&vertical_gtk_box));
        #[cfg(target_os = "macos")]
        menu.init_for_nsapp();
    }
    pub fn run(self) {
        let Self {
            event_loop,
            window,
            web_view,
            ..
        } = self;

        event_loop.run(move |event, _, control_flow| match event {
            tao::event::Event::WindowEvent { event, .. } => match &event {
                tao::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }

                _ => {}
            },
            tao::event::Event::RedrawRequested(_) => {
                web_view.reload().unwrap();
            }

            //TODO: I seriously need to update the event handler to support custom events
            tao::event::Event::UserEvent(()) => {
                if let Err(e) = window.drag_window() {
                    eprintln!("Failed to drag window: {}", e);
                }
            }
            _ => (),
        });
    }
}
