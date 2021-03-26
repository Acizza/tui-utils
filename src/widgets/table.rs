use crate::layout::{BasicConstraint, SimpleLayout};
use tui::{
    buffer::Buffer,
    layout::{Direction, Rect},
    text::Span,
    widgets::Widget,
};
use unicode_width::UnicodeWidthStr;

type Width = u16;

/// A table widget similar to [`tui::widget::Table`](https://docs.rs/tui/0.14.0/tui/widgets/struct.Table.html).
pub struct SimpleTable<'a, I, Ref, const N: usize>
where
    I: IntoIterator<Item = Ref>,
    Ref: AsRef<[Span<'a>]>,
{
    data: I,
    layout: [BasicConstraint; N],
    header: Option<&'a [Span<'a>]>,
    selected: Option<u16>,
    highlight_symbol: Option<(Span<'a>, Width)>,
}

impl<'a, I, Ref, const N: usize> SimpleTable<'a, I, Ref, N>
where
    I: IntoIterator<Item = Ref>,
    Ref: AsRef<[Span<'a>]>,
{
    /// Returns a new [`SimpleTable`] with the given `data` and column `layout`.
    #[inline]
    pub fn new(data: I, layout: [BasicConstraint; N]) -> Self {
        Self {
            data,
            layout,
            header: None,
            selected: None,
            highlight_symbol: None,
        }
    }

    #[inline]
    pub fn header(mut self, header: &'a [Span<'a>]) -> Self {
        self.header = Some(header);
        self
    }

    #[inline]
    pub fn select<Idx>(mut self, selected: Idx) -> Self
    where
        Idx: Into<Option<u16>>,
    {
        self.selected = selected.into();
        self
    }

    #[inline]
    pub fn highlight_symbol(mut self, symbol: Span<'a>) -> Self {
        let len = symbol.content.width();
        self.highlight_symbol = Some((symbol, len as u16));
        self
    }
}

impl<'a, I, Ref, const N: usize> Widget for SimpleTable<'a, I, Ref, N>
where
    I: IntoIterator<Item = Ref>,
    Ref: AsRef<[Span<'a>]>,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let layout = SimpleLayout::new(Direction::Horizontal).split(area, self.layout);

        let offset_x = self
            .highlight_symbol
            .as_ref()
            .map_or(0, |(_, width)| *width);

        let header_offset = if let Some(columns) = self.header {
            if layout.len() != columns.len() {
                return;
            }

            for (i, column) in columns.iter().enumerate() {
                let pos = layout[i];
                let max_width = pos.width.saturating_sub(offset_x);

                if max_width == 0 {
                    break;
                }

                buf.set_span(offset_x + pos.x, pos.y, column, max_width);
            }

            1
        } else {
            0
        };

        let item_offset = match self.selected {
            Some(selected) if selected >= area.height.saturating_sub(header_offset) => {
                header_offset + (selected + 1).saturating_sub(area.height)
            }
            _ => 0,
        };

        let mut offset_y = header_offset;

        for (row_index, row) in self.data.into_iter().skip(item_offset as usize).enumerate() {
            let row_index = row_index as u16;
            let row = row.as_ref();

            if row_index + header_offset >= area.height || layout.len() != row.len() {
                break;
            }

            let is_selected = self.selected.map_or(false, |selected| {
                row_index == selected.saturating_sub(item_offset)
            });

            if let (true, Some((highlight, width))) = (is_selected, &self.highlight_symbol) {
                buf.set_span(area.x, offset_y + area.y, &highlight, *width as u16);
            }

            for (column_index, column) in row.iter().enumerate() {
                let pos = layout[column_index];
                let max_width = pos.width.saturating_sub(offset_x) as usize;

                if max_width == 0 {
                    break;
                }

                let style = match (is_selected, &self.highlight_symbol) {
                    (true, Some((highlight, _))) => highlight.style,
                    _ => column.style,
                };

                buf.set_stringn(
                    offset_x + pos.x,
                    offset_y + pos.y,
                    &column.content,
                    max_width,
                    style,
                );
            }

            offset_y += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleTable;
    use crate::layout::BasicConstraint;
    use tui::{backend::TestBackend, buffer::Buffer, layout::Rect, text::Span, Terminal};

    fn test_table<'a, I, Ref, const N: usize>(
        table: SimpleTable<'a, I, Ref, N>,
        width: u16,
        height: u16,
        expected: Buffer,
    ) where
        I: IntoIterator<Item = Ref>,
        Ref: AsRef<[Span<'a>]>,
    {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                f.render_widget(table, f.size());
            })
            .unwrap();
        terminal.backend().assert_buffer(&expected);
    }

    fn table_with_selection<'a, I, Ref>(
        data: I,
        header: &'a [Span<'a>],
        select: u16,
    ) -> SimpleTable<'a, I, Ref, 2>
    where
        I: IntoIterator<Item = Ref>,
        Ref: AsRef<[Span<'a>]>,
    {
        SimpleTable::new(
            data,
            [BasicConstraint::Length(12), BasicConstraint::Length(13)],
        )
        .header(header)
        .highlight_symbol(Span::raw(">"))
        .select(select)
    }

    fn test_data<'a>() -> [[Span<'a>; 2]; 5] {
        [
            [Span::raw("Left1"), Span::raw("Right1")],
            [Span::raw("Left2"), Span::raw("Right2")],
            [Span::raw("Left3"), Span::raw("Right3")],
            [Span::raw("Left4"), Span::raw("Right4")],
            [Span::raw("Left5"), Span::raw("Right5")],
        ]
    }

    fn test_large_data<'a>() -> [[Span<'a>; 2]; 20] {
        [
            [Span::raw("Left1"), Span::raw("Right1")],
            [Span::raw("Left2"), Span::raw("Right2")],
            [Span::raw("Left3"), Span::raw("Right3")],
            [Span::raw("Left4"), Span::raw("Right4")],
            [Span::raw("Left5"), Span::raw("Right5")],
            [Span::raw("Left6"), Span::raw("Right6")],
            [Span::raw("Left7"), Span::raw("Right7")],
            [Span::raw("Left8"), Span::raw("Right8")],
            [Span::raw("Left9"), Span::raw("Right9")],
            [Span::raw("Left10"), Span::raw("Right10")],
            [Span::raw("Left11"), Span::raw("Right11")],
            [Span::raw("Left12"), Span::raw("Right12")],
            [Span::raw("Left13"), Span::raw("Right13")],
            [Span::raw("Left14"), Span::raw("Right14")],
            [Span::raw("Left15"), Span::raw("Right15")],
            [Span::raw("Left16"), Span::raw("Right16")],
            [Span::raw("Left17"), Span::raw("Right17")],
            [Span::raw("Left18"), Span::raw("Right18")],
            [Span::raw("Left19"), Span::raw("Right19")],
            [Span::raw("Left20"), Span::raw("Right20")],
        ]
    }

    fn test_header<'a>() -> [Span<'a>; 2] {
        [Span::raw("Left Header"), Span::raw("Right Header")]
    }

    #[test]
    fn empty_table() {
        let data = [[]];

        let table = SimpleTable::new(
            &data,
            [BasicConstraint::Length(5), BasicConstraint::Length(5)],
        );

        let expected = {
            let mut buf = Buffer::default();

            buf.resize(Rect {
                x: 0,
                y: 0,
                width: 10,
                height: 10,
            });

            buf
        };

        test_table(table, 10, 10, expected);
    }

    #[test]
    fn basic_table() {
        let data = [[Span::raw("Left"), Span::raw("Right")]];

        let table = SimpleTable::new(
            &data,
            [BasicConstraint::Length(5), BasicConstraint::Length(5)],
        );

        let expected = Buffer::with_lines(vec!["Left Right"]);

        test_table(table, 10, 1, expected);
    }

    #[test]
    fn basic_multiline_table() {
        let data = test_data();

        let table = SimpleTable::new(
            &data,
            [BasicConstraint::Length(6), BasicConstraint::Length(6)],
        );

        let expected = Buffer::with_lines(vec![
            "Left1 Right1",
            "Left2 Right2",
            "Left3 Right3",
            "Left4 Right4",
            "Left5 Right5",
        ]);

        test_table(table, 12, 5, expected);
    }

    #[test]
    fn table_header() {
        let data = test_data();
        let header = test_header();

        let table = SimpleTable::new(
            &data,
            [
                BasicConstraint::Percentage(50),
                BasicConstraint::Percentage(50),
            ],
        )
        .header(&header);

        let expected = Buffer::with_lines(vec![
            "Left Header Right Header",
            "Left1       Right1",
            "Left2       Right2",
            "Left3       Right3",
            "Left4       Right4",
            "Left5       Right5",
        ]);

        test_table(table, 24, 6, expected);
    }

    #[test]
    fn table_header_with_beginning_selection() {
        let data = test_data();
        let header = test_header();
        let table = table_with_selection(&data, &header, 0);

        let expected = Buffer::with_lines(vec![
            " Left Header Right Header",
            ">Left1       Right1",
            " Left2       Right2",
            " Left3       Right3",
            " Left4       Right4",
            " Left5       Right5",
        ]);

        test_table(table, 25, 6, expected);
    }

    #[test]
    fn table_header_with_mid_selection() {
        let data = test_data();
        let header = test_header();
        let table = table_with_selection(&data, &header, 2);

        let expected = Buffer::with_lines(vec![
            " Left Header Right Header",
            " Left1       Right1",
            " Left2       Right2",
            ">Left3       Right3",
            " Left4       Right4",
            " Left5       Right5",
        ]);

        test_table(table, 25, 6, expected);
    }

    #[test]
    fn table_header_with_end_selection() {
        let data = test_data();
        let header = test_header();
        let table = table_with_selection(&data, &header, 4);

        let expected = Buffer::with_lines(vec![
            " Left Header Right Header",
            " Left1       Right1",
            " Left2       Right2",
            " Left3       Right3",
            " Left4       Right4",
            ">Left5       Right5",
        ]);

        test_table(table, 25, 6, expected);
    }

    #[test]
    fn table_header_with_scrolling_edge_selection() {
        let data = test_large_data();
        let header = test_header();
        let table = table_with_selection(&data, &header, 10);

        let expected = Buffer::with_lines(vec![
            " Left Header Right Header",
            " Left7       Right7      ",
            " Left8       Right8      ",
            " Left9       Right9      ",
            " Left10      Right10     ",
            ">Left11      Right11     ",
        ]);

        test_table(table, 25, 6, expected);
    }

    #[test]
    fn table_header_with_scrolling_mid_selection() {
        let data = test_large_data();
        let header = test_header();
        let table = table_with_selection(&data, &header, 14);

        let expected = Buffer::with_lines(vec![
            " Left Header Right Header",
            " Left11      Right11     ",
            " Left12      Right12     ",
            " Left13      Right13     ",
            " Left14      Right14     ",
            ">Left15      Right15     ",
        ]);

        test_table(table, 25, 6, expected);
    }

    #[test]
    fn table_header_with_scrolling_end_selection() {
        let data = test_large_data();
        let header = test_header();
        let table = table_with_selection(&data, &header, 19);

        let expected = Buffer::with_lines(vec![
            " Left Header Right Header",
            " Left16      Right16     ",
            " Left17      Right17     ",
            " Left18      Right18     ",
            " Left19      Right19     ",
            ">Left20      Right20     ",
        ]);

        test_table(table, 25, 6, expected);
    }

    #[test]
    fn table_header_with_scrolling_overflow_selection() {
        let data = test_large_data();
        let header = test_header();
        let table = table_with_selection(&data, &header, 21);

        let expected = Buffer::with_lines(vec![
            " Left Header Right Header",
            " Left18      Right18     ",
            " Left19      Right19     ",
            " Left20      Right20     ",
            "                         ",
            "                         ",
        ]);

        test_table(table, 25, 6, expected);
    }

    #[test]
    fn table_right_header_cutoff() {
        let data = test_data();
        let header = test_header();

        let table = SimpleTable::new(
            &data,
            [BasicConstraint::Length(12), BasicConstraint::Length(13)],
        )
        .header(&header)
        .highlight_symbol(Span::raw(">"))
        .select(Some(2));

        let expected = Buffer::with_lines(vec![
            " Left Header Right H",
            " Left1       Right1",
            " Left2       Right2",
            ">Left3       Right3",
            " Left4       Right4",
            " Left5       Right5",
        ]);

        test_table(table, 20, 6, expected);
    }

    #[test]
    fn table_right_data_cutoff() {
        let data = test_data();
        let header = test_header();

        let table = SimpleTable::new(
            &data,
            [BasicConstraint::Length(12), BasicConstraint::Length(13)],
        )
        .header(&header)
        .highlight_symbol(Span::raw(">"))
        .select(Some(2));

        let expected = Buffer::with_lines(vec![
            " Left Header Righ",
            " Left1       Righ",
            " Left2       Righ",
            ">Left3       Righ",
            " Left4       Righ",
            " Left5       Righ",
        ]);

        test_table(table, 17, 6, expected);
    }

    #[test]
    fn table_left_data_cutoff() {
        let data = test_data();
        let header = test_header();

        let table = SimpleTable::new(
            &data,
            [BasicConstraint::Length(12), BasicConstraint::Length(13)],
        )
        .header(&header)
        .highlight_symbol(Span::raw(">"))
        .select(Some(2));

        let expected = Buffer::with_lines(vec![" Lef", " Lef", " Lef", ">Lef", " Lef", " Lef"]);

        test_table(table, 4, 6, expected);
    }

    #[test]
    fn table_full_cutoff() {
        let data = test_data();
        let header = test_header();

        let table = SimpleTable::new(
            &data,
            [BasicConstraint::Length(12), BasicConstraint::Length(13)],
        )
        .header(&header)
        .highlight_symbol(Span::raw(">"))
        .select(Some(2));

        let expected = {
            let mut buf = Buffer::default();

            buf.resize(Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 6,
            });

            buf
        };

        test_table(table, 0, 6, expected);
    }
}
