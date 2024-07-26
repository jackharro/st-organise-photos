use ratatui;
use ratatui_image;
use std::path::PathBuf;

pub enum Objects {
    Image(Vec<PathBuf>),
    Manpage(Vec<String>),

}

pub fn main() {
    println!("Hello world");
}

pub mod tui {
    fn main(obj: crate::Objects) -> Result<(), std::io::Error> {
    Ok(())
    }
}
