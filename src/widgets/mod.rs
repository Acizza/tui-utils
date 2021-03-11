pub mod list;
pub mod log;
pub mod table;
pub mod text;
pub mod text_fragments;

pub use list::SimpleList;
pub use log::Log;
pub use table::SimpleTable;
pub use text::SimpleText;
pub use text_fragments::TextFragments;

use tui::{style::Style, text::Span};

#[derive(Debug, Clone)]
pub enum Fragment<'a> {
    Span(Span<'a>, SpanOptions),
    Char(char, Style),
    Line,
}

#[allow(clippy::len_without_is_empty)]
impl<'a> Fragment<'a> {
    #[inline]
    pub fn span<S>(span: S) -> Self
    where
        S: Into<Span<'a>>,
    {
        Self::Span(span.into(), SpanOptions::default())
    }

    /// Calculate the total length of each given item, including between lines.
    #[inline]
    #[must_use]
    pub fn total_len<I>(items: I) -> u16
    where
        I: IntoIterator<Item = &'a Self>,
    {
        items.into_iter().fold(0, |acc, item| acc + item.len())
    }

    /// Returns an iterator over all of the given items on the current line.
    #[inline]
    pub fn line_items<I>(items: I) -> impl Iterator<Item = &'a Fragment<'a>>
    where
        I: IntoIterator<Item = &'a Self>,
    {
        items.into_iter().take_while(|item| !Self::is_line(item))
    }

    /// Returns the total length of all items from the given items.
    #[inline]
    #[must_use]
    pub fn line_len<I>(items: I) -> u16
    where
        I: IntoIterator<Item = &'a Self>,
    {
        Self::total_len(Self::line_items(items))
    }

    /// Returns the total number of lines in the given items.
    #[inline]
    #[must_use]
    pub fn num_lines<I>(items: I) -> u16
    where
        I: IntoIterator<Item = &'a Self>,
    {
        1 + items.into_iter().filter(|item| Self::is_line(item)).count() as u16
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> u16 {
        match self {
            Self::Span(span, _) => span.width() as u16,
            Self::Char(_, _) => 1,
            Self::Line => 0,
        }
    }

    fn is_line(&self) -> bool {
        matches!(self, Self::Line)
    }
}

/// Options for a particular span of text.
#[derive(Debug, Clone, Copy)]
pub struct SpanOptions {
    /// Controls what happens when the span is larger than its render area.
    pub overflow: OverflowMode,
}

impl SpanOptions {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    #[must_use]
    pub fn overflow(mut self, overflow: OverflowMode) -> Self {
        self.overflow = overflow;
        self
    }
}

impl Default for SpanOptions {
    fn default() -> Self {
        Self {
            overflow: OverflowMode::default(),
        }
    }
}

/// An action to perform on a span overflow.
#[derive(Debug, Clone, Copy)]
pub enum OverflowMode {
    /// Don't render the span at all.
    Hide,
    /// Render only the visible part of the span.
    Truncate,
}

impl Default for OverflowMode {
    fn default() -> Self {
        Self::Hide
    }
}
