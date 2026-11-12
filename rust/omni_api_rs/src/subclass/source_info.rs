use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::SourceInfo;

pub trait SourceInfoImpl: ObjectImpl {
    fn set_name(&self, value: Option<&str>);
    fn set_description(&self, value: Option<&str>);
    fn set_visible(&self, value: bool);
}

unsafe impl<T: SourceInfoImpl> IsImplementable<T> for SourceInfo {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.set_name = Some(source_info_set_name::<T>);
        iface.set_description = Some(source_info_set_description::<T>);
        iface.set_visible = Some(source_info_set_visible::<T>);
    }
}

unsafe extern "C" fn source_info_set_name<T: SourceInfoImpl>(
    p_this: *mut ffi::OmniSourceInfo,
    value: *const c_char,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let value_borrowed;
    this_imp.set_name(if value.is_null() {
        None
    } else {
        value_borrowed = glib::GString::from_glib_borrow(value);
        Some(&value_borrowed)
    });
}

unsafe extern "C" fn source_info_set_description<T: SourceInfoImpl>(
    p_this: *mut ffi::OmniSourceInfo,
    value: *const c_char,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let value_borrowed;
    this_imp.set_description(if value.is_null() {
        None
    } else {
        value_borrowed = glib::GString::from_glib_borrow(value);
        Some(&value_borrowed)
    });
}

unsafe extern "C" fn source_info_set_visible<T: SourceInfoImpl>(
    p_this: *mut ffi::OmniSourceInfo,
    value: glib::ffi::gboolean,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.set_visible(from_glib(value));
}
