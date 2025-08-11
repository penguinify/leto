use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    platform::macos::WindowBuilderExtMacOS,
    window::{Window, WindowBuilder},
};

use muda::{
    Menu, MenuId, MenuItem, PredefinedMenuItem, Submenu,
    accelerator::{Accelerator, Code, Modifiers},
};
use wry::{WebView, WebViewBuilder};

use crate::ipc::IpcMessage;

const ZOOM_FACTOR: f64 = 0.9; // Adjust zoom factor as needed

pub struct App {
    _title: String,
    _width: u32,
    _height: u32,
    event_loop: EventLoop<UserEvent>,
    zoom_factor: f64,
    window: Window,
    web_view: WebView,
    // prevent pointer issues
    menu_items: Option<Vec<MenuItem>>,
    // prevent pointer issues
    submenus: Option<Vec<Submenu>>,
    reload_menu_id: Option<MenuId>,
    devtools_menu_id: Option<MenuId>,
    _web_view_url: String,
}

#[derive(Debug, Clone)]
enum UserEvent {
    DragWindow,
    ZoomIn,
    ZoomOut,
    ShowWindow,
    MenuEvent(muda::MenuEvent),
}

impl App {
    pub fn new(title: &str, width: u32, height: u32, web_view_url: &str) -> Self {
        let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
        let event_proxy_muda = event_loop.create_proxy();
        // menu events
        muda::MenuEvent::set_event_handler(Some(move |event| {
            if let Err(e) = event_proxy_muda.send_event(UserEvent::MenuEvent(event)) {
                eprintln!("Failed to send menu event: {}", e);
            }
        }));

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
            .with_visible(false)
            .build(&event_loop)
            .expect("Failed to create window");

        window
            .set_ignore_cursor_events(false)
            .expect("Failed to set ignore cursor events");

        let event_proxy_ipc = event_loop.create_proxy();

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

                let json_message: IpcMessage =
                    serde_json::from_str(&message.body()).expect("Failed to parse IPC message");

                match json_message {
                    IpcMessage::DragWindow => {
                        if let Err(e) = event_proxy_ipc.send_event(UserEvent::DragWindow) {
                            eprintln!("Failed to send drag event: {}", e);
                        }
                    }
                    IpcMessage::ZoomIn => {
                        if let Err(e) = event_proxy_ipc.send_event(UserEvent::ZoomIn) {
                            eprintln!("Failed to send zoom in event: {}", e);
                        }
                    }
                    IpcMessage::ZoomOut => {
                        if let Err(e) = event_proxy_ipc.send_event(UserEvent::ZoomOut) {
                            eprintln!("Failed to send zoom out event: {}", e);
                        }
                    }
                    IpcMessage::ClickLink { url } => {
                        if let Err(e) = open::that(&url) {
                            eprintln!("Failed to open URL: {}", e);
                        }
                    }
                    IpcMessage::Loaded => {
                        if let Err(e) = event_proxy_ipc.send_event(UserEvent::ShowWindow) {
                            eprintln!("Failed to send loaded event: {}", e);
                        }
                    } // ts just crashes if there is a ipc message that is not handled
                }
            })
            .with_devtools(true)
            .build(&window)
            .expect("Failed to build web view");

        // realistic zoom level, once the new event loop is implemented I'll make this configurable
        web_view.zoom(ZOOM_FACTOR).unwrap();

        Self {
            _title: title.to_string(),
            _width: width,
            _height: height,
            event_loop,
            zoom_factor: ZOOM_FACTOR,
            window,
            menu_items: None,
            submenus: None,
            reload_menu_id: None,
            devtools_menu_id: None,
            web_view,
            _web_view_url: web_view_url.to_string(),
        }
    }

    // very scary function
    pub fn evaluate_script(&mut self, script: &str) -> Result<(), wry::Error> {
        self.web_view.evaluate_script(script)
    }

    pub fn add_menubar_items(&mut self) {
        //TODO: add developer tools menu item

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

        let developer_m = Submenu::new("developer", true);

        let developer_tools_menu_item = MenuItem::new(
            "Open Developer Tools",
            true,
            Some(Accelerator::new(Some(Modifiers::META), Code::KeyI)),
        );

        let reload_menu_item = MenuItem::new("Reload", true, None);

        menu.append(&developer_m).unwrap();
        developer_m
            .append_items(&[
                &developer_tools_menu_item,
                &PredefinedMenuItem::separator(),
                &reload_menu_item,
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

        self.reload_menu_id = Some(reload_menu_item.id().clone());
        self.devtools_menu_id = Some(developer_tools_menu_item.id().clone());
        self.submenus = Some(vec![about_m, developer_m]);
        self.menu_items = Some(vec![developer_tools_menu_item, reload_menu_item]);
    }
    pub fn run(mut self) {
        let Self {
            event_loop,
            window,
            web_view,
            ..
        } = self;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::WindowEvent { event, .. } => match &event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }

                    _ => {}
                },

                Event::UserEvent(UserEvent::DragWindow) => {
                    if let Err(e) = window.drag_window() {
                        eprintln!("Failed to drag window: {}", e);
                    }
                }
                Event::UserEvent(UserEvent::ZoomIn) => {
                    self.zoom_factor += 0.1;

                    if let Err(e) = web_view.zoom(self.zoom_factor) {
                        eprintln!("Failed to zoom in: {}", e);
                    }
                }
                Event::UserEvent(UserEvent::ZoomOut) => {
                    self.zoom_factor -= 0.1;

                    if let Err(e) = web_view.zoom(self.zoom_factor) {
                        eprintln!("Failed to zoom out: {}", e);
                    }
                }

                Event::UserEvent(UserEvent::ShowWindow) => window.set_visible(true),

                Event::UserEvent(UserEvent::MenuEvent(menu_event)) => {
                    if let Some(reload_id) = &self.reload_menu_id {
                        // currently reload doens't reload the scripts
                        if menu_event.id() == &reload_id {
                            if let Err(e) = web_view.reload() {
                                eprintln!("Failed to reload: {}", e);
                            }
                            return;
                        }
                    }
                    if let Some(devtools_id) = &self.devtools_menu_id {
                        if menu_event.id() == &devtools_id {
                            web_view.open_devtools();
                            return;
                        }
                    }
                    println!("Menu event: {:?}", menu_event);
                }
                _ => (),
            }
        });
    }
}
