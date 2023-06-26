use crate::types::error::RustTutorialError;

use super::Widget;

pub struct Label {
    pub label: String,
}

impl Label {
    pub fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

impl Widget for Label {
    fn width(&self) -> usize {
        self.label
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0)
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> Result<(), RustTutorialError> {
        writeln!(buffer, "{}", &self.label)?;
        Ok(())
    }
}
