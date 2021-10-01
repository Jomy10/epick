#[cfg(unix)]
mod x11;

use crate::color::Color;
use anyhow::Result;
use std::fmt::Debug;

pub trait DisplayPicker: Debug {
    fn get_color_under_cursor(&self) -> Result<Color>;
}

pub fn init_display_picker() -> Option<Box<dyn DisplayPicker>> {
    #[cfg(unix)]
    return x11::X11Conn::new()
        .ok()
        .map(|conn| Box::new(conn) as Box<dyn DisplayPicker>);
    #[cfg(not(unix))]
    return None;
}