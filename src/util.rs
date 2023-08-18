//! Utilities for manipulating text and text hierarchies.

use crate::{Message, MessageStyle};

impl Message {
    /// Allows flattening a hierarchy of message nodes into a linear sequence of styles and strings.
    ///
    /// This function traverses the tree of message nodes via a depth-first method, starting
    /// with `self` as the root, and on each node providing the `flattener` with:
    /// - [`MessageFlattener::push_style`]: the style of the node entered
    /// - [`MessageFlattener::content`]: the content of the node
    /// - [`MessageFlattener::pop_style`]: the style of the node exited
    ///
    /// The push/pop functions can be used to implement a style stack, from which the topmost
    /// style can be computed. Combining this with the content, you effectively have access to
    /// the entire styled message, which you can use for outputting.
    ///
    /// To see an implemention which already has the style stack, see [`StackFlattener`].
    pub fn flatten<F: MessageFlattener>(&self, flattener: &mut F) {
        flattener.push_style(self.style);

        flattener.content(&self.content);
        for child in &self.children {
            child.flatten(flattener);
        }

        flattener.pop_style(self.style);
    }
}

/// Functions called when flattening a hierarchy of [`Message`] nodes using [`Message::flatten`].
pub trait MessageFlattener {
    /// Called when a new style is entered.
    fn push_style(&mut self, style: MessageStyle);

    /// Called when a new piece of text content is encountered.
    fn content(&mut self, content: &str);

    /// Called when exiting a style that we previously entered.
    fn pop_style(&mut self, style: MessageStyle);
}

/// A [`MessageFlattener`] implementation which maintains a stack of [`MessageStyle`]s internally,
/// and provides access via a consumer function.
///
/// When using [`Message::flatten`], a new style is pushed onto the style stack. This style is
/// the result of merging the currently topmost style (or [`MessageStyle::default`] if there are no
/// styles) with the new style entered. When a style is exited, the topmost element of the stack
/// is popped. Therefore, the topmost element of this stack will always be the final merged style
/// of the message currently being processed.
///
/// When text is encountered, the content and the current topmost style is provided to the consumer
/// function `F`. Use this to implement your own logic.
///
/// # Examples
///
/// ```
/// # use expedition::{Color32, IntoMessage, Styleable, StackFlattener, MessageStyle};
/// let msg = "Unstyled "
///     .with("red ".color(Color32::RED))
///     .with("blue ".color(Color32::BLUE))
///         .with("italic ".italic())
///     .with("final");
///
/// let mut all_content: Vec<String> = Vec::new();
/// let mut flattener = StackFlattener::new(|content, style| {
///     all_content.push(content.to_owned());
/// });
/// msg.flatten(&mut flattener);
/// assert_eq!(vec!["Unstyled ", "red ", "blue ", "italic ", "final"], all_content);
/// ```
#[derive(Debug)]
pub struct StackFlattener<F> {
    style_stack: Vec<MessageStyle>,
    consumer: F,
}

impl<F> StackFlattener<F>
where
    F: FnMut(&str, MessageStyle),
{
    /// Creates a new flattener with an empty style stack, and taking in the consumer that is
    /// called when content is encountered.
    pub fn new(consumer: F) -> Self {
        Self {
            style_stack: Vec::new(),
            consumer,
        }
    }
}

impl<F> MessageFlattener for StackFlattener<F>
where
    F: FnMut(&str, MessageStyle),
{
    fn push_style(&mut self, style: MessageStyle) {
        self.style_stack.push(
            self.style_stack
                .last()
                .unwrap_or(&MessageStyle::default())
                .merged_from(style),
        );
    }

    fn content(&mut self, content: &str) {
        let default_style = MessageStyle::default();
        let style = self.style_stack.last().unwrap_or(&default_style);
        (self.consumer)(content, *style);
    }

    fn pop_style(&mut self, _: MessageStyle) {
        self.style_stack.pop();
    }
}
