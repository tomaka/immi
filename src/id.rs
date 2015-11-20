
/// Represents an identifier of a widget. See the documentation of `DrawContext`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WidgetId(usize);

impl From<usize> for WidgetId {
    #[inline]
    fn from(id: usize) -> WidgetId {
        WidgetId(id)
    }
}
