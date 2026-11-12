use glib::subclass::prelude::*;

use crate::ProjectInputStreamBuilder;

pub trait ProjectInputStreamBuilderImpl: ObjectImpl {}

unsafe impl<T: ProjectInputStreamBuilderImpl> IsImplementable<T> for ProjectInputStreamBuilder {
    fn interface_init(_iface: &mut glib::Interface<Self>) {}
}
