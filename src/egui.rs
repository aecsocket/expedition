use egui::{text::LayoutJob, Align, FontId, Stroke, TextFormat};

use crate::prelude::*;

/// Defines how to convert a [TextStyle] into [TextFormat] for egui, allowing you to specify
/// defaults for fields which [TextStyle] doesn't support.
///
/// # Usage
///
///
pub struct TextStyleJob {
    pub font_id: FontId,
    pub background: Color32,
    pub default_color: Color32,
    pub underline_width: f32,
    pub strikethrough_width: f32,
    pub valign: Align,
}

impl Default for TextStyleJob {
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

impl TextStyleJob {
    pub fn to_format(&self, style: &TextStyle) -> TextFormat {
        let foreground = style.color.unwrap_or(self.default_color);
        TextFormat {
            font_id: self.font_id.clone(),
            color: foreground,
            background: self.background,
            italics: style.italic == StyleState::On,
            underline: match style.underline {
                StyleState::On => Stroke::new(self.underline_width, foreground),
                _ => Stroke::NONE,
            },
            strikethrough: match style.strikethrough {
                StyleState::On => Stroke::new(self.strikethrough_width, foreground),
                _ => Stroke::NONE,
            },
            valign: self.valign,
        }
    }

    pub fn to_job(&self, text: &Text) -> LayoutJob {
        let mut job = LayoutJob::default();
        let mut flattener = StackFlattener::new(|content, style| {
            job.append(content, 0.0, self.to_format(style));
        });
        text.flatten(&mut flattener);
        job
    }
}
