use glib::{subclass::prelude::*, translate::*};

use crate::ActiveSource;

pub trait ActiveSourceImpl: ObjectImpl {
    fn start(&self, cancellable: Option<gio::Cancellable>);

    fn stop(&self);

    fn cancel(&self);
}

unsafe impl<T: ActiveSourceImpl> IsImplementable<T> for ActiveSource {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.start = Some(active_source_start::<T>);
        iface.stop = Some(active_source_stop::<T>);
        iface.cancel = Some(active_source_cancel::<T>);
    }
}

unsafe extern "C" fn active_source_start<T: ActiveSourceImpl>(
    p_this: *mut ffi::OmniActiveSource,
    cancellable: *mut gio::ffi::GCancellable,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.start(from_glib_none(cancellable));
}

unsafe extern "C" fn active_source_stop<T: ActiveSourceImpl>(p_this: *mut ffi::OmniActiveSource) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.stop();
}

unsafe extern "C" fn active_source_cancel<T: ActiveSourceImpl>(p_this: *mut ffi::OmniActiveSource) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.cancel();
}
