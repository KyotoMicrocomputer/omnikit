use std::ffi::c_int;

use glib::subclass::prelude::*;

use crate::Hasher;

pub trait HasherImpl: ObjectImpl {
    fn write(&self, _buffer: &[u8]) {}
}

unsafe impl<T: HasherImpl> IsImplementable<T> for Hasher {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.write = Some(hasher_write::<T>);
    }
}

unsafe extern "C" fn hasher_write<T: HasherImpl>(
    p_this: *mut ffi::OmniHasher,
    buffer: *mut u8,
    buffer_len: c_int,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.write(if buffer_len == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(
            buffer,
            c_int::try_into(buffer_len).expect("invalid buffer len"),
        )
    });
}
