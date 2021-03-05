#![warn(clippy::pedantic)]
#![allow(clippy::clippy::cast_possible_truncation)]
#![allow(clippy::inline_always)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::shadow_unrelated)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::module_name_repetitions)]

use tui::{
    buffer::{Buffer, Cell},
    layout::{Alignment, Rect},
};

pub mod layout;
pub mod list;
pub mod widgets;
pub mod wrap;

#[inline]
#[must_use]
pub fn alignment_offset(alignment: Alignment, total_len: u16, item_len: u16) -> u16 {
    match alignment {
        Alignment::Left => 0,
        Alignment::Center => (total_len / 2).saturating_sub(item_len / 2),
        Alignment::Right => total_len.saturating_sub(item_len),
    }
}

#[inline]
pub fn fill_area<F>(area: Rect, buf: &mut Buffer, func: F)
where
    F: Fn(&mut Cell),
{
    for x in 0..area.width {
        for y in 0..area.height {
            func(buf.get_mut(area.x + x, area.y + y))
        }
    }
}

/// Returns a calculated grid position.
///
/// The `x` and `y` fields on the `dimensions` Rect should be used to indicate the x and y coordinates on the grid.
#[inline]
#[must_use]
pub fn grid_pos(dimensions: Rect, container: Rect) -> Rect {
    Rect {
        x: container.x + (dimensions.width * dimensions.x),
        y: container.y + (dimensions.height * dimensions.y),
        width: dimensions.width,
        height: dimensions.height,
    }
}
