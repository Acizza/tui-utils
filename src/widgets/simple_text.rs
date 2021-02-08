use crate::alignment_offset;
use std::borrow::Cow;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    widgets::Widget,
};

/// This widget provides a fast way to draw a single line of text with a fixed style.
///
/// More complicated text can be drawn with the [`TextFragments`][`crate::widgets::text_fragments::TextFragments`] widget.
pub struct SimpleText<'a> {
    text: Cow<'a, str>,
    alignment: Alignment,
    style: Style,
}

impl<'a> SimpleText<'a> {
    #[inline]
    #[must_use]
    pub fn new<S>(text: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            text: text.into(),
            alignment: Alignment::Left,
            style: Style::default(),
        }
    }

    #[inline(always)]
    #[allow(clippy::must_use_candidate)]
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    #[inline(always)]
    #[allow(clippy::must_use_candidate)]
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a> Widget for SimpleText<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let len = self.text.len() as u16;

        if area.width < len {
            return;
        }

        let offset = alignment_offset(self.alignment, area.width, len);

        buf.set_string(area.x + offset, area.y, self.text, self.style);
    }
}
