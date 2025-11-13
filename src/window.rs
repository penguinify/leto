use std::error::Error;

use http::HeaderMap;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    platform::macos::WindowBuilderExtMacOS,
    window::{Window, WindowBuilder},
};
use tracing::event;
use tracing::{Instrument, instrument};

// use rsrpc::detection::DetectableActivity;
#[cfg(target_os = "macos")]
use muda::{
    Menu, MenuId, MenuItem, PredefinedMenuItem, Submenu,
    accelerator::{Accelerator, Code, Modifiers},
};
use wry::{WebView, WebViewBuilder};

use crate::injection::scripts::get_pre_inject_script;
use crate::ipc::FetchOptions;
use crate::ipc::IpcMessage;
use serde_json::json;
use tokio::runtime::Runtime;
use tracing::{error, info};
pub struct App {
    _title: String,
    event_loop: EventLoop<UserEvent>,
    window: Window,
    web_view: WebView,
    // prevent pointer issues
    menu_items: Option<Vec<MenuItem>>,
    // prevent pointer issues
    submenus: Option<Vec<Submenu>>,
    reload_menu_id: Option<MenuId>,
    devtools_menu_id: Option<MenuId>,
    _web_view_url: String,
    runtime: tokio::runtime::Runtime,
}

#[derive(Debug, Clone)]
enum UserEvent {
    DragWindow,
    Zoom(f64),
    MenuEvent(muda::MenuEvent),
    IpcFetchResult {
        req_id: String,
        result: Result<serde_json::Value, serde_json::Value>,
    },
}

impl App {
    pub fn new(title: &str, web_view_url: &str) -> Self {
        info!(
            "Initializing App with title: {}, url: {}",
            title, web_view_url
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

        let runtime = Runtime::new().unwrap();
        let rt_handle = runtime.handle().clone();

        #[cfg(target_os = "macos")]
            let user_agent: String = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15".to_string();
        let headers: HeaderMap = {
            let mut headers = HeaderMap::new();
            headers.insert("content-security-policy", "default-src * 'unsafe-inline' 'unsafe-eval' data: blob:; img-src * data: blob:; media-src * data: blob:; script-src * 'unsafe-inline' 'unsafe-eval' data: blob:; style-src * 'unsafe-inline' data: blob:; font-src * data: blob:; connect-src * wss: ws:; frame-src * data: blob:;".parse().unwrap());
            headers
        };

        #[cfg(target_os = "macos")]
        let web_view = WebViewBuilder::new()
            .with_headers(headers)
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
                    IpcMessage::Fetch {
                        url,
                        options,
                        req_id,
                    } => {
                        let rt_handle = rt_handle.clone();
                        let event_proxy = event_proxy_ipc.clone();
                        let url = url.clone();
                        let options: FetchOptions = options.clone();
                        let req_id = req_id.clone();

                        rt_handle.spawn(async move {
                            fetch_handler(url, options, req_id, event_proxy).await;
                        });
                    }
                }
            })
            .with_devtools(true)
            .with_new_window_req_handler(move |url, window| {
                info!("New window request for URL: {}", url);
                // checks if the webview is requesting to open inside the app
                // code is "safe" since its just running objc bindings for NSView
                // and icl I don't think this is working anyways, but it doesn't matter for now
                unsafe {
                    if window.opener.webview.isHiddenOrHasHiddenAncestor() {
                        info!("Allowing new window inside the app");
                        return wry::NewWindowResponse::Allow;
                    }
                }
                open::that(url).unwrap_or_else(|e| {
                    error!("Failed to open URL in default browser: {}", e);
                });
                wry::NewWindowResponse::Deny
            })
            .with_initialization_script(get_pre_inject_script())
            .build(&window)
            .expect("Failed to build web view");
        info!("WebView built");

        Self {
            _title: title.to_string(),
            event_loop,
            window,
            menu_items: None,
            submenus: None,
            reload_menu_id: None,
            devtools_menu_id: None,
            web_view,
            _web_view_url: web_view_url.to_string(),
            runtime,
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
                &PredefinedMenuItem::close_window(None),
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
        window_m.set_as_windows_menu_for_nsapp();

        self.reload_menu_id = Some(reload_menu_item.id().clone());
        self.devtools_menu_id = Some(developer_tools_menu_item.id().clone());
        self.submenus = Some(vec![about_m, developer_m]);
        self.menu_items = Some(vec![developer_tools_menu_item, reload_menu_item]);

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
            }
            UserEvent::IpcFetchResult { req_id, result } => {
                info!("App::run - Handling IpcFetchResult for req_id: {}", req_id);
                let script = match result {
                    Ok(data) => format!(
                        "window.__LETO__.handleFetchResponse('{}', true, {});",
                        req_id,
                        serde_json::to_string(&data).unwrap()
                    ),
                    Err(data) => format!(
                        "window.__LETO__.handleFetchResponse('{}', false, {});",
                        req_id,
                        serde_json::to_string(&data).unwrap()
                    ),
                };
                let log_script = format!(
                    "window.__LETO__.logMessage('Fetch response for {} being returned.');",
                    req_id
                );
                let _ = web_view.evaluate_script(&log_script);
                let _ = web_view.evaluate_script(&script);
            }
        }
    }
}

#[instrument]
async fn fetch_handler(
    url: String,
    options: FetchOptions,
    req_id: String,
    event_proxy: tao::event_loop::EventLoopProxy<UserEvent>,
) {
    // --- Inside the fetch_task span ---

    // Log the start of the task
    tracing::info!("Starting HTTP request"); // <-- **Start Log**

    let client = reqwest::Client::new();

    // Determine request method and log it
    let method = options.method.to_uppercase();
    tracing::debug!("Request method determined: {}", method); // <-- **Method Log**

    let mut req_builder = match method.as_str() {
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        _ => client.get(&url),
    };

    // ... Header and body setup code remains the same ...
    if let Some(headers) = options.headers
        && let Some(headers_map) = headers.as_object()
    {
        // NOTE: Consider logging the number of headers added, not the headers themselves,
        // as they might contain sensitive info.
        tracing::trace!("Adding {} headers to request", headers_map.len());
        for (k, v) in headers_map {
            if let Some(v_str) = v.as_str() {
                req_builder = req_builder.header(k, v_str);
            }
        }
    }

    if options.body.is_some() {
        tracing::trace!("Adding body to request");
        req_builder = req_builder.body(options.body.unwrap());
    }

    // Log the actual send attempt
    tracing::info!("Sending request..."); // <-- **Pre-send Log**
    let response = req_builder.send().await;

    let result = match response {
        Ok(resp) => {
            let status = resp.status().as_u16();
            // Log the successful response status
            tracing::info!("Request completed with status: {}", status); // <-- **Success Log**

            let headers = resp
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect::<serde_json::Value>();

            // Read the body
            let body_result = resp.text().await;
            let body = body_result.unwrap_or_default();

            Ok(json!({ "status": status, "headers": headers, "body": body }))
        }
        Err(e) => {
            // Log the request error (e.g., connection failure, timeout)
            tracing::error!("Request failed with error: {:?}", e.source());
            Err(json!({ "error": e.to_string() }))
        }
    };

    // Log that the IPC result is being sent
    tracing::debug!("Sending result via IPC proxy."); // <-- **End Log**
    let _ = event_proxy.send_event(UserEvent::IpcFetchResult { req_id, result });
}
