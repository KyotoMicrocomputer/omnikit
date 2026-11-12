use glib::prelude::*;

use crate::{prelude::LinkArrayExt, Link, LinkArray};

pub trait LinkArrayExtManual: IsA<LinkArray> + Sized {
    #[inline]
    fn iter(&self) -> impl Iterator<Item = Link> {
        (0..self.num_links()).map(move |i| self.link(i).expect("null"))
    }
}

impl<T: IsA<LinkArray>> LinkArrayExtManual for T {}
