use std::ffi::c_uint;

use glib::{subclass::prelude::*, translate::*};

use crate::{Link, LinkArray};

pub trait LinkArrayImpl: ObjectImpl {
    fn link(&self, i: c_uint) -> Option<Link>;
    fn num_links(&self) -> c_uint;
}

unsafe impl<T: LinkArrayImpl> IsImplementable<T> for LinkArray {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.dup_link = Some(link_array_dup_link::<T>);
        iface.get_num_links = Some(link_array_get_num_links::<T>);
    }
}

unsafe extern "C" fn link_array_dup_link<T: LinkArrayImpl>(
    p_this: *mut ffi::OmniLinkArray,
    i: c_uint,
) -> *mut ffi::OmniLink {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.link(i).into_glib_ptr()
}

unsafe extern "C" fn link_array_get_num_links<T: LinkArrayImpl>(
    p_this: *mut ffi::OmniLinkArray,
) -> c_uint {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.num_links()
}
