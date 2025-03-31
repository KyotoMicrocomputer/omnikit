use glib::subclass::prelude::*;

use crate::ValueLineLayerBuilder;

pub trait ValueLineLayerBuilderImpl: ObjectImpl {}

unsafe impl<T: ValueLineLayerBuilderImpl> IsImplementable<T> for ValueLineLayerBuilder {
    fn interface_init(_iface: &mut glib::Interface<Self>) {}
}
