//! Mouse utility functions.

use crate::error::GameError;
use crate::error::GameResult;
use crate::graphics::Point2;
use std::collections::HashSet;
use winit::dpi;
pub use winit::event::MouseButton;
pub use winit::window::CursorIcon;

/// Stores state information for the mouse.
#[derive(Clone, Debug)]
pub struct MouseContext {
    last_position: Point2,
    last_delta: Point2,
    delta: Point2,
    buttons_pressed: HashSet<MouseButton>,
    cursor_type: CursorIcon,
    cursor_grabbed: bool,
    cursor_hidden: bool,
    previous_buttons_pressed: HashSet<MouseButton>,
}

impl MouseContext {
    pub(crate) fn new() -> Self {
        Self {
            last_position: Point2::ZERO,
            last_delta: Point2::ZERO,
            delta: Point2::ZERO,
            cursor_type: CursorIcon::Default,
            buttons_pressed: HashSet::new(),
            cursor_grabbed: false,
            cursor_hidden: false,
            previous_buttons_pressed: HashSet::new(),
        }
    }

    pub(crate) fn set_last_position(&mut self, p: Point2) {
        self.last_position = p;
    }

    pub(crate) fn set_last_delta(&mut self, p: Point2) {
        self.last_delta = p;
    }

    /// Resets the value returned by [`mouse::delta`](fn.delta.html) to zero.
    /// You shouldn't need to call this, except when you're running your own event loop.
    /// In this case call it right at the end, after `draw` and `update` have finished.
    pub fn reset_delta(&mut self) {
        self.delta = Point2::ZERO;
    }

    pub(crate) fn set_delta(&mut self, p: Point2) {
        self.delta = p;
    }

    pub(crate) fn set_button(&mut self, button: MouseButton, pressed: bool) {
        if pressed {
            let _ = self.buttons_pressed.insert(button);
        } else {
            let _ = self.buttons_pressed.remove(&button);
        }
    }

    /// Returns whether or not the given mouse button has been pressed this frame.
    pub(crate) fn button_just_pressed(&self, button: MouseButton) -> bool {
        self.buttons_pressed.contains(&button) && !self.previous_buttons_pressed.contains(&button)
    }

    /// Returns whether or not the given mouse button has been released this frame.
    pub(crate) fn button_just_released(&self, button: MouseButton) -> bool {
        !self.buttons_pressed.contains(&button) && self.previous_buttons_pressed.contains(&button)
    }

    /// Copies the current state of the mouse buttons into the context. If you are writing your own event loop
    /// you need to call this at the end of every update in order to use the functions `is_button_just_pressed`
    /// and `is_button_just_released`. Otherwise this is handled for you.
    pub fn save_mouse_state(&mut self) {
        self.previous_buttons_pressed = self.buttons_pressed.clone();
    }

    /// Returns whether or not the given mouse button is pressed.
    pub fn button_pressed(&self, button: MouseButton) -> bool {
        *(self.buttons_pressed.get(&button).unwrap_or(&false))
    }

    /// Returns the current mouse cursor type of the window.
    pub fn cursor_type(&self) -> CursorIcon {
        self.cursor_type
    }

    /// Modifies the mouse cursor type of a window.
    pub fn set_cursor_type(&mut self, window: &glutin::window::Window, cursor_type: CursorIcon) {
        self.cursor_type = cursor_type;
        window.set_cursor_icon(cursor_type);
    }

    /// Get whether or not the mouse is grabbed (confined to the window)
    pub fn cursor_grabbed(&self) -> bool {
        self.cursor_grabbed
    }

    /// Set whether or not the mouse is grabbed (confined to the window)
    pub fn set_cursor_grabbed(
        &mut self,
        window: &glutin::window::Window,
        grabbed: bool,
    ) -> GameResult<()> {
        self.cursor_grabbed = grabbed;
        window
            .set_cursor_grab(grabbed)
            .map_err(|e| GameError::WindowError(e.to_string()))
    }

    /// Set whether or not the mouse is hidden (invisible)
    pub fn cursor_hidden(&self) -> bool {
        self.cursor_hidden
    }

    /// Set whether or not the mouse is hidden (invisible).
    pub fn set_cursor_hidden(&mut self, window: &glutin::window::Window, hidden: bool) {
        self.cursor_hidden = hidden;
        window.set_cursor_visible(!hidden)
    }

    /// Get the current position of the mouse cursor, in pixels.
    /// Complement to [`set_position()`](fn.set_position.html).
    /// Uses strictly window-only coordinates.
    pub fn position(&self) -> mint::Point2<f32> {
        self.last_position.into()
    }

    /// Set the current position of the mouse cursor, in pixels.
    /// Uses strictly window-only coordinates.
    pub fn set_position<P>(&mut self, window: &glutin::window::Window, point: P) -> GameResult<()>
    where
        P: Into<mint::Point2<f32>>,
    {
        let mintpoint = point.into();
        self.last_position = Point2::from(mintpoint);
        window
            .set_cursor_position(dpi::LogicalPosition {
                x: f64::from(mintpoint.x),
                y: f64::from(mintpoint.y),
            })
            .map_err(|_| GameError::WindowError("Couldn't set mouse cursor position!".to_owned()))
    }

    /// Get the distance the cursor was moved during the current frame, in pixels.
    pub fn delta(&self) -> mint::Point2<f32> {
        self.delta.into()
    }

    /// Get the distance the cursor was moved between the latest two mouse_motion_events.
    pub(crate) fn last_delta(&self) -> mint::Point2<f32> {
        self.last_delta.into()
    }
}

impl Default for MouseContext {
    fn default() -> Self {
        Self::new()
    }
}
