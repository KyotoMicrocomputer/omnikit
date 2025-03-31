use std::{ffi::c_void, future::Future, ptr};

use glib::{prelude::*, subclass::prelude::*, translate::*, Error};

use crate::{Hasher, Transformer, TransformerContext, TransformerFeatureFlags, TransformerState};

pub trait TransformerImpl: ObjectImpl + Send + Sync {
    fn new_state_async<P: FnOnce(Result<TransformerState, Error>) + 'static>(
        &self,
        cx: &TransformerContext,
        cancellable: Option<&gio::Cancellable>,
        callback: P,
    ) {
        let cx = cx.clone();
        let cancellable = cancellable.cloned();
        default_async!(callback, move || self.new_state_future(cx, cancellable));
    }
    fn new_state_future(
        &self,
        cx: TransformerContext,
        cancellable: Option<gio::Cancellable>,
    ) -> impl Future<Output = Result<TransformerState, glib::Error>>;

    fn eq(&self, rhs: &Transformer) -> bool;
    fn hash(&self, hasher: &Hasher);
    fn feature_flags(&self) -> TransformerFeatureFlags {
        TransformerFeatureFlags::empty()
    }
}

unsafe impl<T: TransformerImpl> IsImplementable<T> for Transformer {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.new_state_async = Some(transformer_new_state_async::<T>);
        iface.new_state_finish = Some(transformer_new_state_finish);
        iface.eq = Some(transformer_eq::<T>);
        iface.hash = Some(transformer_hash::<T>);
        iface.get_feature_flags = Some(transformer_get_feature_flags::<T>);
    }
}

unsafe extern "C" fn transformer_new_state_async<T: TransformerImpl>(
    p_this: *mut ffi::OmniTransformer,
    cx: *mut ffi::OmniTransformerContext,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let task: gio::Task<TransformerState> = from_glib_full(gio::ffi::g_task_new(
        p_this.cast(),
        cancellable,
        callback,
        user_data,
    ));

    this_imp.new_state_async(
        &from_glib_none(cx),
        Option::as_ref(&from_glib_none(cancellable)),
        |result| task.return_result(result),
    );
}

unsafe extern "C" fn transformer_new_state_finish(
    p_this: *mut ffi::OmniTransformer,
    result: *mut gio::ffi::GAsyncResult,
    err: *mut *mut glib::ffi::GError,
) -> *mut ffi::OmniTransformerState {
    assert_ne!(gio::ffi::g_task_is_valid(result, p_this.cast()), 0);
    let task: gio::Task<TransformerState> =
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

unsafe extern "C" fn transformer_eq<T: TransformerImpl>(
    p_this: *mut ffi::OmniTransformer,
    rhs: *mut ffi::OmniTransformer,
) -> glib::ffi::gboolean {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.eq(&from_glib_none(rhs)).into_glib()
}

unsafe extern "C" fn transformer_hash<T: TransformerImpl>(
    p_this: *mut ffi::OmniTransformer,
    hasher: *mut ffi::OmniHasher,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.hash(&from_glib_none(hasher));
}

unsafe extern "C" fn transformer_get_feature_flags<T: TransformerImpl>(
    p_this: *mut ffi::OmniTransformer,
) -> ffi::OmniTransformerFeatureFlags {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.feature_flags().into_glib()
}
