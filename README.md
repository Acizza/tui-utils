# tui-utils

This is a small Rust library that adds several basic widgets and utilities to complement the [tui](https://github.com/fdehau/tui-rs) crate.
It is only really useful if you are trying to micro-optimize the rendering time of a TUI application.

There are currently two widgets provided by this library &mdash; both of which are non-allocating and are fast alternatives to the [tui::widgets::Paragraph](https://docs.rs/tui/0.14.0/tui/widgets/struct.Paragraph.html) widget:

* `SimpleText` - Renders a single line of text with one style.
* `TextFragments` - Renders a single line of text with multiple styles. Supports nesting via the `FragmentedWidget` trait.