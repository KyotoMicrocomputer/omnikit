use std::ffi::c_void;

use glib::{subclass::prelude::*, translate::*};

use crate::{ArrowRecordBatch, ArrowSchema};

pub trait ArrowRecordBatchImpl: ObjectImpl + Send + Sync {
    /// Export the Arrow array as the FFI format described in
    /// [the Arrow C data interface][1].
    ///
    /// # Safety
    ///
    /// `c_abi_array` and `c_abi_schema` should refer to objects of types
    /// `FFI_ArrowArray` and `FFI_ArrowSchema` and be safe for writing.
    ///
    /// [1]: https://arrow.apache.org/docs/format/CDataInterface.html
    unsafe fn export(&self, c_abi_array: *mut c_void, c_abi_schema: *mut c_void);

    fn schema(&self) -> Option<ArrowSchema>;
}

unsafe impl<T: ArrowRecordBatchImpl> IsImplementable<T> for ArrowRecordBatch {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.export = Some(arrow_record_batch_export::<T>);
        iface.dup_schema = Some(arrow_record_batch_dup_schema::<T>);
    }
}

unsafe extern "C" fn arrow_record_batch_export<T: ArrowRecordBatchImpl>(
    p_this: *mut ffi::OmniArrowRecordBatch,
    c_abi_array: *mut c_void,
    c_abi_schema: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.export(c_abi_array, c_abi_schema);
}

unsafe extern "C" fn arrow_record_batch_dup_schema<T: ArrowRecordBatchImpl>(
    p_this: *mut ffi::OmniArrowRecordBatch,
) -> *mut ffi::OmniArrowSchema {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.schema().into_glib_ptr()
}
