use glib::{subclass::prelude::*, translate::*};

use crate::{RecordClock, SourceActivateContext, StreamSink};

pub trait SourceActivateContextImpl: ObjectImpl + Send + Sync {
    fn stream_sink(&self, stream_handle: i32) -> Option<StreamSink>;
    fn record_clock(&self) -> Option<RecordClock>;
    fn started(&self);
    fn finish(&self, error: Option<glib::Error>);
}

unsafe impl<T: SourceActivateContextImpl> IsImplementable<T> for SourceActivateContext {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.dup_stream_sink = Some(source_activate_context_dup_stream_sink::<T>);
        iface.get_record_clock = Some(source_activate_context_get_record_clock::<T>);
        iface.started = Some(source_activate_context_started::<T>);
        iface.finish = Some(source_activate_context_finish::<T>);
    }
}

unsafe extern "C" fn source_activate_context_dup_stream_sink<T: SourceActivateContextImpl>(
    p_this: *mut ffi::OmniSourceActivateContext,
    stream_handle: i32,
) -> *mut ffi::OmniStreamSink {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.stream_sink(stream_handle).into_glib_ptr()
}

unsafe extern "C" fn source_activate_context_get_record_clock<T: SourceActivateContextImpl>(
    p_this: *mut ffi::OmniSourceActivateContext,
) -> *mut ffi::OmniRecordClock {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    stash_ptr!(
        p_this,
        "omni-api-rs-subclass-source-activate-context-get-record-clock",
        this_imp.record_clock().into_glib_ptr(),
        |ptr| if !ptr.is_null() {
            glib::gobject_ffi::g_object_unref(ptr.cast())
        },
    )
}

unsafe extern "C" fn source_activate_context_started<T: SourceActivateContextImpl>(
    p_this: *mut ffi::OmniSourceActivateContext,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.started();
}

unsafe extern "C" fn source_activate_context_finish<T: SourceActivateContextImpl>(
    p_this: *mut ffi::OmniSourceActivateContext,
    p_error: *mut glib::ffi::GError,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.finish(from_glib_full(p_error));
}
