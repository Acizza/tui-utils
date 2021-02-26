use crate::alignment_offset;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::Span,
    widgets::Widget,
};

use super::OverflowMode;

/// This widget provides a fast way to draw a single line of text with a fixed style.
///
/// More complicated text can be drawn with the [`TextFragments`][`crate::widgets::text_fragments::TextFragments`] widget.
pub struct SimpleText<'a> {
    span: Span<'a>,
    alignment: Alignment,
    overflow: OverflowMode,
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
            overflow: OverflowMode::default(),
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
    pub fn overflow(mut self, overflow: OverflowMode) -> Self {
        self.overflow = overflow;
        self
    }
}

impl<'a> Widget for SimpleText<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 {
            return;
        }

        let len = self.span.width() as u16;
        let offset = alignment_offset(self.alignment, area.width, len);

        let max_width = if area.width < len {
            match self.overflow {
                OverflowMode::Hide => return,
                OverflowMode::Truncate => area.width.saturating_sub(offset.saturating_sub(len)),
            }
        } else {
            len
        };

        buf.set_stringn(
            area.x + offset,
            area.y,
            self.span.content,
            max_width as usize,
            self.span.style,
        );
    }
}
