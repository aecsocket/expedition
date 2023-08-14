//! Features for writing out text messages as text using ANSI color codes, using [`termcolor`].

use termcolor::{Color, ColorSpec, WriteColor};

use crate::{prelude::*, util::StackFlattener};

impl Text {
    /// Writes this text message as a colored message to a [`termcolor::WriteColor`] object.
    ///
    /// This uses [`Text::flatten`] to convert from a node hierarchy to a linear sequence of
    /// [`ColorSpec`]s and messages.
    pub fn write<W: WriteColor>(&self, writer: &mut W) {
        let mut flattener = StackFlattener::new(|content, style| {
            let _ = writer.set_color(
                ColorSpec::new()
                    .set_fg(style.color.map(|c| Color::Rgb(c.r(), c.g(), c.b())))
                    .set_bold(style.bold == Some(true))
                    .set_italic(style.italic == Some(true))
                    .set_underline(style.underline == Some(true))
                    .set_strikethrough(style.strikethrough == Some(true)),
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
