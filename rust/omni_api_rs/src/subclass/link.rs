use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::Link;

pub trait LinkImpl: ObjectImpl {
    fn port_name(&self) -> glib::GString;
    fn param_set(&self) -> Option<glib::Variant>;
    fn set_param_set(&self, value: Option<glib::Variant>);
    fn output_set(&self) -> Option<glib::Variant>;
}

unsafe impl<T: LinkImpl> IsImplementable<T> for Link {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.get_port_name = Some(link_get_port_name::<T>);
        iface.get_param_set = Some(link_get_param_set::<T>);
        iface.set_param_set = Some(link_set_param_set::<T>);
        iface.get_output_set = Some(link_get_output_set::<T>);
    }
}

unsafe extern "C" fn link_get_port_name<T: LinkImpl>(p_this: *mut ffi::OmniLink) -> *const c_char {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    stash_ptr!(
        p_this,
        "omni-api-rs-subclass-link-get-port-name",
        this_imp.port_name().into_glib_ptr(),
        |ptr| glib::ffi::g_free(ptr.cast()),
    )
}

unsafe extern "C" fn link_get_param_set<T: LinkImpl>(
    p_this: *mut ffi::OmniLink,
) -> *mut glib::ffi::GVariant {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    stash_ptr!(
        p_this,
        "omni-api-rs-subclass-link-get-param-set",
        this_imp.param_set().into_glib_ptr(),
        |ptr| if !ptr.is_null() {
            glib::ffi::g_variant_unref(ptr)
        },
    )
}

unsafe extern "C" fn link_set_param_set<T: LinkImpl>(
    p_this: *mut ffi::OmniLink,
    value: *mut glib::ffi::GVariant,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.set_param_set(from_glib_none(value));
}

unsafe extern "C" fn link_get_output_set<T: LinkImpl>(
    p_this: *mut ffi::OmniLink,
) -> *mut glib::ffi::GVariant {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    stash_ptr!(
        p_this,
        "omni-api-rs-subclass-link-get-output-set",
        this_imp.output_set().into_glib_ptr(),
        |ptr| if !ptr.is_null() {
            glib::ffi::g_variant_unref(ptr)
        },
    )
}
