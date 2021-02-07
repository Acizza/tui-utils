use tui::{buffer::Buffer, layout::Rect, text::Span, widgets::Widget};
use unicode_width::UnicodeWidthStr;

type Width = u16;

/// A list widget similar to `tui::widget::List`.
///
/// Unlike the `tui::widget::List` widget, this widget takes an iterator to represent its items.
pub struct SimpleList<'a, I>
where
    I: IntoIterator<Item = Span<'a>>,
{
    items: I,
    selected: Option<u16>,
    highlight_symbol: Option<(Span<'a>, Width)>,
}

impl<'a, I> SimpleList<'a, I>
where
    I: IntoIterator<Item = Span<'a>>,
{
    /// Create a new [`SimpleList`] with the given item iterator.
    #[inline]
    pub fn new(items: I) -> Self {
        Self {
            items,
            selected: None,
            highlight_symbol: None,
        }
    }

    /// Set which item index is selected.
    #[inline]
    pub fn select<Idx>(mut self, index: Idx) -> Self
    where
        Idx: Into<Option<u16>>,
    {
        self.selected = index.into();
        self
    }

    /// Set the span to render on the left of the selected list item.
    #[inline]
    pub fn highlight_symbol(mut self, symbol: Span<'a>) -> Self {
        let len = symbol.content.width();
        self.highlight_symbol = Some((symbol, len as u16));
        self
    }
}

impl<'a, I> Widget for SimpleList<'a, I>
where
    I: IntoIterator<Item = Span<'a>>,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let y_offset = match self.selected {
            Some(selected) if selected >= area.height => (selected + 1).saturating_sub(area.height),
            _ => 0,
        };

        let x_offset = if let Some((_, width)) = &self.highlight_symbol {
            *width
        } else {
            0
        };

        for (i, item) in self.items.into_iter().skip(y_offset as usize).enumerate() {
            let i = i as u16;

            if i >= area.height {
                break;
            }

            let is_selected = if let Some(selected) = self.selected {
                i == selected.saturating_sub(y_offset)
            } else {
                false
            };

            let y_pos = area.y + i;
            let max_width = area.width - x_offset;

            let style = match (is_selected, &self.highlight_symbol) {
                (true, Some((highlight, _))) => {
                    buf.set_span(area.x, y_pos, &highlight, max_width);
                    highlight.style
                }
                _ => item.style,
            };

            buf.set_stringn(
                area.x + x_offset,
                y_pos,
                item.content,
                max_width as usize,
                style,
            );
        }
    }
}
