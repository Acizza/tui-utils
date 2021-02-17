use std::marker::PhantomData;

use super::{Fragment, TextFragments};
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::Widget,
};

/// A widget to render a log of items vertically.
pub struct Log<'a, I, Ref>
where
    I: DoubleEndedIterator<Item = Ref>,
    Ref: AsRef<[Fragment<'a>]>,
{
    items: I,
    alignment: Alignment,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, I, Ref> Log<'a, I, Ref>
where
    I: DoubleEndedIterator<Item = Ref>,
    Ref: AsRef<[Fragment<'a>]>,
{
    /// Create a new [`Log`] with the given item iterator.
    #[inline]
    #[must_use]
    pub fn new(items: I) -> Self {
        Self {
            items,
            alignment: Alignment::Left,
            _phantom: PhantomData,
        }
    }

    #[inline(always)]
    #[allow(clippy::must_use_candidate)]
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

impl<'a, I, Ref> Widget for Log<'a, I, Ref>
where
    I: DoubleEndedIterator<Item = Ref>,
    Ref: AsRef<[Fragment<'a>]>,
{
    #[inline]
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let mut y_pos = area.height;

        let visible_items = {
            let mut total_len = 0;

            self.items.into_iter().rev().take_while(move |item| {
                let len = Fragment::num_lines(item.as_ref());
                total_len += len;

                total_len <= area.height
            })
        };

        for item in visible_items {
            let item = item.as_ref();
            let num_lines = Fragment::num_lines(item);
            let item_y_pos = y_pos.saturating_sub(num_lines);

            let fragments = TextFragments::new(item).alignment(self.alignment);

            let pos = Rect {
                y: area.y + item_y_pos,
                height: num_lines,
                ..area
            };

            fragments.render(pos, buf);

            y_pos -= num_lines;
        }
    }
}
