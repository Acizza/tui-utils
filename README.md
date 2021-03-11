# tui-utils

![Continuous Integration](https://github.com/Acizza/tui-utils/workflows/Continuous%20Integration/badge.svg)
[![total lines](https://tokei.rs/b1/github/acizza/tui-utils)](https://github.com/acizza/tui-utils)

This is a small Rust library that adds several widgets and utilities to micro-optimize the rendering time of applications using the [tui](https://github.com/fdehau/tui-rs) crate.
There are currently five widgets provided by this library, all of which are non-allocating (unless otherwise specified).

The following list of widgets are meant to be used as fast substitutes for an equivalent [tui](https://github.com/fdehau/tui-rs) widget:

* `SimpleText` - Renders a single line of text with one style. Substitute for [tui::widgets::Paragraph](https://docs.rs/tui/0.14.0/tui/widgets/struct.Paragraph.html).
* `TextFragments` - Renders fragments of text with different styles across multiple lines. Substitute for [tui::widgets::Paragraph](https://docs.rs/tui/0.14.0/tui/widgets/struct.Paragraph.html).
* `SimpleList` - Renders a given item iterator as a vertical list. Substitute for [tui::widgets::List](https://docs.rs/tui/0.14.0/tui/widgets/struct.List.html).
* `SimpleTable` - Renders a table. Allocates if more than 4 columns are needed. Substitute for [tui::widgets::Table](https://docs.rs/tui/0.14.0/tui/widgets/struct.Table.html).

And these widgets are unique to this library:

* `Log` - Renders items in a vertical log. This is only a simple abstraction over the `TextFragments` widget to make further abstraction easier.
