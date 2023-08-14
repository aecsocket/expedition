//! Utilities for manipulating text and text hierarchies.

use crate::prelude::*;

impl Text {
    /// Allows flattening a hierarchy of text nodes into a linear sequence of styles and strings.
    ///
    /// This function traverses the tree of text message nodes via a depth-first method, starting
    /// with `self` as the root, and on each node providing the `flattener` with:
    /// - [`TextFlattener::push_style`]: the style of the node entered
    /// - [`TextFlattener::content`]: the content of the node
    /// - [`TextFlattener::pop_style`]: the style of the node exited
    ///
    /// The push/pop functions can be used to implement a style stack, from which the topmost
    /// style can be computed. Combining this with the content, you effectively have access to
    /// the entire styled text message, which you can use for outputting.
    ///
    /// To see an implemention which already has the style stack, see [`StackFlattener`].
    pub fn flatten<F: TextFlattener>(&self, flattener: &mut F) {
        flattener.push_style(&self.style);

        flattener.content(&self.content);
        for child in self.children.iter() {
            child.flatten(flattener);
        }

        flattener.pop_style(&self.style);
    }
}

/// Functions called when flattening a hierarchy of [`Text`] nodes using [`Text::flatten`].
pub trait TextFlattener {
    /// Called when a new style is entered.
    fn push_style(&mut self, style: &TextStyle);

    /// Called when a new piece of text content is encountered.
    fn content(&mut self, content: &str);

    /// Called when exiting a style that we previously entered.
    fn pop_style(&mut self, style: &TextStyle);
}

/// A [`TextFlattener`] implementation which maintains a stack of [`TextStyle`]s internally,
/// and provides access via a consumer function.
///
/// When using [`Text::flatten`], a new style is pushed onto the style stack. This style is
/// the result of merging the currently topmost style (or [`TextStyle::default`] if there are no
/// styles) with the new style entered. When a style is exited, the topmost element of the stack
/// is popped. Therefore, the topmost element of this stack will always be the final merged style
/// of the text currently being processed.
///
/// When text is encountered, the content and the current topmost style is provided to the consumer
/// function `F`. Use this to implement your own logic.
///
/// # Examples
///
/// ```
/// use expedition::prelude::*;
///
/// let text = "Unstyled "
///     .with("red ".color(Color32::RED))
///     .with("blue ".color(Color32::BLUE))
///         .with("italic ".italic())
///     .with("final");
///
/// let mut all_content: Vec<String> = Vec::new();
/// let mut flattener = StackFlattener::new(|content, style| {
///     all_content.push(content.to_owned());
/// });
/// text.flatten(&mut flattener);
/// assert_eq!(vec!["Unstyled ", "red ", "blue ", "italic ", "final"], all_content);
/// ```
pub struct StackFlattener<F> {
    style_stack: Vec<TextStyle>,
    consumer: F,
}

impl<F> StackFlattener<F>
where
    F: FnMut(&str, &TextStyle) -> (),
{
    /// Creates a new flattener with an empty style stack, and taking in the consumer that is
    /// called when content is encountered./
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
