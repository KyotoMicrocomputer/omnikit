use std::ffi::c_void;

use glib::subclass::prelude::*;

use crate::ArrowSchema;

pub trait ArrowSchemaImpl: ObjectImpl + Send + Sync {
    /// Export the Arrow schema as the FFI format described in
    /// [the Arrow C data interface][1].
    ///
    /// # Safety
    ///
    /// `c_abi_schema` should refer to an object of type `FFI_ArrowSchema` and
    /// be safe for writing.
    ///
    /// [1]: https://arrow.apache.org/docs/format/CDataInterface.html
    unsafe fn export(&self, c_abi_schema: *mut c_void);
}

unsafe impl<T: ArrowSchemaImpl> IsImplementable<T> for ArrowSchema {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.export = Some(arrow_schema_export::<T>);
    }
}

unsafe extern "C" fn arrow_schema_export<T: ArrowSchemaImpl>(
    p_this: *mut ffi::OmniArrowSchema,
    c_abi_schema: *mut c_void,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.export(c_abi_schema);
}
