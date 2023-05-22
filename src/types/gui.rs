use super::error::RustTutorialError;

pub mod button;
pub mod label;
pub mod window;

pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> Result<(), RustTutorialError>;

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        if let Ok(()) = self.draw_into(&mut buffer) {
            println!("{buffer}");
        }
    }
}
