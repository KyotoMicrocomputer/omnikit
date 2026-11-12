use std::ffi::{c_char, c_uint};

use glib::{subclass::prelude::*, translate::*};

use crate::{ArrowSchema, StreamBuilder, TableStreamKeyFlags};

pub trait StreamBuilderImpl: ObjectImpl {
    fn handle(&self) -> i32;
    fn name(&self) -> Option<glib::GString>;
    fn set_name(&self, value: Option<&str>);
    fn table_schema(&self) -> Option<ArrowSchema>;
    fn table_primary_key(&self) -> Option<glib::GString>;
    fn set_table_primary_key(&self, value: Option<&str>);
    fn append_table_secondary_key(&self, name: &str, flags: TableStreamKeyFlags) -> u32;
    fn num_table_secondary_keys(&self) -> u32;
    fn table_secondary_key(&self, index: u32) -> Option<(glib::GString, TableStreamKeyFlags)>;
}

unsafe impl<T: StreamBuilderImpl> IsImplementable<T> for StreamBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.get_handle = Some(stream_builder_get_handle::<T>);
        iface.get_name = Some(stream_builder_get_name::<T>);
        iface.set_name = Some(stream_builder_set_name::<T>);
        iface.dup_table_schema = Some(stream_builder_dup_table_schema::<T>);
        iface.get_table_primary_key = Some(stream_builder_get_table_primary_key::<T>);
        iface.set_table_primary_key = Some(stream_builder_set_table_primary_key::<T>);
        iface.append_table_secondary_key = Some(stream_builder_append_table_secondary_key::<T>);
        iface.get_num_table_secondary_keys = Some(stream_builder_get_num_table_secondary_keys::<T>);
        iface.get_table_secondary_key = Some(stream_builder_get_table_secondary_key::<T>);
    }
}

unsafe extern "C" fn stream_builder_get_handle<T: StreamBuilderImpl>(
    p_this: *mut ffi::OmniStreamBuilder,
) -> i32 {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.handle()
}

unsafe extern "C" fn stream_builder_get_name<T: StreamBuilderImpl>(
    p_this: *mut ffi::OmniStreamBuilder,
) -> *const c_char {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    stash_ptr!(
        p_this,
        "omni-api-rs-subclass-stream-builder-get-name",
        this_imp.name().into_glib_ptr(),
        |ptr| glib::ffi::g_free(ptr.cast()),
    )
}

unsafe extern "C" fn stream_builder_set_name<T: StreamBuilderImpl>(
    p_this: *mut ffi::OmniStreamBuilder,
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

unsafe extern "C" fn stream_builder_dup_table_schema<T: StreamBuilderImpl>(
    p_this: *mut ffi::OmniStreamBuilder,
) -> *mut ffi::OmniArrowSchema {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.table_schema().into_glib_ptr()
}

unsafe extern "C" fn stream_builder_get_table_primary_key<T: StreamBuilderImpl>(
    p_this: *mut ffi::OmniStreamBuilder,
) -> *const c_char {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    stash_ptr!(
        p_this,
        "omni-api-rs-subclass-stream-builder-get-table-primary-key",
        this_imp.table_primary_key().into_glib_ptr(),
        |ptr| glib::ffi::g_free(ptr.cast()),
    )
}

unsafe extern "C" fn stream_builder_set_table_primary_key<T: StreamBuilderImpl>(
    p_this: *mut ffi::OmniStreamBuilder,
    value: *const c_char,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let value_borrowed;

    this_imp.set_table_primary_key(if value.is_null() {
        None
    } else {
        value_borrowed = glib::GString::from_glib_borrow(value);
        Some(&value_borrowed)
    });
}

unsafe extern "C" fn stream_builder_append_table_secondary_key<T: StreamBuilderImpl>(
    p_this: *mut ffi::OmniStreamBuilder,
    name: *const c_char,
    flags: ffi::OmniTableStreamKeyFlags,
) -> c_uint {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.append_table_secondary_key(&glib::GString::from_glib_borrow(name), from_glib(flags))
}

unsafe extern "C" fn stream_builder_get_num_table_secondary_keys<T: StreamBuilderImpl>(
    p_this: *mut ffi::OmniStreamBuilder,
) -> c_uint {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.num_table_secondary_keys()
}

unsafe extern "C" fn stream_builder_get_table_secondary_key<T: StreamBuilderImpl>(
    p_this: *mut ffi::OmniStreamBuilder,
    index: c_uint,
    p_out_name: *mut *mut c_char,
    p_out_flags: *mut ffi::OmniTableStreamKeyFlags,
) -> glib::ffi::gboolean {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let Some((name, flags)) = this_imp.table_secondary_key(index) else {
        return glib::ffi::GFALSE;
    };

    if !p_out_name.is_null() {
        *p_out_name = name.into_glib_ptr();
    }

    if !p_out_flags.is_null() {
        *p_out_flags = flags.into_glib();
    }

    glib::ffi::GTRUE
}
