use std::{ffi::c_void, ptr};

use glib::{prelude::*, subclass::prelude::*, translate::*, Error};

use crate::{ArrowRecordBatch, ArrowRecordBatchStream, ArrowSchema};

pub trait ArrowRecordBatchStreamImpl: ObjectImpl + Send + Sync {
    fn next_async<P: FnOnce(Result<Option<ArrowRecordBatch>, Error>) + Send + 'static>(
        &self,
        cancellable: Option<&gio::Cancellable>,
        callback: P,
    );
    fn schema(&self) -> Option<ArrowSchema>;
}

unsafe impl<T: ArrowRecordBatchStreamImpl> IsImplementable<T> for ArrowRecordBatchStream {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.next_async = Some(arrow_record_batch_stream_next_async::<T>);
        iface.next_finish = Some(arrow_record_batch_stream_next_finish);
        iface.get_schema = Some(arrow_record_batch_stream_get_schema::<T>);
    }
}

unsafe extern "C" fn arrow_record_batch_stream_next_async<T: ArrowRecordBatchStreamImpl>(
    p_this: *mut ffi::OmniArrowRecordBatchStream,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let task: gio::Task<Option<ArrowRecordBatch>> = from_glib_full(gio::ffi::g_task_new(
        p_this.cast(),
        cancellable,
        callback,
        user_data,
    ));

    this_imp.next_async(Option::as_ref(&from_glib_none(cancellable)), |result| {
        task.return_result(result)
    });
}

unsafe extern "C" fn arrow_record_batch_stream_next_finish(
    p_this: *mut ffi::OmniArrowRecordBatchStream,
    result: *mut gio::ffi::GAsyncResult,
    err: *mut *mut glib::ffi::GError,
) -> *mut ffi::OmniArrowRecordBatch {
    assert_ne!(gio::ffi::g_task_is_valid(result, p_this.cast()), 0);
    let task: gio::Task<Option<ArrowRecordBatch>> =
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

unsafe extern "C" fn arrow_record_batch_stream_get_schema<T: ArrowRecordBatchStreamImpl>(
    p_this: *mut ffi::OmniArrowRecordBatchStream,
) -> *mut ffi::OmniArrowSchema {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    stash_ptr!(
        p_this,
        "omni-api-rs-subclass-arrow-record-batch-stream-get-schema",
        this_imp.schema().into_glib_ptr(),
        |ptr| if !ptr.is_null() {
            glib::gobject_ffi::g_object_unref(ptr.cast())
        },
    )
}
