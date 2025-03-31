use std::{ffi::c_void, future::Future, ptr};

use glib::{prelude::*, subclass::prelude::*, translate::*, Error};

use crate::{Source, SourceSnapshot};

/// The subclassing trait for [`Source`].
pub trait SourceImpl: ObjectImpl + Send + Sync {
    fn take_snapshot_async<P: FnOnce(Result<SourceSnapshot, Error>) + Send + 'static>(
        &self,
        cancellable: Option<&gio::Cancellable>,
        callback: P,
    ) {
        let cancellable = cancellable.cloned();
        default_async!(callback, move || self.take_snapshot_future(cancellable));
    }
    fn take_snapshot_future(
        &self,
        cancellable: Option<gio::Cancellable>,
    ) -> impl Future<Output = Result<SourceSnapshot, glib::Error>>;
}

unsafe impl<T: SourceImpl> IsImplementable<T> for Source {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.take_snapshot_async = Some(source_take_snapshot_async::<T>);
        iface.take_snapshot_finish = Some(source_take_snapshot_finish);
    }
}

unsafe extern "C" fn source_take_snapshot_async<T: SourceImpl>(
    p_this: *mut ffi::OmniSource,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let task: gio::Task<SourceSnapshot> = from_glib_full(gio::ffi::g_task_new(
        p_this.cast(),
        cancellable,
        callback,
        user_data,
    ));

    this_imp.take_snapshot_async(Option::as_ref(&from_glib_none(cancellable)), |result| {
        task.return_result(result)
    });
}

unsafe extern "C" fn source_take_snapshot_finish(
    p_this: *mut ffi::OmniSource,
    result: *mut gio::ffi::GAsyncResult,
    err: *mut *mut glib::ffi::GError,
) -> *mut ffi::OmniSourceSnapshot {
    // TODO: De-duplicate all `_finish` impls because they are mostly identical
    assert_ne!(gio::ffi::g_task_is_valid(result, p_this.cast()), 0);
    let task: gio::Task<SourceSnapshot> =
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
