use super::Fragment;
use crate::alignment_offset;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::Span,
    widgets::Widget,
};

/// Draw fragments of text with different styles across multiple lines.
///
/// This serves as an alternative for `tui::widget::Paragraph`.
/// It is meant to be used for simple text layouts that don't need scrolling.
///
/// If you only need to draw a single line of text with one style, consider using [`SimpleText`][`crate::widgets::simple_text::SimpleText`] instead.
pub struct TextFragments<'a> {
    items: &'a [Fragment<'a>],
    alignment: Alignment,
}

impl<'a> TextFragments<'a> {
    #[inline]
    #[must_use]
    pub fn new(items: &'a [Fragment<'a>]) -> Self {
        Self {
            items,
            alignment: Alignment::Left,
        }
    }

    #[inline(always)]
    #[allow(clippy::must_use_candidate)]
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    fn can_draw_at_x(area: Rect, x: u16) -> bool {
        x <= area.width
    }

    fn can_draw_at_y(area: Rect, y: u16) -> bool {
        y < area.height
    }
}

impl<'a> Widget for TextFragments<'a> {
    #[inline]
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let mut item_offset = 0;
        let mut offset_y = 0;

        'outer: while item_offset <= self.items.len() {
            let line_items = {
                let slice = &self.items[item_offset..];

                let next_line_pos = slice
                    .iter()
                    .position(Fragment::is_line)
                    .unwrap_or(slice.len());

                // Our next item slice needs to be the item after the line fragment
                item_offset += next_line_pos + 1;

                &slice[..next_line_pos]
            };

            let mut offset_x =
                alignment_offset(self.alignment, area.width, Fragment::total_len(line_items));

            for item in line_items {
                let start_x = area.x + offset_x;
                let start_y = area.y + offset_y;
                let len = item.len();

                if !Self::can_draw_at_x(area, offset_x + len) {
                    break 'outer;
                }

                match item {
                    Fragment::Span(Span { content, style }) => {
                        buf.set_string(start_x, start_y, content, *style)
                    }
                    Fragment::Char(ch, style) => {
                        buf.get_mut(start_x, start_y)
                            .set_char(*ch)
                            .set_style(*style);
                    }
                    Fragment::Line => break,
                }

                offset_x += len;
            }

            offset_y += 1;

            if !Self::can_draw_at_y(area, offset_y) {
                break;
            }
        }
    }
}
