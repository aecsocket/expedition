//! Features for converting objects to an [`egui`] format.

use egui::{text::LayoutJob, Align, Color32, FontId, Stroke, TextFormat};

use crate::{Message, MessageStyle, StackFlattener};

/// Defines how to convert a [`MessageStyle`] into [`TextFormat`] for egui.
///
/// Since a [`MessageStyle`] is a simpler and less featureful type than [`TextFormat`], we must
/// provide some defaults if we want to convert the former into the latter. This struct provides
/// the defaults that we use when converting.
///
/// You can also use this to convert an entire [`Message`] into a [`LayoutJob`] using the same format
/// conversion.
///
/// # Examples
///
/// ```
/// use expedition::{egui::StyleToFormat, Color32, MessageStyle, Styleable};
/// use egui::{FontId, Stroke, TextFormat};
///
/// let style_to_format = StyleToFormat {
///     // you can't specify a font in `MessageStyle`, so we specify a default here
///     font_id: FontId::monospace(14.0),
///     // in egui, an underline is determined by a `Stroke` - no underline means `Stroke::NONE`
///     // in `MessageStyle`, underline is just a boolean
///     // so here we define one property of the underline stroke, which will be created *if*
///     // the actual text object has an underline
///     // note that the color of all strokes is set to the text object's color
///     underline_width: 1.5,
///     ..Default::default()
/// };
///
/// assert_eq!(
///     TextFormat {
///         font_id: FontId::monospace(14.0),
///         italics: true,
///         // since we won't apply an underline, the stroke is NONE
///         underline: Stroke::NONE,
///         ..Default::default()
///     },
///     style_to_format.to_format(MessageStyle::default()
///         .italic()),
/// );
///
/// assert_eq!(
///     TextFormat {
///         font_id: FontId::monospace(14.0),
///         // since we'll apply an underline, here is where our `1.5` from before will be used
///         underline: Stroke::new(1.5, Color32::RED),
///         color: Color32::RED,
///         ..Default::default()
///     },
///     style_to_format.to_format(MessageStyle::default()
///         .color(Color32::RED)
///         .underline()),
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StyleToFormat {
    /// [`TextFormat::font_id`]
    pub font_id: FontId,
    /// [`TextFormat::background`]
    pub background: Color32,
    /// [`TextFormat::color`]
    pub default_color: Color32,
    /// [`Stroke::width`] of [`TextFormat::underline`]
    pub underline_width: f32,
    /// [`Stroke::width`] of [`TextFormat::strikethrough`]
    pub strikethrough_width: f32,
    /// [`TextFormat::valign`]
    pub valign: Align,
}

impl Default for StyleToFormat {
    fn default() -> Self {
        Self {
            font_id: FontId::default(),
            background: Color32::TRANSPARENT,
            default_color: Color32::GRAY,
            underline_width: 1.0,
            strikethrough_width: 1.0,
            valign: Align::BOTTOM,
        }
    }
}

impl StyleToFormat {
    /// Converts a [`MessageStyle`] to a [`TextFormat`] using the defaults provided in this struct.
    pub fn to_format(&self, style: MessageStyle) -> TextFormat {
        let foreground = style.color.unwrap_or(self.default_color);
        TextFormat {
            font_id: self.font_id.clone(),
            color: foreground,
            background: self.background,
            italics: style.italic == Some(true),
            underline: match style.underline {
                Some(true) => Stroke::new(self.underline_width, foreground),
                _ => Stroke::NONE,
            },
            strikethrough: match style.strikethrough {
                Some(true) => Stroke::new(self.strikethrough_width, foreground),
                _ => Stroke::NONE,
            },
            valign: self.valign,
        }
    }

    /// Converts a hierarchy of [`Message`] nodes to a sequence of [`LayoutJob`] styled sections.
    ///
    /// This uses [`Message::flatten`] to perform the conversion from hierarchy to [`LayoutJob::append`] calls.
    pub fn to_job(&self, text: &Message) -> LayoutJob {
        let mut job = LayoutJob::default();
        let mut flattener = StackFlattener::new(|content, style| {
            job.append(content, 0.0, self.to_format(style));
        });
        text.flatten(&mut flattener);
        job
    }
}
