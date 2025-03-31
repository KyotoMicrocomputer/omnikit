use std::ffi::c_uint;

use glib::{subclass::prelude::*, translate::*};

use crate::PortLinkSet;

pub trait PortLinkSetImpl: ObjectImpl {
    fn port_index(&self) -> c_uint;
    fn num_param_sets(&self) -> c_uint;
    fn param_set(&self, i: c_uint) -> Option<glib::Variant>;
    fn set_output_set(&self, i: c_uint, output_set: glib::Variant);
    fn set_output_set_all(&self, output_set: glib::Variant);
}

unsafe impl<T: PortLinkSetImpl> IsImplementable<T> for PortLinkSet {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.get_port_index = Some(port_link_set_get_port_index::<T>);
        iface.get_num_param_sets = Some(port_link_set_get_num_param_sets::<T>);
        iface.dup_param_set = Some(port_link_set_dup_param_set::<T>);
        iface.set_output_set = Some(port_link_set_set_output_set::<T>);
        iface.set_output_set_all = Some(port_link_set_set_output_set_all::<T>);
    }
}

unsafe extern "C" fn port_link_set_get_port_index<T: PortLinkSetImpl>(
    p_this: *mut ffi::OmniPortLinkSet,
) -> c_uint {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.port_index()
}

unsafe extern "C" fn port_link_set_get_num_param_sets<T: PortLinkSetImpl>(
    p_this: *mut ffi::OmniPortLinkSet,
) -> c_uint {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.num_param_sets()
}

unsafe extern "C" fn port_link_set_dup_param_set<T: PortLinkSetImpl>(
    p_this: *mut ffi::OmniPortLinkSet,
    i: c_uint,
) -> *mut glib::ffi::GVariant {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.param_set(i).into_glib_ptr()
}

unsafe extern "C" fn port_link_set_set_output_set<T: PortLinkSetImpl>(
    p_this: *mut ffi::OmniPortLinkSet,
    i: c_uint,
    output_set: *mut glib::ffi::GVariant,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.set_output_set(i, from_glib_none(output_set));
}

unsafe extern "C" fn port_link_set_set_output_set_all<T: PortLinkSetImpl>(
    p_this: *mut ffi::OmniPortLinkSet,
    output_set: *mut glib::ffi::GVariant,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.set_output_set_all(from_glib_none(output_set));
}
