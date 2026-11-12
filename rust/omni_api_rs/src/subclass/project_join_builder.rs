use glib::subclass::prelude::*;

use crate::ProjectJoinBuilder;

pub trait ProjectJoinBuilderImpl: ObjectImpl {}

unsafe impl<T: ProjectJoinBuilderImpl> IsImplementable<T> for ProjectJoinBuilder {
    fn interface_init(_iface: &mut glib::Interface<Self>) {}
}
