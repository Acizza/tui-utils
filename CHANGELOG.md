# Changelog

## To Be Released

## 0.3.0 - February 8th, 2021

### Breaking Changes

* The following functions have had the #[must_use] attribute applied to them:
    * `alignment_offset`
    * `pad_rect_horiz`
    * `pad_rect_left`
    * `fill_area`
    * `grid_pos`
    * `widgets::simple_text::SimpleText::new`
    * `widgets::simple_list::SimpleList::new`
    * `widgets::text_fragments::TextFragments::new`
    * `widgets::text_fragments::Fragment::total_len`
    * `widgets::text_fragments::Fragment::line_items`
    * `widgets::text_fragments::Fragment::line_len`
    * `widgets::text_fragments::Fragment::num_lines`
    * `widgets::text_fragments::Fragment::len`

### Features

* Added `SimpleList` widget to render lists with items given via an iterator.

## 0.2.1 - February 5th, 2021

### Fixes

* Disabled default features from the `tui` crate. This prevents termion from becoming a dependency.

## 0.2.0 - February 5th, 2021

### Breaking Changes

* The `Text` variant of the `Fragment` enum in the `TextFragments` widget has been replaced with a new variant named `Span` that takes a [tui::text::Span](https://docs.rs/tui/0.14.0/tui/text/struct.Span.html) and a boolean indicating whether or not unicode should be supported.
* Nesting support in the `TextFragments` widget has been removed. With the addition of multiline support, this feature became too difficult to do efficiently. Because of this, the `FragmentedWidget` trait was also removed.
* The `text_fragments` macro has been removed.

### Features

* The `TextFragments` widget now supports multiple lines. Lines can be inserted in fragment arrays with the `Fragment::Line` enum variant.
* Added `line_items`, `line_len`, and `num_lines` functions to the `Fragment` enum.
* Added `grid_pos` function to efficiently calculate grid-based positions.