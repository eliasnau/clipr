use arboard::Clipboard;
use anyhow::Result;

pub fn copy_to_clipboard(text: &str, append: bool) -> Result<()> {
    let mut clipboard = Clipboard::new()?;
    if append {
        let current = clipboard.get_text().unwrap_or_default();
        clipboard.set_text(current + text)?;
    } else {
        clipboard.set_text(text.to_owned())?;
    }
    Ok(())
}

pub fn get_clipboard_content() -> Result<String> {
    let mut clipboard = Clipboard::new()?;  // Added 'mut' here
    Ok(clipboard.get_text()?)
}
