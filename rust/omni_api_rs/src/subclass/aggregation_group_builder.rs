use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::AggregationGroupBuilder;

pub trait AggregationGroupBuilderImpl: ObjectImpl {
    fn set_title(&self, value: Option<&str>);
    fn set_active(&self, value: bool);
}

unsafe impl<T: AggregationGroupBuilderImpl> IsImplementable<T> for AggregationGroupBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.set_title = Some(aggregation_group_builder_set_title::<T>);
        iface.set_active = Some(aggregation_group_builder_set_active::<T>);
    }
}

unsafe extern "C" fn aggregation_group_builder_set_title<T: AggregationGroupBuilderImpl>(
    p_this: *mut ffi::OmniAggregationGroupBuilder,
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

unsafe extern "C" fn aggregation_group_builder_set_active<T: AggregationGroupBuilderImpl>(
    p_this: *mut ffi::OmniAggregationGroupBuilder,
    value: glib::ffi::gboolean,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.set_active(from_glib(value));
}
