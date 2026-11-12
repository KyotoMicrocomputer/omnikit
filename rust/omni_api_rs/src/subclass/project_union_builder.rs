use glib::subclass::prelude::*;

use crate::ProjectUnionBuilder;

pub trait ProjectUnionBuilderImpl: ObjectImpl {}

unsafe impl<T: ProjectUnionBuilderImpl> IsImplementable<T> for ProjectUnionBuilder {
    fn interface_init(_iface: &mut glib::Interface<Self>) {}
}
