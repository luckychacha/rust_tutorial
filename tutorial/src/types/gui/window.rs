use crate::types::error::RustTutorialError;

use super::Widget;

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    pub fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        let window_width = self
            .widgets
            .iter()
            .map(|widget| widget.width())
            .max()
            .unwrap_or(self.title.len());
        window_width + 10
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> Result<(), RustTutorialError> {
        let width = self.width();

        // generate inner widget contents.
        let mut inner_buffer = String::new();
        for widget in &self.widgets {
            widget.draw_into(&mut inner_buffer)?;
        }

        // print title
        writeln!(buffer, "+-{:-<width$}-+", "")?;
        writeln!(buffer, "| {:^width$} |", &self.title)?;
        writeln!(buffer, "+={:=<width$}=+", "")?;

        // print widgets
        for line in inner_buffer.lines() {
            writeln!(buffer, "| {:<width$} |", line)?;
        }

        // end
        writeln!(buffer, "+-{:-<width$}-+", "")?;

        Ok(())
        // unimplemented!()
    }

    fn draw(&self) {
        let mut buffer = String::new();
        if let Ok(()) = self.draw_into(&mut buffer) {
            println!("{buffer}");
        }
    }
}
