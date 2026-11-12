use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::StreamDebugTrackBuilder;

pub trait StreamDebugTrackBuilderImpl: ObjectImpl {
    fn set_id(&self, value: Option<&str>);
    fn set_title(&self, value: Option<&str>);
}

unsafe impl<T: StreamDebugTrackBuilderImpl> IsImplementable<T> for StreamDebugTrackBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.set_id = Some(track_builder_set_id::<T>);
        iface.set_title = Some(track_builder_set_title::<T>);
    }
}

unsafe extern "C" fn track_builder_set_id<T: StreamDebugTrackBuilderImpl>(
    p_this: *mut ffi::OmniStreamDebugTrackBuilder,
    value: *const c_char,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let value_borrowed;
    this_imp.set_id(if value.is_null() {
        None
    } else {
        value_borrowed = glib::GString::from_glib_borrow(value);
        Some(&value_borrowed)
    });
}

unsafe extern "C" fn track_builder_set_title<T: StreamDebugTrackBuilderImpl>(
    p_this: *mut ffi::OmniStreamDebugTrackBuilder,
    value: *const c_char,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let value_borrowed;
    this_imp.set_title(if value.is_null() {
        None
    } else {
        value_borrowed = glib::GString::from_glib_borrow(value);
        Some(&value_borrowed)
    });
}
