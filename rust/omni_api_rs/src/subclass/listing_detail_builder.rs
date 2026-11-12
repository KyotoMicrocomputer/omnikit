use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::{ColumnBuilder, ListingDetailBuilder};

pub trait ListingDetailBuilderImpl: ObjectImpl {
    fn append_column(&self, field_name: &str) -> Option<ColumnBuilder>;
}

unsafe impl<T: ListingDetailBuilderImpl> IsImplementable<T> for ListingDetailBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.append_column = Some(listing_detail_builder_append_column::<T>);
    }
}

unsafe extern "C" fn listing_detail_builder_append_column<T: ListingDetailBuilderImpl>(
    p_this: *mut ffi::OmniListingDetailBuilder,
    value: *const c_char,
) -> *mut ffi::OmniColumnBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp
        .append_column(&glib::GString::from_glib_borrow(value))
        .into_glib_ptr()
}
