use crate::widgets::{Fragment, SpanOptions};
use smallvec::SmallVec;
use tui::text::Span;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Wrap the given `fragments` iterator by its letters to fit within the given `area_width`.
///
/// Returns a new `SmallVec` containing the given `fragments` with `Fragment::Line`'s inserted at appropriate places.
/// This only operates on the `Fragment::Span` variant, so long lists containing `Fragment::Char` may overflow the given `area_width`.
#[inline]
pub fn by_letters<'a, I>(fragments: I, area_width: u16) -> SmallVec<[Fragment<'a>; 4]>
where
    I: IntoIterator<Item = Fragment<'a>>,
{
    let fragments = fragments.into_iter();

    let mut results = if let (_, Some(max)) = fragments.size_hint() {
        SmallVec::with_capacity(max)
    } else {
        SmallVec::new()
    };

    let mut line_length = 0;

    for fragment in fragments {
        match fragment {
            Fragment::Span(span, opts) => {
                let wrapped = wrap_span_letters(span, opts, area_width, &mut line_length);
                results.extend(wrapped.into_iter());
            }
            Fragment::Line => {
                line_length = 0;
                results.push(fragment);
            }
            fragment @ Fragment::Char(..) => {
                line_length += fragment.len();
                results.push(fragment);
            }
        }
    }

    results
}

fn wrap_span_letters<'a>(
    span: Span<'a>,
    opts: SpanOptions,
    area_width: u16,
    line_length: &mut u16,
) -> SmallVec<[Fragment<'a>; 4]> {
    let mut results = SmallVec::new();

    let mut segment_start = 0;
    let graphemes = UnicodeSegmentation::grapheme_indices(span.content.as_ref(), true);

    for (pos, ch) in graphemes {
        let ch_width = ch.width() as u16;
        *line_length += ch_width;

        if *line_length <= area_width {
            continue;
        }

        let segment_span = Span::styled(span.content[segment_start..pos].to_owned(), span.style);

        results.reserve(2);
        results.push(Fragment::Span(segment_span, opts));
        results.push(Fragment::Line);

        segment_start = pos;
        *line_length = ch_width;
    }

    // If our start position hasn't moved, then we never had to wrap anything
    let segment = if segment_start == 0 {
        span
    } else {
        let content = span.content[segment_start..].to_owned();

        // The grapheme loop already increases the line length, so to avoid counting it twice
        // we should only increase it here if a line break was inserted (aka our start position moved)
        *line_length += content.width() as u16;

        Span::styled(content, span.style)
    };

    results.push(Fragment::Span(segment, opts));
    results
}

/// Wrap the given `fragments` iterator by newline characters.
///
/// Returns a new `SmallVec` containing the given `fragments` with `Fragment::Line`'s inserted at appropriate places.
#[inline]
pub fn by_newlines<'a, I>(fragments: I) -> SmallVec<[Fragment<'a>; 4]>
where
    I: IntoIterator<Item = Fragment<'a>>,
{
    let fragments = fragments.into_iter();

    let mut results = if let (_, Some(max)) = fragments.size_hint() {
        SmallVec::with_capacity(max)
    } else {
        SmallVec::new()
    };

    for fragment in fragments {
        match fragment {
            Fragment::Span(span, opts) => {
                let wrapped = wrap_span_newlines(span, opts);
                results.extend(wrapped.into_iter());
            }
            Fragment::Char('\n', _) => results.push(Fragment::Line),
            fragment @ Fragment::Char(..) | fragment @ Fragment::Line => results.push(fragment),
        }
    }

    results
}

fn wrap_span_newlines(span: Span, opts: SpanOptions) -> SmallVec<[Fragment; 4]> {
    let mut results = SmallVec::with_capacity(1);
    let mut segment_start = 0;

    for (pos, ch) in span.content.as_bytes().iter().enumerate() {
        if *ch != b'\n' {
            continue;
        }

        let segment = &span.content[segment_start..pos];

        if !segment.is_empty() {
            let segment_span = Span::styled(segment.to_owned(), span.style);

            results.reserve(2);
            results.push(Fragment::Span(segment_span, opts));
        }

        results.push(Fragment::Line);

        // Our next position should skip the newline character
        segment_start = (pos + 1).min(span.content.len());
    }

    // If our start position never moved, then we never had to wrap anything -- which means we can return early!
    if segment_start == 0 {
        results.push(Fragment::Span(span, opts));
        return results;
    }

    let content = &span.content[segment_start..];

    if !content.is_empty() {
        let content_span = Span::styled(content.to_owned(), span.style);
        results.push(Fragment::Span(content_span, opts));
    }

    results
}

#[cfg(test)]
mod tests {
    use super::by_newlines;
    use crate::widgets::Fragment;

    #[test]
    fn newlines_empty() {
        let fragments = [];
        let result = by_newlines(fragments.iter().cloned());

        assert_eq!(result.as_slice(), []);
    }

    #[test]
    fn newlines_no_wrapping() {
        let fragments = [Fragment::span("this is a no wrapping test")];
        let result = by_newlines(fragments.iter().cloned());

        assert_eq!(result.as_slice(), fragments);
    }

    #[test]
    fn newlines_wrap_once() {
        let fragments = [Fragment::span("this is a simple\nwrapping test")];
        let result = by_newlines(fragments.iter().cloned());

        assert_eq!(
            result.as_slice(),
            [
                Fragment::span("this is a simple"),
                Fragment::Line,
                Fragment::span("wrapping test")
            ]
        );
    }

    #[test]
    fn newlines_wrap_three_times() {
        let fragments = [Fragment::span("this is\na simple\nwrapping\ntest")];
        let result = by_newlines(fragments.iter().cloned());

        assert_eq!(
            result.as_slice(),
            [
                Fragment::span("this is"),
                Fragment::Line,
                Fragment::span("a simple"),
                Fragment::Line,
                Fragment::span("wrapping"),
                Fragment::Line,
                Fragment::span("test")
            ]
        );
    }

    #[test]
    fn newlines_multiple_wraps_at_a_time() {
        let fragments = [Fragment::span("this is a\n\n\nwrapping test")];
        let result = by_newlines(fragments.iter().cloned());

        assert_eq!(
            result.as_slice(),
            [
                Fragment::span("this is a"),
                Fragment::Line,
                Fragment::Line,
                Fragment::Line,
                Fragment::span("wrapping test")
            ]
        );
    }

    #[test]
    fn newlines_only_wraps() {
        let fragments = [Fragment::span("\n\n\n")];
        let result = by_newlines(fragments.iter().cloned());

        assert_eq!(
            result.as_slice(),
            [Fragment::Line, Fragment::Line, Fragment::Line]
        );
    }

    #[test]
    fn newlines_only_wraps_once() {
        let fragments = [Fragment::span("\n")];
        let result = by_newlines(fragments.iter().cloned());

        assert_eq!(result.as_slice(), [Fragment::Line]);
    }

    #[test]
    fn newlines_starts_with_wrap_ends_with_wrap() {
        let fragments = [Fragment::span("\nThis is a test!\n")];
        let result = by_newlines(fragments.iter().cloned());

        assert_eq!(
            result.as_slice(),
            [
                Fragment::Line,
                Fragment::span("This is a test!"),
                Fragment::Line
            ]
        );
    }
}
