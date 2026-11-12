use std::{ffi::c_void, future::Future, ptr};

use gio::GioFutureResult;
use glib::{prelude::*, subclass::prelude::*, thread_guard::ThreadGuard, translate::*, Error};

use crate::{PipelineSnapshot, TransformUniformContext, TransformerContext, TransformerState};

pub trait TransformerStateImpl: ObjectImpl + Send + Sync {
    fn init_async<P: FnOnce(Result<(), Error>) + 'static>(
        &self,
        cancellable: Option<&gio::Cancellable>,
        callback: P,
    ) {
        let cancellable = cancellable.cloned();
        default_async!(callback, move || self.init_future(cancellable));
    }
    fn init_future(
        &self,
        cancellable: Option<gio::Cancellable>,
    ) -> impl Future<Output = Result<(), glib::Error>>;

    fn deserialize_from_async<P: FnOnce(Result<(), Error>) + 'static>(
        &self,
        buffer: &glib::Bytes,
        cancellable: Option<&gio::Cancellable>,
        callback: P,
    ) {
        let buffer = buffer.clone();
        let cancellable = cancellable.cloned();
        default_async!(callback, move || self
            .deserialize_from_future(buffer, cancellable));
    }
    fn deserialize_from_future(
        &self,
        buffer: glib::Bytes,
        cancellable: Option<gio::Cancellable>,
    ) -> impl Future<Output = Result<(), glib::Error>> {
        self.parent_deserialize_from_future(buffer, cancellable)
    }

    fn serialize_async<P: FnOnce(Result<glib::Bytes, Error>) + 'static>(
        &self,
        cancellable: Option<&gio::Cancellable>,
        callback: P,
    ) {
        let cancellable = cancellable.cloned();
        default_async!(callback, move || self.serialize_future(cancellable));
    }
    fn serialize_future(
        &self,
        cancellable: Option<gio::Cancellable>,
    ) -> impl Future<Output = Result<glib::Bytes, glib::Error>> {
        self.parent_serialize_future(cancellable)
    }

    fn copy_from_async<P: FnOnce(Result<(), Error>) + 'static>(
        &self,
        source_state: TransformerState,
        cancellable: Option<&gio::Cancellable>,
        callback: P,
    ) {
        let cancellable = cancellable.cloned();
        default_async!(callback, move || self
            .copy_from_future(source_state, cancellable));
    }
    fn copy_from_future(
        &self,
        source_state: TransformerState,
        cancellable: Option<gio::Cancellable>,
    ) -> impl Future<Output = Result<(), glib::Error>>;

    fn take_from_async<P: FnOnce(Result<(), Error>) + 'static>(
        &self,
        source_state: TransformerState,
        cancellable: Option<&gio::Cancellable>,
        callback: P,
    ) {
        let cancellable = cancellable.cloned();
        default_async!(callback, move || self
            .take_from_future(source_state, cancellable));
    }
    fn take_from_future(
        &self,
        source_state: TransformerState,
        cancellable: Option<gio::Cancellable>,
    ) -> impl Future<Output = Result<(), glib::Error>> {
        self.copy_from_future(source_state, cancellable)
    }

    fn transform_uniform_async<P: FnOnce(Result<(), Error>) + 'static>(
        &self,
        transform_cx: &TransformUniformContext,
        transformer_context: &TransformerContext,
        snapshot: &PipelineSnapshot,
        cancellable: Option<&gio::Cancellable>,
        callback: P,
    ) {
        let transform_cx = transform_cx.clone();
        let transformer_context = transformer_context.clone();
        let snapshot = snapshot.clone();
        let cancellable = cancellable.cloned();
        default_async!(callback, move || self.transform_uniform_future(
            transform_cx,
            transformer_context,
            snapshot,
            cancellable
        ));
    }
    fn transform_uniform_future(
        &self,
        transform_cx: TransformUniformContext,
        transformer_context: TransformerContext,
        snapshot: PipelineSnapshot,
        cancellable: Option<gio::Cancellable>,
    ) -> impl Future<Output = Result<(), glib::Error>>;
}

