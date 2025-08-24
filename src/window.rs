use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    platform::macos::WindowBuilderExtMacOS,
    window::{Window, WindowBuilder},
};

// use rsrpc::detection::DetectableActivity;
#[cfg(target_os = "macos")]
use muda::{
    Menu, MenuId, MenuItem, PredefinedMenuItem, Submenu,
    accelerator::{Accelerator, Code, Modifiers},
};
use wry::{WebView, WebViewBuilder};

use crate::ipc::IpcMessage;
use crate::microphone;
use crate::scripts::get_pre_inject_script;

use tracing::{error, info};

pub struct App {
    _title: String,
    _width: u32,
    _height: u32,
    event_loop: EventLoop<UserEvent>,
    window: Window,
    web_view: WebView,
    // prevent pointer issues
    menu_items: Option<Vec<MenuItem>>,
    // prevent pointer issues
    submenus: Option<Vec<Submenu>>,
    reload_menu_id: Option<MenuId>,
    devtools_menu_id: Option<MenuId>,
    request_mid_permission_id: Option<MenuId>,
    _web_view_url: String,
}

#[derive(Debug, Clone)]
enum UserEvent {
    DragWindow,
    Zoom(f64),
    MenuEvent(muda::MenuEvent),
}

impl App {
    pub fn new(title: &str, width: u32, height: u32, web_view_url: &str) -> Self {
        info!(
            "Initializing App with title: {}, size: {}x{}, url: {}",
            title, width, height, web_view_url
        );
        let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
        let event_proxy_muda = event_loop.create_proxy();

        // menu events
        muda::MenuEvent::set_event_handler(Some(move |event| {
            if let Err(e) = event_proxy_muda.send_event(UserEvent::MenuEvent(event)) {
                error!("Failed to send menu event: {}", e);
            } else {
                info!("Sent menu event");
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
            .with_min_inner_size(tao::dpi::LogicalSize::new(936.0, 720.0))
            .build(&event_loop)
            .expect("Failed to create window");
        info!("Window created");

        if let Err(e) = window.set_ignore_cursor_events(false) {
            error!("Failed to set ignore cursor events: {}", e);
        } else {
            info!("Set ignore cursor events to false");
        }

        let event_proxy_ipc = event_loop.create_proxy();

        #[cfg(target_os = "macos")]
        let user_agent: String = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15".to_string();

        #[cfg(target_os = "macos")]
        let web_view = WebViewBuilder::new()
            .with_url(web_view_url)
            .with_user_agent(user_agent)
            .with_background_color(tao::window::RGBA::from((40, 43, 48, 255)))
            .with_ipc_handler(move |message| {
                info!("Received IPC message: {}", message.body());

                let json_message: IpcMessage = match serde_json::from_str(&message.body()) {
                    Ok(msg) => msg,
                    Err(e) => {
                        error!("Failed to parse IPC message: {}", e);
                        return;
                    }
                };

                match json_message {
                    IpcMessage::DragWindow => {
                        if let Err(e) = event_proxy_ipc.send_event(UserEvent::DragWindow) {
                            error!("Failed to send drag event: {}", e);
                        } else {
                            info!("DragWindow event sent");
                        }
                    }
                    IpcMessage::Zoom { level } => {
                        if let Err(e) = event_proxy_ipc.send_event(UserEvent::Zoom(level)) {
                            error!("Failed to send zoom event: {}", e);
                        } else {
                            info!("Zoom event sent with level {}", level);
                        }
                    }
                }
            })
            .with_devtools(true)
            .with_new_window_req_handler(move |url| {
                info!("New window request for URL: {}", url);

                // makes sure it isn't an embed
                if url.contains("embed") {
                    info!("Ignoring embed URL: {}", url);
                    return false;
                }

                let _ = open::that(&url);
                false
            })
            .with_initialization_script(get_pre_inject_script())
            .build(&window)
            .expect("Failed to build web view");
        info!("WebView built");

        Self {
            _title: title.to_string(),
            _width: width,
            _height: height,
            event_loop,
            window,
            menu_items: None,
            submenus: None,
            request_mid_permission_id: None,
            reload_menu_id: None,
            devtools_menu_id: None,
            web_view,
            _web_view_url: web_view_url.to_string(),
        }
    }

    pub fn evaluate_script(&mut self, script: &str) -> Result<(), wry::Error> {
        info!("Evaluating script");
        self.web_view.evaluate_script(script)
    }

    pub fn add_menubar_items(&mut self) {
        info!("Adding menubar items");

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

        let edit_m = Submenu::new("edit", true);
        menu.append(&edit_m).unwrap();

        edit_m
            .append_items(&[
                &PredefinedMenuItem::undo(None),
                &PredefinedMenuItem::redo(None),
                &PredefinedMenuItem::separator(),
                &PredefinedMenuItem::cut(None),
                &PredefinedMenuItem::copy(None),
                &PredefinedMenuItem::paste(None),
                &PredefinedMenuItem::select_all(None),
            ])
            .unwrap();

        let window_m = Submenu::new("window", true);
        menu.append(&window_m).unwrap();

        window_m
            .append_items(&[
                &PredefinedMenuItem::minimize(None),
                &PredefinedMenuItem::separator(),
                &PredefinedMenuItem::bring_all_to_front(None),
            ])
            .unwrap();

        let developer_m = Submenu::new("developer", true);

        let developer_tools_menu_item = MenuItem::new(
            "Open Developer Tools",
            true,
            Some(Accelerator::new(Some(Modifiers::META), Code::KeyI)),
        );

        let reload_menu_item = MenuItem::new("Reload", true, None);

        let request_mic_permission_item =
            MenuItem::new("Request Microphone Permission", true, None);

        menu.append(&developer_m).unwrap();
        developer_m
            .append_items(&[
                &developer_tools_menu_item,
                &PredefinedMenuItem::separator(),
                &reload_menu_item,
                &request_mic_permission_item,
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
        window_m.set_as_windows_menu_for_nsapp();

        self.reload_menu_id = Some(reload_menu_item.id().clone());
        self.devtools_menu_id = Some(developer_tools_menu_item.id().clone());
        self.request_mid_permission_id = Some(request_mic_permission_item.id().clone());
        self.submenus = Some(vec![about_m, developer_m]);
        self.menu_items = Some(vec![
            developer_tools_menu_item,
            reload_menu_item,
            request_mic_permission_item,
        ]);

        info!("Menubar items added");
    }

    pub fn run(self) {
        info!("App::run - Starting event loop");
        let Self {
            event_loop,
            window,
            web_view,
            reload_menu_id,
            devtools_menu_id,
            request_mid_permission_id,
            ..
        } = self;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::WindowEvent { event, .. } => {
                    if let WindowEvent::CloseRequested = &event {
                        info!("App::run - Window close requested, exiting");
                        *control_flow = ControlFlow::Exit;
                    }
                }
                Event::UserEvent(user_event) => {
                    Self::handle_user_event(
                        user_event,
                        &window,
                        &web_view,
                        reload_menu_id.as_ref(),
                        devtools_menu_id.as_ref(),
                        request_mid_permission_id.as_ref(),
                    );
                }
                _ => {}
            }
        });
    }

    fn handle_user_event(
        user_event: UserEvent,
        window: &Window,
        web_view: &WebView,
        reload_menu_id: Option<&MenuId>,
        devtools_menu_id: Option<&MenuId>,
        request_mid_permission_id: Option<&MenuId>,
    ) {
        match user_event {
            UserEvent::DragWindow => {
                info!("App::run - Handling DragWindow event");
                if let Err(e) = window.drag_window() {
                    error!("App::run - Failed to drag window: {}", e);
                }
            }
            UserEvent::Zoom(level) => {
                info!("App::run - Handling Zoom event, level: {}", level);
                if let Err(e) = web_view.zoom(level) {
                    error!("App::run - Failed to zoom: {}", e);
                }
            }
            UserEvent::MenuEvent(menu_event) => {
                info!("App::run - Handling MenuEvent: {:?}", menu_event);

                if reload_menu_id.map_or(false, |id| menu_event.id() == id) {
                    info!("App::run - Reload menu selected, reloading web view");
                    if let Err(e) = web_view.reload() {
                        error!("App::run - Failed to reload web view: {}", e);
                    }
                    return;
                }
                if devtools_menu_id.map_or(false, |id| menu_event.id() == id) {
                    info!("App::run - Developer tools menu selected, opening devtools");
                    web_view.open_devtools();
                    return;
                }
                if request_mid_permission_id.map_or(false, |id| menu_event.id() == id) {
                    info!("App::run - Request microphone permission menu selected");
                    #[cfg(target_os = "macos")]
                    {
                        if let Ok(mut input_audio) = microphone::InputAudio::new() {
                            if let Err(e) = input_audio.start_stream() {
                                error!("App::run - Failed to start microphone stream: {}", e);
                            }
                        } else {
                            error!("App::run - Failed to request microphone permission");
                        }
                    }
                }
            }
        }
    }
}
