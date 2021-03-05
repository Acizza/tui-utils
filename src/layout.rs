use tui::layout::{Direction, Rect};

/// Build simple layouts much faster than [`tui::layout::Layout`](https://docs.rs/tui/0.14.0/tui/layout/struct.Layout.html) and without ever allocating.
#[derive(Clone)]
pub struct SimpleLayout {
    direction: Direction,
    margin_x: u16,
    margin_y: u16,
}

impl SimpleLayout {
    #[inline]
    #[must_use]
    pub const fn new(direction: Direction) -> Self {
        Self {
            direction,
            margin_x: 0,
            margin_y: 0,
        }
    }

    #[inline]
    #[must_use]
    pub fn horizontal_margin(mut self, margin: u16) -> Self {
        self.margin_x = margin;
        self
    }

    #[inline]
    #[must_use]
    pub fn vertical_margin(mut self, margin: u16) -> Self {
        self.margin_y = margin;
        self
    }

    #[inline]
    #[must_use]
    pub fn margin(mut self, margin: u16) -> Self {
        self.margin_x = margin;
        self.margin_y = margin;
        self
    }

    fn get_padded(&self, area: Rect) -> Rect {
        area.pad(self.margin_x, self.margin_y)
    }

    /// Build a layout with two evenly split cells.
    #[inline]
    #[must_use]
    pub fn split_evenly(self, area: Rect) -> EvenSplit {
        let area = self.get_padded(area);
        let gen_rect = GenericRect::from_dir(&self.direction, area);

        let half_size = gen_rect.size / 2;

        let left = GenericRect {
            size: half_size,
            ..gen_rect
        };

        let right = GenericRect {
            pos: gen_rect.pos + half_size,
            size: half_size,
        };

        EvenSplit {
            left: left.as_rect(&self.direction, area),
            right: right.as_rect(&self.direction, area),
        }
    }

    /// Build a layout consisting of four evenly sized cells.
    #[inline]
    #[must_use]
    pub fn split_quarters(self, area: Rect) -> QuarterSplit {
        let area = self.get_padded(area);
        let gen_rect = GenericRect::from_dir(&self.direction, area);

        let quarter_size = gen_rect.size / 4;

        let quarter = |offset| {
            let rect = GenericRect {
                pos: gen_rect.pos + (quarter_size * offset),
                size: quarter_size,
            };

            rect.as_rect(&self.direction, area)
        };

        QuarterSplit {
            first: quarter(0),
            second: quarter(1),
            third: quarter(2),
            fourth: quarter(3),
        }
    }

    /// Build a layout consisting of four evenly sized quadrants.
    #[inline]
    #[must_use]
    pub fn split_quadrants(self, area: Rect) -> QuadrantSplit {
        let area = self.get_padded(area);

        let half_width = area.width / 2;
        let half_height = area.height / 2;

        let half_size_rect = |x, y| Rect {
            x,
            y,
            width: half_width,
            height: half_height,
        };

        QuadrantSplit {
            top_left: half_size_rect(area.x, area.y),
            top_right: half_size_rect(area.x + half_width, area.y),
            bottom_left: half_size_rect(area.x, area.y + half_height),
            bottom_right: half_size_rect(area.x + half_width, area.y + half_height),
        }
    }
}

impl Default for SimpleLayout {
    fn default() -> Self {
        Self {
            direction: Direction::Horizontal,
            margin_x: 0,
            margin_y: 0,
        }
    }
}

#[derive(Copy, Clone)]
struct GenericRect {
    pos: u16,
    size: u16,
}

impl GenericRect {
    const fn new(pos: u16, size: u16) -> Self {
        Self { pos, size }
    }

    const fn from_dir(dir: &Direction, area: Rect) -> Self {
        match dir {
            Direction::Horizontal => Self::new(area.x, area.width),
            Direction::Vertical => Self::new(area.y, area.height),
        }
    }

    const fn as_rect(self, dir: &Direction, area: Rect) -> Rect {
        match dir {
            Direction::Horizontal => self.as_horiz_rect(area),
            Direction::Vertical => self.as_vert_rect(area),
        }
    }

    const fn as_horiz_rect(self, area: Rect) -> Rect {
        Rect {
            x: self.pos,
            width: self.size,
            ..area
        }
    }

    const fn as_vert_rect(self, area: Rect) -> Rect {
        Rect {
            y: self.pos,
            height: self.size,
            ..area
        }
    }
}

/// An evenly split layout.
#[derive(Clone)]
pub struct EvenSplit {
    /// The left / top cell.
    pub left: Rect,
    /// The right / bottom cell.
    pub right: Rect,
}

/// A layout consisting of four evenly sized cells.
#[derive(Clone)]
pub struct QuarterSplit {
    pub first: Rect,
    pub second: Rect,
    pub third: Rect,
    pub fourth: Rect,
}

/// A layout consisting of four evenly sized quadrants.
#[derive(Clone)]
pub struct QuadrantSplit {
    pub top_left: Rect,
    pub top_right: Rect,
    pub bottom_left: Rect,
    pub bottom_right: Rect,
}

/// Extensions to modify [`Rect`](https://docs.rs/tui/0.14.0/tui/layout/struct.Rect.html) structs.
pub trait RectExt: Sized {
    fn pad(self, padding_x: u16, padding_y: u16) -> Rect;
    fn pad_horiz(self, padding: u16) -> Rect;
    fn pad_left(self, padding: u16) -> Rect;
    fn pad_vert(self, padding: u16) -> Rect;

    /// Returns a rect spanning from the top of the given `rect` down to the number of `lines` specified.
    fn lines_from_top(self, lines: u16) -> Rect;
    /// Returns a rect spanning from the bottom of the given `rect` up to the number of `lines` specified.
    fn lines_from_bottom(self, lines: u16) -> Rect;
}

impl RectExt for Rect {
    fn pad(self, padding_x: u16, padding_y: u16) -> Self {
        Self {
            x: self.x + padding_x,
            y: self.y + padding_y,
            width: self.width.saturating_sub(padding_x * 2),
            height: self.height.saturating_sub(padding_y * 2),
        }
    }

    fn pad_horiz(self, padding: u16) -> Self {
        Self {
            x: self.x + padding,
            width: self.width.saturating_sub(padding * 2),
            ..self
        }
    }

    fn pad_left(self, padding: u16) -> Self {
        Self {
            x: self.x + padding,
            width: self.width.saturating_sub(padding),
            ..self
        }
    }

    fn pad_vert(self, padding: u16) -> Self {
        Self {
            y: self.y + padding,
            height: self.height.saturating_sub(padding * 2),
            ..self
        }
    }

    fn lines_from_top(self, lines: u16) -> Self {
        Self {
            height: lines.min(self.height),
            ..self
        }
    }

    fn lines_from_bottom(self, lines: u16) -> Self {
        let max_height = self.height.saturating_sub(lines);

        Self {
            y: self.y + max_height,
            height: lines.min(max_height),
            ..self
        }
    }
}
