use tao::error::ExternalError;

pub fn drag_handler(_window: tao::window::Window) -> Result<(), ExternalError> {
    _window.drag_window()
}
