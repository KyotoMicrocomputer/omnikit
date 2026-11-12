use std::{
    ffi::{c_int, c_void},
    ptr,
};

use glib::{prelude::*, subclass::prelude::*, translate::*};

use crate::{ArrowRecordBatch, StreamSink, Time};

pub trait StreamSinkImpl: ObjectImpl {
    fn feed_table_batch(&self, batch: Option<ArrowRecordBatch>, time_start: Time, time_end: Time);

    fn feed_table_batch_async<P: FnOnce() + Send + 'static>(
        &self,
        batch: Option<ArrowRecordBatch>,
        time_start: Time,
        time_end: Time,
        callback: P,
    );
}

unsafe impl<T: StreamSinkImpl> IsImplementable<T> for StreamSink {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.feed_table_batch = Some(stream_sink_feed_table_batch::<T>);
        iface.feed_table_batch_async = Some(stream_sink_feed_table_batch_async::<T>);
        iface.feed_table_batch_finish = Some(stream_sink_feed_table_batch_finish);
    }
}

unsafe extern "C" fn stream_sink_feed_table_batch<T: StreamSinkImpl>(
    p_this: *mut ffi::OmniStreamSink,
    batch: *mut ffi::OmniArrowRecordBatch,
    time_start: ffi::OmniTime,
    time_end: ffi::OmniTime,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.feed_table_batch(from_glib_none(batch), time_start, time_end);
}

unsafe extern "C" fn stream_sink_feed_table_batch_async<T: StreamSinkImpl>(
    p_this: *mut ffi::OmniStreamSink,
    batch: *mut ffi::OmniArrowRecordBatch,
    time_start: ffi::OmniTime,
    time_end: ffi::OmniTime,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let task: gio::Task<u8> = from_glib_full(gio::ffi::g_task_new(
        p_this.cast(),
        ptr::null_mut(),
        callback,
        user_data,
    ));

    this_imp.feed_table_batch_async(from_glib_none(batch), time_start, time_end, || {
        task.return_result(Ok(0))
    });
}

unsafe extern "C" fn stream_sink_feed_table_batch_finish(
    p_this: *mut ffi::OmniStreamSink,
    result: *mut gio::ffi::GAsyncResult,
) -> c_int {
    // TODO: De-duplicate all `_finish` impls because they are mostly identical
    assert_ne!(gio::ffi::g_task_is_valid(result, p_this.cast()), 0);
    let task: gio::Task<u8> = gio::AsyncResult::from_glib_none(result).downcast().unwrap();
    task.propagate().unwrap();
    0
}
