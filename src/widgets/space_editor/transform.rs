use druid::{Data, Event, MouseEvent, Point, Size, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Clone, Data, Deserialize, Serialize)]
pub struct Transform {
    pub pan: Vec2,
    pub zoom_level: f64,
}

impl Transform {
    /// Pans the viewport transform by the given amount.
    pub fn pan_by(&mut self, delta: Vec2) {
        self.pan -= delta / self.zoom();
    }

    /// Returns the zoom amount, calculated from the zoom level.
    pub fn zoom(&self) -> f64 {
        f64::powf(2.0, self.zoom_level * 0.25)
    }

    /// Converts a point from screen space to viewport space.
    ///
    /// This can be used to pick things in the viewport, given a mouse position.
    pub fn to_viewport_space(&self, point: Point, viewport_size: Size) -> Point {
        let vec = (point - viewport_size.to_vec2() / 2.0).to_vec2() / self.zoom() + self.pan;
        vec.to_point()
    }

    /// Converts a point from viewport space to screen space.
    ///
    /// This transformation is the inverse of [`Viewport::to_viewport_space`].
    pub fn to_screen_space(&self, point: Point, viewport_size: Size) -> Point {
        let vec = (point - self.pan).to_vec2() * self.zoom() + viewport_size.to_vec2() / 2.0;
        vec.to_point()
    }

    /// Converts the screen space mouse position in the given event to a viewport space position,
    /// and returns a new event with the altered position.
    ///
    /// Only the widget-relative position is altered ([`MouseEvent::pos`]), the window-relative
    /// position is left as is.
    pub fn mouse_to_viewport_space(&self, event: &Event, viewport_size: Size) -> Event {
        let convert_mouse_event = |event: &MouseEvent| {
            let mut event = event.clone();
            event.pos = self.to_viewport_space(event.pos, viewport_size);
            event
        };

        match event {
            Event::MouseDown(mouse) => Event::MouseDown(convert_mouse_event(mouse)),
            Event::MouseUp(mouse) => Event::MouseUp(convert_mouse_event(mouse)),
            Event::MouseMove(mouse) => Event::MouseMove(convert_mouse_event(mouse)),
            _ => event.clone(),
        }
    }
}
