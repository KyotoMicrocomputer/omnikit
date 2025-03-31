use glib::prelude::*;

use crate::{prelude::*, PortLinkSet};

pub trait PortLinkSetExtManual: IsA<PortLinkSet> + Sized {
    /// Get all link parameter sets as `Vec<Variant>`.
    fn param_sets(&self) -> Vec<glib::Variant> {
        (0..self.num_param_sets())
            .map(|i| {
                self.param_set(i)
                    .expect("`param_set` unexpectedly returned null")
            })
            .collect()
    }

    /// Get all link parameter sets as `Vec<VariantDict>`.
    ///
    /// `VariantDict` allows for faster lookups, but this comes at the upfront
    /// cost of converting `Variant` to `VariantDict`. The result is not cached,
    /// so you should avoid calling this method frequently.
    fn param_set_dicts(&self) -> Vec<glib::VariantDict> {
        (0..self.num_param_sets())
            .map(|i| {
                glib::VariantDict::new(Some(
                    &self
                        .param_set(i)
                        .expect("`param_set` unexpectedly returned null"),
                ))
            })
            .collect()
    }
}

impl<T: IsA<PortLinkSet>> PortLinkSetExtManual for T {}
