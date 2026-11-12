use glib::{subclass::prelude::*, translate::*};

use crate::{GroupFieldBuilder, SortOrder};

pub trait GroupFieldBuilderImpl: ObjectImpl {
    fn append_sort_key_with_sql(&self, sql: &str, sort_order: SortOrder);
}

unsafe impl<T: GroupFieldBuilderImpl> IsImplementable<T> for GroupFieldBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        iface.as_mut().append_sort_key_with_sql =
            Some(group_field_builder_append_sort_key_with_sql::<T>);
    }
}

unsafe extern "C" fn group_field_builder_append_sort_key_with_sql<T: GroupFieldBuilderImpl>(
    p_this: *mut ffi::OmniGroupFieldBuilder,
    sql: *const std::ffi::c_char,
    sort_order: ffi::OmniSortOrder,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.append_sort_key_with_sql(
        &glib::GString::from_glib_borrow(sql),
        glib::translate::from_glib(sort_order),
    );
}
