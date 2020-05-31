use clipboard::{ClipboardContext, ClipboardProvider};

pub fn copy_value_to_clipboard(value: String) -> Result<(), String> {
    let mut ctx: ClipboardContext = match ClipboardProvider::new() {
        Ok(ctx) => ctx,
        Err(_) => return Err(String::from("Failed to get clipboard context")),
    };

    match ctx.set_contents(value) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Failed to copy value to clipboard")),
    }
}