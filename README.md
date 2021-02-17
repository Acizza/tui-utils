# tui-utils

This is a small Rust library that adds several widgets and utilities to micro-optimize the rendering time of applications using the [tui](https://github.com/fdehau/tui-rs) crate.
There are currently four widgets provided by this library, all of which are non-allocating.

The following list of widgets are meant to be used as fast substitutes for an equivalent [tui](https://github.com/fdehau/tui-rs) widget:

* `SimpleText` - Renders a single line of text with one style. Substitute for [tui::widgets::Paragraph](https://docs.rs/tui/0.14.0/tui/widgets/struct.Paragraph.html).
* `TextFragments` - Renders fragments of text with different styles across multiple lines. Substitute for [tui::widgets::Paragraph](https://docs.rs/tui/0.14.0/tui/widgets/struct.Paragraph.html).
* `SimpleList` - Renders a given item iterator as a vertical list. Substitute for [tui::widgets::List](https://docs.rs/tui/0.14.0/tui/widgets/struct.List.html).

And these widgets are unique to this library:

* `Log` - Renders items in a vertical log. This is only a simple abstraction over the `TextFragments` widget to make further abstraction easier.
