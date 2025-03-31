use std::ffi::{c_char, c_double, c_int};

use glib::{subclass::prelude::*, translate::*};

use crate::{ColumnBuilder, ColumnFlags, SortOrder};

pub trait ColumnBuilderImpl: ObjectImpl {
    fn set_title(&self, value: Option<&str>);
    fn set_width(&self, value: f64);
    fn set_flags(&self, value: ColumnFlags);
    fn set_sort_order(&self, value: SortOrder);
    fn set_sort_precedence(&self, value: c_int);
    fn set_visible(&self, value: bool);
}

unsafe impl<T: ColumnBuilderImpl> IsImplementable<T> for ColumnBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.set_title = Some(column_builder_set_title::<T>);
        iface.set_width = Some(column_builder_set_width::<T>);
        iface.set_flags = Some(column_builder_set_flags::<T>);
        iface.set_sort_order = Some(column_builder_set_sort_order::<T>);
        iface.set_sort_precedence = Some(column_builder_set_sort_precedence::<T>);
        iface.set_visible = Some(column_builder_set_visible::<T>);
    }
}

unsafe extern "C" fn column_builder_set_title<T: ColumnBuilderImpl>(
    p_this: *mut ffi::OmniColumnBuilder,
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

unsafe extern "C" fn column_builder_set_width<T: ColumnBuilderImpl>(
    p_this: *mut ffi::OmniColumnBuilder,
    value: c_double,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.set_width(value);
}

unsafe extern "C" fn column_builder_set_flags<T: ColumnBuilderImpl>(
    p_this: *mut ffi::OmniColumnBuilder,
    value: ffi::OmniColumnFlags,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.set_flags(from_glib(value));
}

unsafe extern "C" fn column_builder_set_sort_order<T: ColumnBuilderImpl>(
    p_this: *mut ffi::OmniColumnBuilder,
    value: ffi::OmniSortOrder,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.set_sort_order(from_glib(value));
}

unsafe extern "C" fn column_builder_set_sort_precedence<T: ColumnBuilderImpl>(
    p_this: *mut ffi::OmniColumnBuilder,
    value: c_int,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.set_sort_precedence(value);
}

unsafe extern "C" fn column_builder_set_visible<T: ColumnBuilderImpl>(
    p_this: *mut ffi::OmniColumnBuilder,
    value: glib::ffi::gboolean,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.set_visible(from_glib(value));
}
