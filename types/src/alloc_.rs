//! Code which depends on the `alloc` feature.

use crate::{Mic, mic};
use alloc::borrow::ToOwned;

impl ToOwned for mic {
    type Owned = Mic;

    #[inline]
    fn to_owned(&self) -> Self::Owned {
        self.to_mic()
    }
}
