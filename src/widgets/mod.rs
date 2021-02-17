pub mod log;
pub mod simple_list;
pub mod simple_text;
pub mod text_fragments;

pub use log::Log;
pub use simple_list::SimpleList;
pub use simple_text::SimpleText;
pub use text_fragments::TextFragments;

use tui::{style::Style, text::Span};

type UnicodeSupport = bool;

#[derive(Debug, Clone)]
pub enum Fragment<'a> {
    Span(Span<'a>, UnicodeSupport),
    Char(char, Style),
    Line,
}

#[allow(clippy::len_without_is_empty)]
impl<'a> Fragment<'a> {
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
    #[must_use]
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
        items.into_iter().filter(|item| Self::is_line(item)).count() as u16
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> u16 {
        match self {
            Self::Span(span, false) => span.content.len() as u16,
            Self::Span(span, true) => span.width() as u16,
            Self::Char(_, _) => 1,
            Self::Line => 0,
        }
    }

    fn is_line(&self) -> bool {
        matches!(self, Self::Line)
    }
}
