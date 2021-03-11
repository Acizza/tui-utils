use tui::style::Color;

#[inline]
#[must_use]
pub fn either(value: bool, true_color: Color, false_color: Color) -> Color {
    if value {
        true_color
    } else {
        false_color
    }
}
