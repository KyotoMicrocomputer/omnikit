use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::TransformOpBuilder;

pub trait TransformOpBuilderImpl: ObjectImpl {
    fn append_input_stream(&self, stream_handle: i32);
    fn append_output_stream(&self, stream_handle: i32);
    fn name(&self) -> Option<glib::GString>;
    fn set_name(&self, value: Option<&str>);
}

unsafe impl<T: TransformOpBuilderImpl> IsImplementable<T> for TransformOpBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.append_input_stream = Some(transform_op_builder_append_input_stream::<T>);
        iface.append_output_stream = Some(transform_op_builder_append_output_stream::<T>);
        iface.get_name = Some(transform_op_builder_get_name::<T>);
        iface.set_name = Some(transform_op_builder_set_name::<T>);
    }
}

unsafe extern "C" fn transform_op_builder_append_input_stream<T: TransformOpBuilderImpl>(
    p_this: *mut ffi::OmniTransformOpBuilder,
    stream_handle: i32,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.append_input_stream(stream_handle);
}

unsafe extern "C" fn transform_op_builder_append_output_stream<T: TransformOpBuilderImpl>(
    p_this: *mut ffi::OmniTransformOpBuilder,
    stream_handle: i32,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.append_output_stream(stream_handle);
}

unsafe extern "C" fn transform_op_builder_get_name<T: TransformOpBuilderImpl>(
    p_this: *mut ffi::OmniTransformOpBuilder,
) -> *const c_char {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    stash_ptr!(
        p_this,
        "omni-api-rs-subclass-transform-op-builder-get-name",
        this_imp.name().into_glib_ptr(),
        |ptr| glib::ffi::g_free(ptr.cast()),
    )
}

unsafe extern "C" fn transform_op_builder_set_name<T: TransformOpBuilderImpl>(
    p_this: *mut ffi::OmniTransformOpBuilder,
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
