use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::SummaryBuilder;

pub trait SummaryBuilderImpl: ObjectImpl {
    fn set_id(&self, value: Option<&str>);
}

unsafe impl<T: SummaryBuilderImpl> IsImplementable<T> for SummaryBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.set_id = Some(summary_builder_set_id::<T>);
    }
}

unsafe extern "C" fn summary_builder_set_id<T: SummaryBuilderImpl>(
    p_this: *mut ffi::OmniSummaryBuilder,
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