pub trait TransformerStateImplExt: TransformerStateImpl + ObjectSubclass {
    fn parent_serialize_future(
        &self,
        cancellable: Option<gio::Cancellable>,
    ) -> impl Future<Output = Result<glib::Bytes, Error>> + Unpin + 'static {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TransformerState>()
                as *const ffi::OmniTransformerStateIface;
            let serialize_async = (*parent_iface)
                .serialize_async
                .expect("no parent \"serialize_async\" implementation");

            unsafe extern "C" fn parent_serialize_future_callback<T>(
                _source_object: *mut glib::gobject_ffi::GObject,
                res: *mut gio::ffi::GAsyncResult,
                user_data: glib::ffi::gpointer,
            ) where
                T: TransformerStateImpl,
            {
                let type_data = T::type_data();
                let parent_iface = type_data.as_ref().parent_interface::<TransformerState>()
                    as *const ffi::OmniTransformerStateIface;
                let serialize_finish = (*parent_iface)
                    .serialize_finish
                    .expect("no parent \"serialize_finish\" implementation");

                let user_data: Box<ThreadGuard<GioFutureResult<Result<glib::Bytes, Error>>>> =
                    Box::from_raw(user_data as *mut _);
                let user_data = user_data.into_inner();

                let mut error = ptr::null_mut();
                let ret = serialize_finish(_source_object as *mut _, res, &mut error);
                let result = if error.is_null() {
                    Ok(from_glib_full(ret))
                } else {
                    Err(from_glib_full(error))
                };
                user_data.resolve(result);
            }

            gio::GioFuture::new(&*self.obj(), move |obj, _, res| {
                let user_data: Box<ThreadGuard<GioFutureResult<Result<glib::Bytes, Error>>>> =
                    Box::new(ThreadGuard::new(res));
                serialize_async(
                    obj.unsafe_cast_ref::<TransformerState>().to_glib_none().0,
                    cancellable.to_glib_none().0,
                    Some(parent_serialize_future_callback::<Self>),
                    Box::into_raw(user_data) as *mut _,
                );
            })
        }
    }

    fn parent_deserialize_from_future(
        &self,
        buffer: glib::Bytes,
        cancellable: Option<gio::Cancellable>,
    ) -> impl Future<Output = Result<(), Error>> + Unpin + 'static {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<TransformerState>()
                as *const ffi::OmniTransformerStateIface;
            let deserialize_from_async = (*parent_iface)
                .deserialize_from_async
                .expect("no parent \"deserialize_from_async\" implementation");

            unsafe extern "C" fn parent_deserialize_from_future_callback<T>(
                _source_object: *mut glib::gobject_ffi::GObject,
                res: *mut gio::ffi::GAsyncResult,
                user_data: glib::ffi::gpointer,
            ) where
                T: TransformerStateImpl,
            {
                let type_data = T::type_data();
                let parent_iface = type_data.as_ref().parent_interface::<TransformerState>()
                    as *const ffi::OmniTransformerStateIface;
                let deserialize_from_finish = (*parent_iface)
                    .deserialize_from_finish
                    .expect("no parent \"deserialize_from_finish\" implementation");

                let user_data: Box<ThreadGuard<GioFutureResult<Result<(), Error>>>> =
                    Box::from_raw(user_data as *mut _);
                let user_data = user_data.into_inner();

                let mut error = ptr::null_mut();
                deserialize_from_finish(_source_object as *mut _, res, &mut error);
                let result = if error.is_null() {
                    Ok(())
                } else {
                    Err(from_glib_full(error))
                };
                user_data.resolve(result);
            }

            gio::GioFuture::new(&*self.obj(), move |obj, _, res| {
                let user_data: Box<ThreadGuard<GioFutureResult<Result<(), Error>>>> =
                    Box::new(ThreadGuard::new(res));
                deserialize_from_async(
                    obj.unsafe_cast_ref::<TransformerState>().to_glib_none().0,
                    buffer.to_glib_none().0,
                    cancellable.to_glib_none().0,
                    Some(parent_deserialize_from_future_callback::<Self>),
                    Box::into_raw(user_data) as *mut _,
                );
            })
        }
    }
}

impl<T: TransformerStateImpl> TransformerStateImplExt for T {}

