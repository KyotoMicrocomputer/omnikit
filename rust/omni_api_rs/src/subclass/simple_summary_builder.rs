use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::SimpleSummaryBuilder;

pub trait SimpleSummaryBuilderImpl: ObjectImpl {
    fn set_detail_field_name(&self, value: Option<&str>);
    fn set_count_field_name(&self, value: Option<&str>);
}

unsafe impl<T: SimpleSummaryBuilderImpl> IsImplementable<T> for SimpleSummaryBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.set_detail_field_name = Some(simple_summary_builder_set_detail_field_name::<T>);
        iface.set_count_field_name = Some(simple_summary_builder_set_count_field_name::<T>);
    }
}

unsafe extern "C" fn simple_summary_builder_set_detail_field_name<T: SimpleSummaryBuilderImpl>(
    p_this: *mut ffi::OmniSimpleSummaryBuilder,
    value: *const c_char,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let value_borrowed;
    this_imp.set_detail_field_name(if value.is_null() {
        None
    } else {
        value_borrowed = glib::GString::from_glib_borrow(value);
        Some(&value_borrowed)
    });
}

unsafe extern "C" fn simple_summary_builder_set_count_field_name<T: SimpleSummaryBuilderImpl>(
    p_this: *mut ffi::OmniSimpleSummaryBuilder,
    value: *const c_char,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let value_borrowed;
    this_imp.set_count_field_name(if value.is_null() {
        None
    } else {
        value_borrowed = glib::GString::from_glib_borrow(value);
        Some(&value_borrowed)
    });
}
