use std::fmt::Debug;
use arboard::{Clipboard, Error};

pub struct CustomClipboard {
    pub clipboard: Clipboard,
}

impl Debug for CustomClipboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CustomClipboard")
    }
}

impl Default for CustomClipboard {
    fn default() -> Self {
        Self {
            clipboard: Clipboard::new().unwrap(),
        }
    }
}

impl Clone for CustomClipboard {
    fn clone(&self) -> Self {
        Self {
            clipboard: Clipboard::new().unwrap(),
        }
    }
}

impl CustomClipboard {
    pub fn set_text(&mut self, text: &str) -> Result<(), Error> {
        self.clipboard.set_text(text)
    }
    pub fn get_text(&mut self) -> Result<String, Error> {
        self.clipboard.get_text()
    }
    pub fn clear(&mut self) -> Result<(), Error> {
        self.clipboard.clear()
    }
}
