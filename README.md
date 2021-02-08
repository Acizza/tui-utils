# tui-utils

This is a small Rust library that adds several widgets and utilities to micro-optimize the rendering time of applications using the [tui](https://github.com/fdehau/tui-rs) crate.

There are currently three widgets provided by this library &mdash; all of which are non-allocating and are fast substitutes to an equivalent [tui](https://github.com/fdehau/tui-rs) widget:

* `SimpleText` - Renders a single line of text with one style. Substitute for [tui::widgets::Paragraph](https://docs.rs/tui/0.14.0/tui/widgets/struct.Paragraph.html).
* `TextFragments` - Renders fragments of text with different styles across multiple lines. Substitute for [tui::widgets::Paragraph](https://docs.rs/tui/0.14.0/tui/widgets/struct.Paragraph.html).
* `SimpleList` - Renders a given item iterator as a vertical list. Substitute for [tui::widgets::List](https://docs.rs/tui/0.14.0/tui/widgets/struct.List.html).