use glib::{subclass::prelude::*, translate::*};

use crate::ProjectOutputStreamBuilder;

pub trait ProjectOutputStreamBuilderImpl: ObjectImpl {
    fn distinct(&self) -> bool;
    fn set_distinct(&self, value: bool);
}

unsafe impl<T: ProjectOutputStreamBuilderImpl> IsImplementable<T> for ProjectOutputStreamBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.get_distinct = Some(project_output_stream_builder_get_distinct::<T>);
        iface.set_distinct = Some(project_output_stream_builder_set_distinct::<T>);
    }
}

unsafe extern "C" fn project_output_stream_builder_get_distinct<
    T: ProjectOutputStreamBuilderImpl,
>(
    p_this: *mut ffi::OmniProjectOutputStreamBuilder,
) -> glib::ffi::gboolean {
    let this_instance = &*(p_this as *mut T::Instance);
    this_instance.imp().distinct().into_glib()
}

unsafe extern "C" fn project_output_stream_builder_set_distinct<
    T: ProjectOutputStreamBuilderImpl,
>(
    p_this: *mut ffi::OmniProjectOutputStreamBuilder,
    value: glib::ffi::gboolean,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    this_instance.imp().set_distinct(from_glib(value));
}
