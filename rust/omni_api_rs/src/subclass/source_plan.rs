use std::{ffi::c_void, future::Future, ptr};

use glib::{prelude::*, subclass::prelude::*, translate::*, Error};

use crate::{ActiveSource, SourceActivateContext, SourcePlan};

pub trait SourcePlanImpl: ObjectImpl {
    fn activate_async<P: FnOnce(Result<ActiveSource, Error>) + Send + 'static>(
        &self,
        context: SourceActivateContext,
        cancellable: Option<&gio::Cancellable>,
        callback: P,
    ) {
        let cancellable = cancellable.cloned();
        default_async!(callback, move || self
            .activate_future(context, cancellable,));
    }
    fn activate_future(
        &self,
        context: SourceActivateContext,
        cancellable: Option<gio::Cancellable>,
    ) -> impl Future<Output = Result<ActiveSource, glib::Error>>;
}

unsafe impl<T: SourcePlanImpl> IsImplementable<T> for SourcePlan {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.activate_async = Some(source_plan_activate_async::<T>);
        iface.activate_finish = Some(source_plan_activate_finish);
    }
}

unsafe extern "C" fn source_plan_activate_async<T: SourcePlanImpl>(
    p_this: *mut ffi::OmniSourcePlan,
    context: *mut ffi::OmniSourceActivateContext,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let task: gio::Task<ActiveSource> = from_glib_full(gio::ffi::g_task_new(
        p_this.cast(),
        cancellable,
        callback,
        user_data,
    ));

    this_imp.activate_async(
        from_glib_none(context),
        Option::as_ref(&from_glib_none(cancellable)),
        |result| task.return_result(result),
    );
}

unsafe extern "C" fn source_plan_activate_finish(
    p_this: *mut ffi::OmniSourcePlan,
    result: *mut gio::ffi::GAsyncResult,
    err: *mut *mut glib::ffi::GError,
) -> *mut ffi::OmniActiveSource {
    // TODO: De-duplicate all `_finish` impls because they are mostly identical
    assert_ne!(gio::ffi::g_task_is_valid(result, p_this.cast()), 0);
    let task: gio::Task<ActiveSource> =
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
