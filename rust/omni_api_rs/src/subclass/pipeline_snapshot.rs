use std::{ffi::c_void, ptr};

use glib::{prelude::*, subclass::prelude::*, translate::*, Error};

use crate::{
    ArrowRecordBatch, ArrowRecordBatchStream, PhysStreamId, PipelineLayout, PipelineQuery,
    PipelineSnapshot, StreamStateFlags, Time,
};

pub trait PipelineSnapshotImpl: ObjectImpl + Send + Sync {
    fn layout(&self) -> Option<PipelineLayout>;
    fn stream_state_flags(&self, stream_id: &PhysStreamId) -> StreamStateFlags;
    fn stream_start_time(&self, stream_id: &PhysStreamId) -> Time;
    fn stream_end_time(&self, stream_id: &PhysStreamId) -> Time;
    fn stream_next_time(&self, stream_id: &PhysStreamId, scan_start: Time) -> Time;
    fn stream_watermark_time(&self, stream_id: &PhysStreamId, scan_start: Time) -> Time;
    fn execute_query_async<P: FnOnce(Result<ArrowRecordBatch, Error>) + Send + 'static>(
        &self,
        query: &PipelineQuery,
        param_values: Option<glib::Variant>,
        cancellable: Option<&gio::Cancellable>,
        callback: P,
    );
    fn execute_query_stream_async<
        P: FnOnce(Result<ArrowRecordBatchStream, Error>) + Send + 'static,
    >(
        &self,
        query: &PipelineQuery,
        param_values: Option<glib::Variant>,
        cancellable: Option<&gio::Cancellable>,
        callback: P,
    );
}

unsafe impl<T: PipelineSnapshotImpl> IsImplementable<T> for PipelineSnapshot {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.layout = Some(pipeline_snapshot_layout::<T>);
        iface.get_stream_state_flags = Some(pipeline_snapshot_get_stream_state_flags::<T>);
        iface.get_stream_start_time = Some(pipeline_snapshot_get_stream_start_time::<T>);
        iface.get_stream_end_time = Some(pipeline_snapshot_get_stream_end_time::<T>);
        iface.get_stream_next_time = Some(pipeline_snapshot_get_stream_next_time::<T>);
        iface.get_stream_watermark_time = Some(pipeline_snapshot_get_stream_watermark_time::<T>);
        iface.execute_query_async = Some(pipeline_snapshot_execute_query_async::<T>);
        iface.execute_query_finish = Some(pipeline_snapshot_execute_query_finish);
        iface.execute_query_stream_async = Some(pipeline_snapshot_execute_query_stream_async::<T>);
        iface.execute_query_stream_finish = Some(pipeline_snapshot_execute_query_stream_finish);
    }
}

unsafe extern "C" fn pipeline_snapshot_layout<T: PipelineSnapshotImpl>(
    p_this: *mut ffi::OmniPipelineSnapshot,
) -> *mut ffi::OmniPipelineLayout {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.layout().into_glib_ptr()
}

unsafe extern "C" fn pipeline_snapshot_get_stream_state_flags<T: PipelineSnapshotImpl>(
    p_this: *mut ffi::OmniPipelineSnapshot,
    stream_id: *mut ffi::OmniPhysStreamId,
) -> ffi::OmniStreamStateFlags {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp
        .stream_state_flags(&from_glib_none(stream_id))
        .into_glib()
}

unsafe extern "C" fn pipeline_snapshot_get_stream_start_time<T: PipelineSnapshotImpl>(
    p_this: *mut ffi::OmniPipelineSnapshot,
    stream_id: *mut ffi::OmniPhysStreamId,
) -> ffi::OmniTime {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.stream_start_time(&from_glib_none(stream_id))
}

unsafe extern "C" fn pipeline_snapshot_get_stream_end_time<T: PipelineSnapshotImpl>(
    p_this: *mut ffi::OmniPipelineSnapshot,
    stream_id: *mut ffi::OmniPhysStreamId,
) -> ffi::OmniTime {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.stream_end_time(&from_glib_none(stream_id))
}

