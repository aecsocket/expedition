//! The core objects used by the library.

use std::fmt;

use crate::prelude::*;
use itertools::Itertools;

// core types

/// Represents a generic rich text message, designed to be used as the intermediary format for
/// text manipulation.
///
/// This provides rich text features such as decoration and coloring, which you can easily create
/// yourself via the builder-style APIs. The features available are designed to be simple and
/// universal, so that as many different output modes as possible can be supported. For example,
/// text can be converted to:
/// - a raw string
/// - ANSI codes for printing to a terminal
/// - a [`LayoutJob`] for [`egui`]
///
/// The messages are stored in a tree structure, with one message capable of holding child nodes,
/// which can be traversed and used for applying styling on top of existing styling.
///
/// # Examples
///
/// ```
/// use expedition::prelude::*;
///
/// // Create a new Text using `new`, passing an `Into<String>`
/// let text1 = Text::new("Hello world!");
///
/// // Or get one directly from a string
/// let text2 = "Hello world!".into_text();
/// assert_eq!(text1, text2);
///
/// // Apply styling using a builder pattern
/// let text1 = Text::new("Unstyled text, ")
///     .with(Text::new("red text, ").color(Color32::RED))
///     .with(Text::new("blue text").color(Color32::BLUE));
///
/// // Which can be simplified to
/// let text2 = "Unstyled text, "
///     .with("red text, ".color(Color32::RED))
///     .with("blue text".color(Color32::BLUE));
/// assert_eq!(text1, text2);
///
/// // Add decorations such as bold, italic, underline, and strikethrough
/// let text = "Unstyled, "
///     .with("bold text, ".bold())
///     .with("italic text, ".italic())
///     .with("all the decorations".bold().italic().underline().strikethrough());
///
/// // Styling from child nodes takes priority over parent nodes
/// let text = "Red text, ".color(Color32::RED)
///     .with("still red text, ")
///     .with("red and italic, ".italic())
///     .with("blue and not italic".color(Color32::BLUE));
///
/// // Or use `no_X()` to disable the decoration `X`
/// let text = "Italic text, ".italic()
///     .with("not italic anymore".no_italic());
/// ```
///
/// # Output
///
/// ## To raw string
///
/// Text implements [`fmt::Display`], so just use [`ToString`] or as an argument in a [`format!`]
/// call to get a raw string without any styling.
///
/// ```
/// # use expedition::prelude::*;
///
/// let text = "Hello, "
///     .with("world!");
/// assert_eq!("Hello, world!", text.to_string());
///
/// let text = "This ignores all styling".color(Color32::RED).italic().bold();
/// assert_eq!("This ignores all styling", text.to_string());
/// ```
///
/// ## Other formats
///
/// See the documentation for the crate features to see usgae info for specific output formats.
///
/// [`LayoutJob`]: egui::text::LayoutJob
/// ```
#[derive(Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Text {
    /// What text this object holds.
    pub content: String,
    /// Decoration and formatting applied to this text message.
    pub style: TextStyle,
    /// Child text messages added on to this text.
    pub children: Vec<Text>,
}

/// Styling that is currently applied to the contents of a [`Text`].
///
/// All styling elements are optional, as styles can be layered on top of one another through
/// merging. Because of this, [`Default`] returns a style object that applies no styling changes
/// to a piece of text - effectively an "identity style".
#[derive(Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextStyle {
    /// Foreground text color.
    pub color: Option<Color32>,
    /// Bold decoration.
    pub bold: Option<bool>,
    /// Italic decoration.
    pub italic: Option<bool>,
    /// Underline decoration.
    pub underline: Option<bool>,
    /// Strikethrough decoration.
    pub strikethrough: Option<bool>,
}

impl Text {
    /// Creates a new text message with default styling and no children.
    ///
    /// It is also possible to create text directly from an `Into<String>`.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: TextStyle::default(),
            children: Vec::new(),
        }
    }
}

impl TextStyle {
    /// Creates a new text style from the defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets if this style is equivalent to the default text style - that is, a style which will
    /// make no changes.
    #[must_use]
    pub fn is_default(&self) -> bool {
        self == &Self::default()
    }

    /// Merges another style into this one, with the values in `from` taking precedence over the
    /// values in `self`.
    pub fn merge_from(&mut self, from: &Self) {
        self.color = from.color.or(self.color);
        self.bold = from.bold.or(self.bold);
        self.italic = from.italic.or(self.italic);
        self.underline = from.underline.or(self.underline);
        self.strikethrough = from.strikethrough.or(self.strikethrough);
    }

    /// Creates a new style which is the result of merging `from` on top of `self`, using
    /// [`Self::merge_from`]
    #[must_use]
    pub fn merged_from(&self, from: &Self) -> Self {
        let mut res = self.clone();
        res.merge_from(from);
        res
    }
}

// text traits

/// Used to create a [`Text`].
pub trait TextBuilder: Sized {
    /// Converts this object into a [`Text`].
    fn into_text(self) -> Text;

