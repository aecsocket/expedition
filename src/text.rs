use std::fmt;

use crate::prelude::*;

// core types

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Text {
    pub content: String,
    pub style: TextStyle,
    pub children: Vec<Text>,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: TextStyle::default(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextStyle {
    pub color: Option<Color32>,
    pub bold: StyleState,
    pub italic: StyleState,
    pub underline: StyleState,
    pub strikethrough: StyleState,
}

impl TextStyle {
    pub fn is_default(&self) -> bool {
        self.color.is_none()
            && self.bold == StyleState::default()
            && self.italic == StyleState::default()
            && self.underline == StyleState::default()
            && self.strikethrough == StyleState::default()
    }

    pub fn merge_from(&mut self, from: &Self) {
        fn decoration(this: StyleState, from: StyleState) -> StyleState {
            match from {
                StyleState::Inherit => this,
                state => state,
            }
        }

        self.color = match from.color {
            Some(t) => Some(t),
            None => self.color,
        };
        self.bold = decoration(self.bold, from.bold);
        self.italic = decoration(self.italic, from.italic);
        self.underline = decoration(self.underline, from.underline);
        self.strikethrough = decoration(self.strikethrough, from.strikethrough);
    }

    pub fn merged_from(&self, from: &Self) -> Self {
        let mut res = self.clone();
        res.merge_from(from);
        res
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StyleState {
    #[default]
    Inherit,
    On,
    Off,
}

// text traits

pub trait TextBuilder: Sized {
    fn into_text(self) -> Text;

    fn with(self, with: impl Into<Text>) -> Text {
        let mut text = self.into_text();
        text.children.push(with.into());
        text
    }
}

impl<T: Into<Text>> TextBuilder for T {
    fn into_text(self) -> Text {
        self.into()
    }
}

impl<T: Into<String>> From<T> for Text {
    fn from(value: T) -> Self {
        Text::new(value)
    }
}

// styling traits

pub trait Styleable: Sized {
    type Out;

    fn with_color(self, color: Option<Color32>) -> Self::Out;

    fn with_bold(self, state: StyleState) -> Self::Out;

    fn with_italic(self, state: StyleState) -> Self::Out;

    fn with_underline(self, state: StyleState) -> Self::Out;

    fn with_strikethrough(self, state: StyleState) -> Self::Out;

    fn color(self, color: Color32) -> Self::Out {
        self.with_color(Some(color))
    }

    fn bold(self) -> Self::Out {
        self.with_bold(StyleState::On)
    }

    fn no_bold(self) -> Self::Out {
        self.with_bold(StyleState::Off)
    }

    fn italic(self) -> Self::Out {
        self.with_italic(StyleState::On)
    }

    fn no_italic(self) -> Self::Out {
        self.with_italic(StyleState::Off)
    }

    fn underline(self) -> Self::Out {
        self.with_underline(StyleState::On)
    }

    fn no_underline(self) -> Self::Out {
        self.with_underline(StyleState::Off)
    }

    fn strikethrough(self) -> Self::Out {
        self.with_strikethrough(StyleState::On)
    }

    fn no_strikethrough(self) -> Self::Out {
        self.with_strikethrough(StyleState::Off)
    }
}

impl Styleable for TextStyle {
    type Out = Self;

    fn with_color(mut self, color: Option<Color32>) -> Self::Out {
        self.color = color;
        self
    }

    fn with_bold(mut self, state: StyleState) -> Self::Out {
        self.bold = state;
        self
    }

    fn with_italic(mut self, state: StyleState) -> Self::Out {
        self.italic = state;
        self
    }

    fn with_underline(mut self, state: StyleState) -> Self::Out {
        self.underline = state;
        self
    }

    fn with_strikethrough(mut self, state: StyleState) -> Self::Out {
        self.strikethrough = state;
        self
    }
}

impl<T: Into<Text>> Styleable for T {
    type Out = Text;

    fn with_color(self, color: Option<Color32>) -> Self::Out {
        let mut text = self.into();
        text.style.color = color;
        text
    }

    fn with_bold(self, state: StyleState) -> Self::Out {
        let mut text = self.into();
        text.style.bold = state;
        text
    }

    fn with_italic(self, state: StyleState) -> Self::Out {
        let mut text = self.into();
        text.style.italic = state;
        text
    }

    fn with_underline(self, state: StyleState) -> Self::Out {
        let mut text = self.into();
        text.style.underline = state;
        text
    }

    fn with_strikethrough(self, state: StyleState) -> Self::Out {
        let mut text = self.into();
        text.style.strikethrough = state;
        text
    }
}

// display

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = if self.content.is_empty() {
            None
        } else {
            Some(format!("{:?}", self.content))
        };

        let style = if self.style.is_default() {
            None
        } else {
            Some(format!("{}", self.style))
        };

        let children = if self.children.is_empty() {
            None
        } else {
            Some(format!(
                "[{}]",
                self.children
                    .iter()
                    .map(|c| c.to_string())
                    .intersperse(", ".to_owned())
                    .collect::<String>(),
            ))
        };

        let parts: Vec<String> = [content, style, children].into_iter().flatten().collect();

        if parts.len() == 1 {
            write!(f, "{}", parts[0])
        } else {
            write!(
                f,
                "({})",
                parts
                    .into_iter()
                    .intersperse("; ".to_owned())
                    .collect::<String>(),
            )
        }
    }
}

impl fmt::Display for TextStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn decoration(state: StyleState, name: &'static str) -> Option<String> {
            use StyleState::*;

            match state {
                Inherit => None,
                On => Some(name.to_owned()),
                Off => Some(format!("!{}", name)),
            }
        }

        let color = self.color.map(|color| format!("{:?}", color));
        let bold = decoration(self.bold, "Bold");
        let italic = decoration(self.italic, "Italic");
        let underline = decoration(self.underline, "Underline");
        let strikethrough = decoration(self.strikethrough, "Strikethrough");

        write!(
            f,
            "{}",
            [color, bold, italic, underline, strikethrough]
                .into_iter()
                .flatten()
                .intersperse(" + ".to_owned())
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn a() {
        let text = ""
            .italic()
            .with("Hello ".with("lovely".underline()))
            .with(" World!".no_italic().bold());

        println!("{}", text);
    }
}