unsafe impl<T: TransformerStateImpl> IsImplementable<T> for TransformerState {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.init_async = Some(transformer_state_init_async::<T>);
        iface.init_finish = Some(transformer_state_init_finish);
        iface.deserialize_from_async = Some(transformer_state_deserialize_from_async::<T>);
        iface.deserialize_from_finish = Some(transformer_state_deserialize_from_finish);
        iface.serialize_async = Some(transformer_state_serialize_async::<T>);
        iface.serialize_finish = Some(transformer_state_serialize_finish);
        iface.copy_from_async = Some(transformer_state_copy_from_async::<T>);
        iface.copy_from_finish = Some(transformer_state_copy_from_finish);
        iface.take_from_async = Some(transformer_state_take_from_async::<T>);
        iface.take_from_finish = Some(transformer_state_take_from_finish);
        iface.transform_uniform_async = Some(transformer_state_transform_uniform_async::<T>);
        iface.transform_uniform_finish = Some(transformer_state_transform_uniform_finish);
    }
}

unsafe extern "C" fn transformer_state_init_async<T: TransformerStateImpl>(
    p_this: *mut ffi::OmniTransformerState,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let task: gio::Task<u8> = from_glib_full(gio::ffi::g_task_new(
        p_this.cast(),
        cancellable,
        callback,
        user_data,
    ));

    this_imp.init_async(Option::as_ref(&from_glib_none(cancellable)), |result| {
        task.return_result(result.map(|()| 0))
    });
}

unsafe extern "C" fn transformer_state_init_finish(
    p_this: *mut ffi::OmniTransformerState,
    result: *mut gio::ffi::GAsyncResult,
    err: *mut *mut glib::ffi::GError,
) {
    // TODO: De-duplicate all `_finish` impls because they are mostly identical
    assert_ne!(gio::ffi::g_task_is_valid(result, p_this.cast()), 0);
    let task: gio::Task<u8> = gio::AsyncResult::from_glib_none(result).downcast().unwrap();
    match task.propagate() {
        Ok(_) => {}
        Err(e) => {
            if !err.is_null() {
                *err = e.into_glib_ptr();
            }
        }
    }
}

unsafe extern "C" fn transformer_state_deserialize_from_async<T: TransformerStateImpl>(
    p_this: *mut ffi::OmniTransformerState,
    buffer: *mut glib::ffi::GBytes,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let task: gio::Task<u8> = from_glib_full(gio::ffi::g_task_new(
        p_this.cast(),
        cancellable,
        callback,
        user_data,
    ));

    this_imp.deserialize_from_async(
        &from_glib_none(buffer),
        Option::as_ref(&from_glib_none(cancellable)),
        |result| task.return_result(result.map(|()| 0)),
    );
}

unsafe extern "C" fn transformer_state_deserialize_from_finish(
    p_this: *mut ffi::OmniTransformerState,
    result: *mut gio::ffi::GAsyncResult,
    err: *mut *mut glib::ffi::GError,
) {
    // TODO: De-duplicate all `_finish` impls because they are mostly identical
    assert_ne!(gio::ffi::g_task_is_valid(result, p_this.cast()), 0);
    let task: gio::Task<u8> = gio::AsyncResult::from_glib_none(result).downcast().unwrap();
    match task.propagate() {
        Ok(_) => {}
        Err(e) => {
            if !err.is_null() {
                *err = e.into_glib_ptr();
            }
        }
    }
}

unsafe extern "C" fn transformer_state_serialize_async<T: TransformerStateImpl>(
    p_this: *mut ffi::OmniTransformerState,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let task: gio::Task<glib::Bytes> = from_glib_full(gio::ffi::g_task_new(
        p_this.cast(),
        cancellable,
        callback,
        user_data,
    ));

    this_imp.serialize_async(Option::as_ref(&from_glib_none(cancellable)), |result| {
        task.return_result(result)
    });
}