unsafe extern "C" fn pipeline_snapshot_get_stream_next_time<T: PipelineSnapshotImpl>(
    p_this: *mut ffi::OmniPipelineSnapshot,
    stream_id: *mut ffi::OmniPhysStreamId,
    scan_start: ffi::OmniTime,
) -> ffi::OmniTime {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.stream_next_time(&from_glib_none(stream_id), scan_start)
}

unsafe extern "C" fn pipeline_snapshot_get_stream_watermark_time<T: PipelineSnapshotImpl>(
    p_this: *mut ffi::OmniPipelineSnapshot,
    stream_id: *mut ffi::OmniPhysStreamId,
    scan_start: ffi::OmniTime,
) -> ffi::OmniTime {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.stream_watermark_time(&from_glib_none(stream_id), scan_start)
}

unsafe extern "C" fn pipeline_snapshot_execute_query_async<T: PipelineSnapshotImpl>(
    p_this: *mut ffi::OmniPipelineSnapshot,
    query: *mut ffi::OmniPipelineQuery,
    param_values: *mut glib::ffi::GVariant,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let task: gio::Task<ArrowRecordBatch> = from_glib_full(gio::ffi::g_task_new(
        p_this.cast(),
        cancellable,
        callback,
        user_data,
    ));

    this_imp.execute_query_async(
        &from_glib_none(query),
        from_glib_none(param_values),
        Option::as_ref(&from_glib_none(cancellable)),
        |result| task.return_result(result),
    );
}

unsafe extern "C" fn pipeline_snapshot_execute_query_finish(
    p_this: *mut ffi::OmniPipelineSnapshot,
    result: *mut gio::ffi::GAsyncResult,
    err: *mut *mut glib::ffi::GError,
) -> *mut ffi::OmniArrowRecordBatch {
    assert_ne!(gio::ffi::g_task_is_valid(result, p_this.cast()), 0);
    let task: gio::Task<ArrowRecordBatch> =
        gio::AsyncResult::from_glib_none(result).downcast().unwrap();
    match task.propagate() {
        Ok(x) => x.into_glib_ptr(),
        Err(e) => {
            if !err.is_null() {
                *err = e.into_glib_ptr();
            }
            ptr::null_mut()
        }
    }
}

unsafe extern "C" fn pipeline_snapshot_execute_query_stream_async<T: PipelineSnapshotImpl>(
    p_this: *mut ffi::OmniPipelineSnapshot,
    query: *mut ffi::OmniPipelineQuery,
    param_values: *mut glib::ffi::GVariant,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let task: gio::Task<ArrowRecordBatchStream> = from_glib_full(gio::ffi::g_task_new(
        p_this.cast(),
        cancellable,
        callback,
        user_data,
    ));

    this_imp.execute_query_stream_async(
        &from_glib_none(query),
        from_glib_none(param_values),
        Option::as_ref(&from_glib_none(cancellable)),
        |result| task.return_result(result),
    );
}

unsafe extern "C" fn pipeline_snapshot_execute_query_stream_finish(
    p_this: *mut ffi::OmniPipelineSnapshot,
    result: *mut gio::ffi::GAsyncResult,
    err: *mut *mut glib::ffi::GError,
) -> *mut ffi::OmniArrowRecordBatchStream {
    assert_ne!(gio::ffi::g_task_is_valid(result, p_this.cast()), 0);
    let task: gio::Task<ArrowRecordBatchStream> =
        gio::AsyncResult::from_glib_none(result).downcast().unwrap();
    match task.propagate() {
        Ok(x) => x.into_glib_ptr(),
        Err(e) => {
            if !err.is_null() {
                *err = e.into_glib_ptr();
            }
            ptr::null_mut()
        }
    }
}
