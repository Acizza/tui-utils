# Changelog

## To Be Released

### Features

* Added new `SimpleTable` widget. On (very) naive benchmarks, it's roughly ~10x faster than [tui::widgets::Table](https://docs.rs/tui/0.14.0/tui/widgets/struct.Table.html).

## 0.8.0 - March 10th, 2021

### Features

* Added `layout::SimpleLayout::split` function to split layouts from a list of `BasicConstraint`'s, similarly to `tui::layout::Layout::split`. The new `BasicConstraint` enum only supports constraints that are easy & fast to solve for. Each variant is described below:
    * `Length` - A specific number of characters / lines.
    * `Percentage` - A percentage ranging from 0 to 100 of the entire given area.
    * `MinLenGrowthPcnt` - A minimum number of characters / lines that grows by a specified percentage of the entire given area.
    * `MinLenRemaining` - A minimum number of characters / lines that leaves a specified number of characters / lines left at the end.

    The `MinLenGrowthPcnt` and `MinLenRemaining` constraints are meant to be used only when the remaining constraints total size is well known. For example:

    ```rust
        [
            BasicConstraint::MinLenGrowthPcnt(5, 30),
            BasicConstraint::Percentage(15),
            BasicConstraint::Percentage(15),
        ]
    ```

    In the above example, we know that the constraints following `MinLenGrowthPcnt` total 30%, so we should use that as our growth percentage. If you use a growth percentage that doesn't line up exactly with the amount of the space the remaining constraints use up, you may get odd results. Therefore, it is recommended to only use `Percentage` constraints following `MinLenGrowthPcnt`, and `Length` constraints following `MinLenRemaining`. Manually specifying the amount of space left avoids the use of a cassowary solver.

### Breaking Changes

* Moved `grid_pos` function to `layout::RectExt` trait.

## 0.7.0 - March 5th, 2021

### Features

* Added new `layout` module to compute some simple layouts faster than `tui::layout::Layout` and perform common `tui::layout::Rect` operations via a new `RectExt` trait.

### Breaking Changes

* The `pad_rect_horiz` and `pad_rect_left` functions have been moved to the `layout::RectExt` trait.

### Fixes

* Fixed yet another overflow issue in the `SimpleText` widget.

## 0.6.0 - March 1st, 2021

### Features

* Added various utilities to complement the `SimpleList` widget in a new `list` module. These utilities make it easy to use wrapping indices / list selections, and selectable enum variants.

## 0.5.1 - February 26th, 2021

### Fixes

* Fixed various overflow issues in `TextFragments` widgets &mdash; especially when using `OverflowMode::Truncate`.

* The `SimpleText` widget will no longer render when the given rect's height is zero.

## 0.5.0 - February 23rd, 2021

### Breaking Changes

* Replaced `UnicodeSupport` boolean parameter in `Fragment::Span` with `SpanOptions`. Unicode support is now always enabled, as there isn't a noticeable performance difference.

* `SimpleText::new()` now accepts any type that can be converted to a `tui::text::Span`. Since `tui::text::Span` also contains the style for the text, `SimpleText::style()` has been removed.

### Features

* Added `SpanOptions` struct to control what happens when spans overflow in the SimpleText and TextFragments widget.

## 0.4.1 - February 20th, 2021

### Fixes

* Fixed off-by-one error in `widgets::wrap::wrap_span_letters`.

* Fixed potential overflow in `widgets::text_fragments::TextFragments::{can_draw_at_x, can_draw_at_y}`.

## 0.4.0 - February 19th, 2021

### Breaking Changes

* The `Fragment` enum has been moved to the widgets module root.

* `Fragment::num_lines` will now always return a value >= 1.

### Features

* Added a `Log` widget to render lines of `Fragment` arrays. This is a relatively simply abstraction over the `TextFragments` widget.

* Added the ability to letter-wrap `Fragment` arrays with a new `wrap` module. This will allocate if more than 4 fragments are needed.

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