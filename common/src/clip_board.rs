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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_value = String::from("123");
        copy_value_to_clipboard(test_value.clone()).unwrap();

        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let value = ctx.get_contents().unwrap();

        assert_eq!(test_value, value);
    }
}