unsafe extern "C" fn transformer_state_serialize_finish(
    p_this: *mut ffi::OmniTransformerState,
    result: *mut gio::ffi::GAsyncResult,
    err: *mut *mut glib::ffi::GError,
) -> *mut glib::ffi::GBytes {
    // TODO: De-duplicate all `_finish` impls because they are mostly identical
    assert_ne!(gio::ffi::g_task_is_valid(result, p_this.cast()), 0);
    let task: gio::Task<glib::Bytes> = gio::AsyncResult::from_glib_none(result).downcast().unwrap();
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

unsafe extern "C" fn transformer_state_copy_from_async<T: TransformerStateImpl>(
    p_this: *mut ffi::OmniTransformerState,
    source_state: *mut ffi::OmniTransformerState,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let task: gio::Task<u8> = from_glib_full(gio::ffi::g_task_new(
        p_this.cast(),
        cancellable,
        callback,
        user_data,
    ));

    this_imp.copy_from_async(
        from_glib_none(source_state),
        Option::as_ref(&from_glib_none(cancellable)),
        |result| task.return_result(result.map(|()| 0)),
    );
}

unsafe extern "C" fn transformer_state_copy_from_finish(
    p_this: *mut ffi::OmniTransformerState,
    result: *mut gio::ffi::GAsyncResult,
    err: *mut *mut glib::ffi::GError,
) {
    // TODO: De-duplicate all `_finish` impls because they are mostly identical
    assert_ne!(gio::ffi::g_task_is_valid(result, p_this.cast()), 0);
    let task: gio::Task<u8> = gio::AsyncResult::from_glib_none(result).downcast().unwrap();
    match task.propagate() {
        Ok(_) => {}
        Err(e) => {
            if !err.is_null() {
                *err = e.into_glib_ptr();
            }
        }
    }
}

unsafe extern "C" fn transformer_state_take_from_async<T: TransformerStateImpl>(
    p_this: *mut ffi::OmniTransformerState,
    source_state: *mut ffi::OmniTransformerState,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let task: gio::Task<u8> = from_glib_full(gio::ffi::g_task_new(
        p_this.cast(),
        cancellable,
        callback,
        user_data,
    ));

    this_imp.take_from_async(
        from_glib_none(source_state),
        Option::as_ref(&from_glib_none(cancellable)),
        |result| task.return_result(result.map(|()| 0)),
    );
}

unsafe extern "C" fn transformer_state_take_from_finish(
    p_this: *mut ffi::OmniTransformerState,
    result: *mut gio::ffi::GAsyncResult,
    err: *mut *mut glib::ffi::GError,
) {
    // TODO: De-duplicate all `_finish` impls because they are mostly identical
    assert_ne!(gio::ffi::g_task_is_valid(result, p_this.cast()), 0);
    let task: gio::Task<u8> = gio::AsyncResult::from_glib_none(result).downcast().unwrap();
    match task.propagate() {
        Ok(_) => {}
        Err(e) => {
            if !err.is_null() {
                *err = e.into_glib_ptr();
            }
        }
    }
}

unsafe extern "C" fn transformer_state_transform_uniform_async<T: TransformerStateImpl>(
    p_this: *mut ffi::OmniTransformerState,
    transform_cx: *mut ffi::OmniTransformUniformContext,
    transformer_context: *mut ffi::OmniTransformerContext,
    snapshot: *mut ffi::OmniPipelineSnapshot,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let task: gio::Task<u8> = from_glib_full(gio::ffi::g_task_new(
        p_this.cast(),
        cancellable,
        callback,
        user_data,
    ));

    this_imp.transform_uniform_async(
        &from_glib_none(transform_cx),
        &from_glib_none(transformer_context),
        &from_glib_none(snapshot),
        Option::as_ref(&from_glib_none(cancellable)),
        |result| task.return_result(result.map(|()| 0)),
    );
}

unsafe extern "C" fn transformer_state_transform_uniform_finish(
    p_this: *mut ffi::OmniTransformerState,
    result: *mut gio::ffi::GAsyncResult,
    err: *mut *mut glib::ffi::GError,
) {
    // TODO: De-duplicate all `_finish` impls because they are mostly identical
    assert_ne!(gio::ffi::g_task_is_valid(result, p_this.cast()), 0);
    let task: gio::Task<u8> = gio::AsyncResult::from_glib_none(result).downcast().unwrap();
    match task.propagate() {
        Ok(_) => {}
        Err(e) => {
            if !err.is_null() {
                *err = e.into_glib_ptr();
            }
        }
    }
}
