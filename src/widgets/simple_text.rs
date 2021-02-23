use crate::alignment_offset;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::Span,
    widgets::Widget,
};

/// This widget provides a fast way to draw a single line of text with a fixed style.
///
/// More complicated text can be drawn with the [`TextFragments`][`crate::widgets::text_fragments::TextFragments`] widget.
pub struct SimpleText<'a> {
    span: Span<'a>,
    alignment: Alignment,
}

impl<'a> SimpleText<'a> {
    #[inline]
    #[must_use]
    pub fn new<S>(span: S) -> Self
    where
        S: Into<Span<'a>>,
    {
        Self {
            span: span.into(),
            alignment: Alignment::Left,
        }
    }

    #[inline(always)]
    #[allow(clippy::must_use_candidate)]
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

impl<'a> Widget for SimpleText<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let len = self.span.width() as u16;

        if area.width < len {
            return;
        }

        let offset = alignment_offset(self.alignment, area.width, len);

        buf.set_string(area.x + offset, area.y, self.span.content, self.span.style);
    }
}
