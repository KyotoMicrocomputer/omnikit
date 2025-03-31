use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::{ArrowSchema, StreamRepr, StreamReprType};

pub trait StreamReprImpl: ObjectImpl + Send + Sync {
    fn repr_type(&self) -> StreamReprType;
    fn table_name(&self) -> Option<glib::GString>;
    fn table_schema(&self) -> Option<ArrowSchema>;
}

unsafe impl<T: StreamReprImpl> IsImplementable<T> for StreamRepr {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.get_repr_type = Some(stream_repr_get_repr_type::<T>);
        iface.get_table_name = Some(stream_repr_get_table_name::<T>);
        iface.get_table_schema = Some(stream_repr_get_table_schema::<T>);
    }
}

unsafe extern "C" fn stream_repr_get_repr_type<T: StreamReprImpl>(
    p_this: *mut ffi::OmniStreamRepr,
) -> ffi::OmniStreamReprType {
    let this = &*(p_this as *mut T::Instance);
    let this_imp = this.imp();

    this_imp.repr_type().into_glib()
}

unsafe extern "C" fn stream_repr_get_table_name<T: StreamReprImpl>(
    p_this: *mut ffi::OmniStreamRepr,
) -> *const c_char {
    let this = &*(p_this as *mut T::Instance);
    let this_imp = this.imp();

    stash_ptr!(
        p_this,
        "omni-api-rs-subclass-stream-repr-get-table-name",
        this_imp.table_name().into_glib_ptr(),
        |ptr| glib::ffi::g_free(ptr.cast()),
    )
}

unsafe extern "C" fn stream_repr_get_table_schema<T: StreamReprImpl>(
    p_this: *mut ffi::OmniStreamRepr,
) -> *mut ffi::OmniArrowSchema {
    let this = &*(p_this as *mut T::Instance);
    let this_imp = this.imp();

    stash_ptr!(
        p_this,
        "omni-api-rs-subclass-stream-repr-get-table-schema",
        this_imp.table_schema().into_glib_ptr(),
        |ptr| if !ptr.is_null() {
            glib::gobject_ffi::g_object_unref(ptr.cast())
        },
    )
}