    /// Appends `with` to the end of this text message.
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
        Self::new(value)
    }
}

// styling traits

/// Used to apply styling to a text message.
pub trait Styleable: Sized {
    /// The resulting type of all operations.
    type Out;

    /// Changes the color state.
    fn with_color(self, color: Option<Color32>) -> Self::Out;

    /// Changes the bold state.
    fn with_bold(self, state: Option<bool>) -> Self::Out;

    /// Changes the italic state.
    fn with_italic(self, state: Option<bool>) -> Self::Out;

    /// Changes the underline state.
    fn with_underline(self, state: Option<bool>) -> Self::Out;

    /// Changes the strikethrough state.
    fn with_strikethrough(self, state: Option<bool>) -> Self::Out;

    /// Sets a color.
    fn color(self, color: Color32) -> Self::Out {
        self.with_color(Some(color))
    }

    /// Sets bold to be enabled.
    fn bold(self) -> Self::Out {
        self.with_bold(Some(true))
    }

    /// Sets bold to be disabled.
    fn no_bold(self) -> Self::Out {
        self.with_bold(Some(false))
    }

    /// Sets italic to be enabled.
    fn italic(self) -> Self::Out {
        self.with_italic(Some(true))
    }

    /// Sets italic to be disabled.
    fn no_italic(self) -> Self::Out {
        self.with_italic(Some(false))
    }

    /// Sets underline to be enabled.
    fn underline(self) -> Self::Out {
        self.with_underline(Some(true))
    }

    /// Sets underline to be disabled.
    fn no_underline(self) -> Self::Out {
        self.with_underline(Some(false))
    }

    /// Sets strikethrough to be enabled.
    fn strikethrough(self) -> Self::Out {
        self.with_strikethrough(Some(true))
    }

    /// Sets strikethrough to be disabled.
    fn no_strikethrough(self) -> Self::Out {
        self.with_strikethrough(Some(false))
    }
}

impl Styleable for TextStyle {
    type Out = Self;

    fn with_color(mut self, color: Option<Color32>) -> Self::Out {
        self.color = color;
        self
    }

    fn with_bold(mut self, state: Option<bool>) -> Self::Out {
        self.bold = state;
        self
    }

    fn with_italic(mut self, state: Option<bool>) -> Self::Out {
        self.italic = state;
        self
    }

    fn with_underline(mut self, state: Option<bool>) -> Self::Out {
        self.underline = state;
        self
    }

    fn with_strikethrough(mut self, state: Option<bool>) -> Self::Out {
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

    fn with_bold(self, state: Option<bool>) -> Self::Out {
        let mut text = self.into();
        text.style.bold = state;
        text
    }

    fn with_italic(self, state: Option<bool>) -> Self::Out {
        let mut text = self.into();
        text.style.italic = state;
        text
    }

    fn with_underline(self, state: Option<bool>) -> Self::Out {
        let mut text = self.into();
        text.style.underline = state;
        text
    }

    fn with_strikethrough(self, state: Option<bool>) -> Self::Out {
        let mut text = self.into();
        text.style.strikethrough = state;
        text
    }
}

// display + debug

impl fmt::Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = if self.content.is_empty() {
            None
        } else {
            Some(format!("{:?}", self.content))
        };

        let style = if self.style.is_default() {
            None
        } else {
            Some(format!("{:?}", self.style))
        };

        let children = if self.children.is_empty() {
            None
        } else {
            Some(format!("{:?}", self.children))
        };

        let parts: Vec<String> = [content, style, children].into_iter().flatten().collect();

        if parts.len() == 1 {
            write!(f, "{}", parts[0])
        } else {
            write!(f, "({})", parts.into_iter().join(", "))
        }
    }
}

impl fmt::Debug for TextStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn decoration(state: Option<bool>, name: &'static str) -> Option<String> {
            state.map(|value| if value {
                name.to_owned()
            } else {
                format!("!{}", name)
            })
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
                .join(" + "),
        )
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[derive(Default)]
        struct Flattener {
            buf: String,
        }

        impl TextFlattener for Flattener {
            fn push_style(&mut self, _: &TextStyle) {}

            fn content(&mut self, content: &str) {
                self.buf.push_str(content);
            }

            fn pop_style(&mut self, _: &TextStyle) {}
        }

        let mut flattener = Flattener::default();
        self.flatten(&mut flattener);
        write!(f, "{}", flattener.buf)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn default() {
        assert_eq!(
            Text {
                content: String::new(),
                style: TextStyle {
                    color: None,
                    bold: None,
                    italic: None,
                    underline: None,
                    strikethrough: None
                },
                children: Vec::new(),
            },
            Text::default(),
        );
    }

    #[test]
    fn with() {
        assert_eq!(
            Text {
                content: "one".to_owned(),
                children: vec![Text::new("two"), Text::new("three"),],
                ..Default::default()
            },
            "one".with("two").with("three"),
        )
    }
}
