use std::ffi::c_uint;

use glib::{subclass::prelude::*, translate::*};

use crate::{ArrowRecordBatch, Time, TransformUniformContext};

pub trait TransformUniformContextImpl: ObjectImpl + Send + Sync {
    fn start(&self) -> Time;
    fn end(&self) -> Time;
    fn len_hint(&self) -> Time;
    fn is_continuation(&self) -> bool;
    fn blocked_by(&self, input_stream_i: c_uint, scan_start: Time);
    fn advance(&self, time_end: Time);
    fn depend_on(&self, input_stream_i: c_uint, start: Time, end: Time);
    fn append_batch(&self, batch: Option<ArrowRecordBatch>);
}

unsafe impl<T: TransformUniformContextImpl> IsImplementable<T> for TransformUniformContext {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.get_start = Some(transform_uniform_context_get_start::<T>);
        iface.get_end = Some(transform_uniform_context_get_end::<T>);
        iface.get_len_hint = Some(transform_uniform_context_get_len_hint::<T>);
        iface.get_continuation = Some(transform_uniform_context_get_continuation::<T>);
        iface.blocked_by = Some(transform_uniform_context_blocked_by::<T>);
        iface.advance = Some(transform_uniform_context_advance::<T>);
        iface.depend_on = Some(transform_uniform_context_depend_on::<T>);
        iface.append_batch = Some(transform_uniform_context_append_batch::<T>);
    }
}

unsafe extern "C" fn transform_uniform_context_get_start<T: TransformUniformContextImpl>(
    p_this: *mut ffi::OmniTransformUniformContext,
) -> ffi::OmniTime {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.start()
}

unsafe extern "C" fn transform_uniform_context_get_end<T: TransformUniformContextImpl>(
    p_this: *mut ffi::OmniTransformUniformContext,
) -> ffi::OmniTime {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.end()
}

unsafe extern "C" fn transform_uniform_context_get_len_hint<T: TransformUniformContextImpl>(
    p_this: *mut ffi::OmniTransformUniformContext,
) -> ffi::OmniTime {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.len_hint()
}

unsafe extern "C" fn transform_uniform_context_get_continuation<T: TransformUniformContextImpl>(
    p_this: *mut ffi::OmniTransformUniformContext,
) -> glib::ffi::gboolean {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.is_continuation().into_glib()
}

unsafe extern "C" fn transform_uniform_context_blocked_by<T: TransformUniformContextImpl>(
    p_this: *mut ffi::OmniTransformUniformContext,
    input_stream_i: c_uint,
    scan_start: ffi::OmniTime,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.blocked_by(input_stream_i, scan_start);
}

unsafe extern "C" fn transform_uniform_context_advance<T: TransformUniformContextImpl>(
    p_this: *mut ffi::OmniTransformUniformContext,
    time_end: ffi::OmniTime,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.advance(time_end);
}

unsafe extern "C" fn transform_uniform_context_depend_on<T: TransformUniformContextImpl>(
    p_this: *mut ffi::OmniTransformUniformContext,
    input_stream_i: c_uint,
    start: ffi::OmniTime,
    end: ffi::OmniTime,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.depend_on(input_stream_i, start, end);
}

unsafe extern "C" fn transform_uniform_context_append_batch<T: TransformUniformContextImpl>(
    p_this: *mut ffi::OmniTransformUniformContext,
    batch: *mut ffi::OmniArrowRecordBatch,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.append_batch(from_glib_none(batch));
}
