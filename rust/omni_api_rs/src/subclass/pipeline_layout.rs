use std::{
    ffi::{c_char, c_void},
    ptr,
};

use glib::{prelude::*, subclass::prelude::*, translate::*, Error};

use crate::{PhysStreamId, PipelineLayout, PipelineQuery, StreamRepr};

pub trait PipelineLayoutImpl: ObjectImpl + Send + Sync {
    fn stream_repr(&self, stream_id: &PhysStreamId) -> Result<StreamRepr, Error>;
    fn prepare_query_async<P: FnOnce(Result<PipelineQuery, Error>) + Send + 'static>(
        &self,
        sql: &str,
        cancellable: Option<&gio::Cancellable>,
        callback: P,
    );
}

unsafe impl<T: PipelineLayoutImpl> IsImplementable<T> for PipelineLayout {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.stream_repr = Some(pipeline_layout_stream_repr::<T>);
        iface.prepare_query_async = Some(pipeline_layout_prepare_query_async::<T>);
        iface.prepare_query_finish = Some(pipeline_layout_prepare_query_finish);
    }
}

unsafe extern "C" fn pipeline_layout_stream_repr<T: PipelineLayoutImpl>(
    p_this: *mut ffi::OmniPipelineLayout,
    stream_id: *mut ffi::OmniPhysStreamId,
    err: *mut *mut glib::ffi::GError,
) -> *mut ffi::OmniStreamRepr {
    let this = &*(p_this as *mut T::Instance);
    let this_imp = this.imp();

    match this_imp.stream_repr(&from_glib_none(stream_id)) {
        Ok(x) => x.into_glib_ptr(),
        Err(e) => {
            if !err.is_null() {
                *err = e.into_glib_ptr();
            }
            ptr::null_mut()
        }
    }
}

unsafe extern "C" fn pipeline_layout_prepare_query_async<T: PipelineLayoutImpl>(
    p_this: *mut ffi::OmniPipelineLayout,
    sql: *const c_char,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let task: gio::Task<PipelineQuery> = from_glib_full(gio::ffi::g_task_new(
        p_this.cast(),
        cancellable,
        callback,
        user_data,
    ));

    this_imp.prepare_query_async(
        &glib::GString::from_glib_borrow(sql),
        Option::as_ref(&from_glib_none(cancellable)),
        |result| task.return_result(result),
    );
}

unsafe extern "C" fn pipeline_layout_prepare_query_finish(
    this: *mut ffi::OmniPipelineLayout,
    result: *mut gio::ffi::GAsyncResult,
    err: *mut *mut glib::ffi::GError,
) -> *mut ffi::OmniPipelineQuery {
    assert_ne!(gio::ffi::g_task_is_valid(result, this.cast()), 0);
    let task: gio::Task<PipelineQuery> =
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
