//! Code which depends on the `alloc` feature

use crate::{mic, Mic};
use alloc::borrow::ToOwned;

impl ToOwned for mic {
    type Owned = Mic;

    fn to_owned(&self) -> Self::Owned {
        self.to_mic()
    }
}
