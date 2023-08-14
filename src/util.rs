use crate::prelude::*;

pub trait TextFlattener {
    fn push_style(&mut self, style: &TextStyle);

    fn content(&mut self, content: &str);

    fn pop_style(&mut self, style: &TextStyle);
}

impl Text {
    pub fn flatten<F: TextFlattener>(&self, flattener: &mut F) {
        flattener.push_style(&self.style);

        flattener.content(&self.content);
        for child in self.children.iter() {
            child.flatten(flattener);
        }

        flattener.pop_style(&self.style);
    }
}

pub struct StackFlattener<F> {
    style_stack: Vec<TextStyle>,
    consumer: F,
}

impl<F> StackFlattener<F>
where
    F: FnMut(&str, &TextStyle) -> (),
{
    pub fn new(consumer: F) -> Self {
        Self {
            style_stack: Vec::new(),
            consumer,
        }
    }
}

impl<F> TextFlattener for StackFlattener<F>
where
    F: FnMut(&str, &TextStyle) -> (),
{
    fn push_style(&mut self, style: &TextStyle) {
        self.style_stack.push(
            self.style_stack
                .last()
                .unwrap_or(&TextStyle::default())
                .merged_from(style),
        );
    }

    fn content(&mut self, content: &str) {
        let default_style = TextStyle::default();
        let style = self.style_stack.last().unwrap_or(&default_style);
        (self.consumer)(content, style);
    }

    fn pop_style(&mut self, _: &TextStyle) {
        self.style_stack.pop();
    }
}
