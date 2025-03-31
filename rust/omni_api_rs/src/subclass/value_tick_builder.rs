use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::ValueTickBuilder;

pub trait ValueTickBuilderImpl: ObjectImpl {
    fn set_label(&self, value: Option<&str>);
}

unsafe impl<T: ValueTickBuilderImpl> IsImplementable<T> for ValueTickBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.set_label = Some(value_tick_builder_set_label::<T>);
    }
}

unsafe extern "C" fn value_tick_builder_set_label<T: ValueTickBuilderImpl>(
    p_this: *mut ffi::OmniValueTickBuilder,
    value: *const c_char,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let value_borrowed;
    this_imp.set_label(if value.is_null() {
        None
    } else {
        value_borrowed = glib::GString::from_glib_borrow(value);
        Some(&value_borrowed)
    });
}
