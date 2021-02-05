use crate::alignment_offset;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::Span,
    widgets::Widget,
};

type LineItems<'a> = &'a [Fragment<'a>];

/// Draw fragments of text with different styles across multiple lines.
///
/// This serves as an alternative for `tui::widget::Paragraph`.
/// It is meant to be used for simple text layouts that don't need scrolling.
///
/// If you only need to draw a single line of text with one style, consider using [SimpleText][`crate::widgets::simple_text::SimpleText`] instead.
pub struct TextFragments<'a> {
    items: &'a [LineItems<'a>],
    alignment: Alignment,
}

impl<'a> TextFragments<'a> {
    #[inline]
    pub fn new(items: &'a [LineItems<'a>]) -> Self {
        Self {
            items,
            alignment: Alignment::Left,
        }
    }

    #[inline(always)]
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    fn can_draw_at_x(area: Rect, x: u16) -> bool {
        x <= area.right()
    }

    fn can_draw_at_y(area: Rect, y: u16) -> bool {
        y <= area.top()
    }
}

impl<'a> Widget for TextFragments<'a> {
    #[inline]
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        for (offset_y, line_items) in self.items.into_iter().enumerate() {
            let mut offset_x =
                alignment_offset(self.alignment, area.width, Fragment::total_len(line_items));

            for item in *line_items {
                let start_x = area.x + offset_x;
                let start_y = area.y + offset_y as u16;

                match item {
                    Fragment::AsciiSpan(span) => {
                        let len = span.content.len() as u16;

                        if !Self::can_draw_at_x(area, start_x + len) {
                            return;
                        }

                        buf.set_string(start_x, start_y, &span.content, span.style);
                        offset_x += len;
                    }
                    Fragment::UnicodeSpan(span) => {
                        let len = span.width() as u16;

                        if !Self::can_draw_at_x(area, start_x + len) {
                            return;
                        }

                        buf.set_string(start_x, start_y, &span.content, span.style);
                        offset_x += len;
                    }
                    Fragment::Char(ch, style) => {
                        if !Self::can_draw_at_x(area, start_x) {
                            return;
                        }

                        buf.get_mut(start_x, start_y)
                            .set_char(*ch)
                            .set_style(*style);

                        offset_x += 1;
                    }
                }
            }

            if !Self::can_draw_at_y(area, offset_y as u16) {
                break;
            }
        }
    }
}

pub enum Fragment<'a> {
    AsciiSpan(Span<'a>),
    UnicodeSpan(Span<'a>),
    Char(char, Style),
}

impl<'a> Fragment<'a> {
    /// Calculate the total length of each given item.
    #[inline]
    pub fn total_len(items: &[Self]) -> u16 {
        items.iter().fold(0, |acc, item| match item {
            Self::AsciiSpan(span) => acc + span.content.len() as u16,
            Self::UnicodeSpan(span) => acc + span.width() as u16,
            Self::Char(_, _) => acc + 1,
        })
    }
}

impl<'a> From<(char, Style)> for Fragment<'a> {
    fn from((ch, style): (char, Style)) -> Self {
        Self::Char(ch, style)
    }
}

/// Represents a widget that can be rendered as text fragments.
pub trait FragmentedWidget {
    /// Returns a reference to every text fragment.
    ///
    /// The [`text_fragments`] macro can be used in some cases to build the array.
    fn fragments(&self) -> &[LineItems];
}
