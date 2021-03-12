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
            fragment => {
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
