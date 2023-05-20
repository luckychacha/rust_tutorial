use crate::types::error::RustTutorialError;

use super::{label::Label, Widget};

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
}

impl Button {
    pub fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + 2
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> Result<(), RustTutorialError> {
        let width = self.width();
        let mut label = String::new();
        self.label.draw_into(&mut label)?;

        writeln!(buffer, "+{:-<width$}+", "")?;
        for line in label.lines() {
            writeln!(buffer, "|{:^width$}|", line)?;
        }
        writeln!(buffer, "+{:-<width$}+", "")?;

        // println!("left aligned:  |{:/<width$}|", "foo");
        Ok(())
    }
}
