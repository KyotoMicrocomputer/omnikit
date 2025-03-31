//! Traits and essential types intended for blanket imports.

#[doc(hidden)]
pub use glib::prelude::*;

pub use crate::{
    auto::traits::*,
    link_array::LinkArrayExtManual,
    link_policy::{LinkPolicyExtManual, ParamSpecBuilderLinkPolicyExt},
    plugin_registry::PluginRegistryExtManual,
    port_link_set::PortLinkSetExtManual,
};
