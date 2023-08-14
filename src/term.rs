use termcolor::{Color, ColorSpec, WriteColor};

use crate::{prelude::*, util::StackFlattener};

impl Text {
    pub fn write<W: WriteColor>(&self, writer: &mut W) {
        let mut flattener = StackFlattener::new(|content, style| {
            let _ = writer.set_color(
                ColorSpec::new()
                    .set_fg(style.color.map(|c| Color::Rgb(c.r(), c.g(), c.b())))
                    .set_bold(style.bold == StyleState::On)
                    .set_italic(style.italic == StyleState::On)
                    .set_underline(style.underline == StyleState::On)
                    .set_strikethrough(style.strikethrough == StyleState::On),
            );
            let _ = write!(writer, "{}", content);
        });
        self.flatten(&mut flattener);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::io::Write;
    use termcolor::{ColorChoice, StandardStream, WriteColor};

    #[test]
    fn a() {
        let text = "Unstyled, "
            .with("Red ".color(Color32::RED).with("and some bold ".bold()))
            .with("Blue ".color(Color32::BLUE).with("and italic ".italic()))
            .with("but no longer ")
            .with("underline".underline())
            .with(" EVERYTHING".bold().italic().underline().strikethrough());

        let mut stdout = StandardStream::stdout(ColorChoice::Auto);
        text.write(&mut stdout);
        let _ = writeln!(&mut stdout);
        let _ = stdout.reset();
    }
}
