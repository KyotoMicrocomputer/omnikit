use std::{
    ffi::{c_uint, c_void},
    future::Future,
    ptr,
};

use glib::{prelude::*, subclass::prelude::*, translate::*, Error};

use crate::{PortLinkSet, SourcePlan, SourcePlanContext, SourceSnapshot};

pub trait SourceSnapshotImpl: ObjectImpl {
    fn port_attributes(&self, i: c_uint) -> Option<glib::Variant>;

    fn plan_async<P: FnOnce(Result<SourcePlan, Error>) + Send + 'static>(
        &self,
        port_link_sets: Vec<PortLinkSet>,
        cx: SourcePlanContext,
        cancellable: Option<&gio::Cancellable>,
        callback: P,
    ) {
        let cancellable = cancellable.cloned();
        default_async!(callback, move || self.plan_future(
            port_link_sets,
            cx,
            cancellable
        ));
    }
    fn plan_future(
        &self,
        port_link_sets: Vec<PortLinkSet>,
        cx: SourcePlanContext,
        cancellable: Option<gio::Cancellable>,
    ) -> impl Future<Output = Result<SourcePlan, glib::Error>>;

    fn is_stale(&self) -> bool;
    fn num_ports(&self) -> c_uint;
}

unsafe impl<T: SourceSnapshotImpl> IsImplementable<T> for SourceSnapshot {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.dup_port_attributes = Some(source_snapshot_dup_port_attributes::<T>);
        iface.plan_async = Some(source_snapshot_plan_async::<T>);
        iface.plan_finish = Some(source_snapshot_plan_finish);
        iface.get_stale = Some(source_snapshot_get_stale::<T>);
        iface.get_num_ports = Some(source_snapshot_get_num_ports::<T>);
    }
}

unsafe extern "C" fn source_snapshot_dup_port_attributes<T: SourceSnapshotImpl>(
    p_this: *mut ffi::OmniSourceSnapshot,
    i: c_uint,
) -> *mut glib::ffi::GVariant {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.port_attributes(i).into_glib_ptr()
}

unsafe extern "C" fn source_snapshot_plan_async<T: SourceSnapshotImpl>(
    p_this: *mut ffi::OmniSourceSnapshot,
    port_link_sets: *mut glib::ffi::GPtrArray,
    cx: *mut ffi::OmniSourcePlanContext,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let task: gio::Task<SourcePlan> = from_glib_full(gio::ffi::g_task_new(
        p_this.cast(),
        cancellable,
        callback,
        user_data,
    ));

    this_imp.plan_async(
        FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(port_link_sets),
        from_glib_none(cx),
        Option::as_ref(&from_glib_none(cancellable)),
        |result| task.return_result(result),
    );
}

unsafe extern "C" fn source_snapshot_plan_finish(
    p_this: *mut ffi::OmniSourceSnapshot,
    result: *mut gio::ffi::GAsyncResult,
    err: *mut *mut glib::ffi::GError,
) -> *mut ffi::OmniSourcePlan {
    // TODO: De-duplicate all `_finish` impls because they are mostly identical
    assert_ne!(gio::ffi::g_task_is_valid(result, p_this.cast()), 0);
    let task: gio::Task<SourcePlan> = gio::AsyncResult::from_glib_none(result).downcast().unwrap();
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

unsafe extern "C" fn source_snapshot_get_stale<T: SourceSnapshotImpl>(
    p_this: *mut ffi::OmniSourceSnapshot,
) -> glib::ffi::gboolean {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.is_stale().into_glib()
}

unsafe extern "C" fn source_snapshot_get_num_ports<T: SourceSnapshotImpl>(
    p_this: *mut ffi::OmniSourceSnapshot,
) -> c_uint {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.num_ports()
}
