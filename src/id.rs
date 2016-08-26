// Copyright 2016 immi Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// Represents an identifier of a widget. See the documentation of `DrawContext`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WidgetId(usize);

impl From<usize> for WidgetId {
    #[inline]
    fn from(id: usize) -> WidgetId {
        WidgetId(id)
    }
}
