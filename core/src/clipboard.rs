use clipboard::{ClipboardContext, ClipboardProvider};

use std::error::Error;

pub fn copy_value_to_clipboard(value: String) -> Result<(), Box<dyn Error>> {
    let mut ctx: ClipboardContext = match ClipboardProvider::new() {
        Ok(ctx) => ctx,
        Err(_) => return Err(Box::from("Failed to get clipboard context")),
    };

    match ctx.set_contents(value) {
        Ok(_) => Ok(()),
        Err(_) => Err(Box::from("Failed to copy value to clipboard")),
    }
}