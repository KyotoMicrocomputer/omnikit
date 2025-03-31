use glib::subclass::prelude::*;

use crate::ProjectFilterBuilder;

pub trait ProjectFilterBuilderImpl: ObjectImpl {
    fn output_stream(&self) -> i32;
    fn set_output_stream(&self, value: i32);
}

unsafe impl<T: ProjectFilterBuilderImpl> IsImplementable<T> for ProjectFilterBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.get_output_stream = Some(project_filter_builder_get_output_stream::<T>);
        iface.set_output_stream = Some(project_filter_builder_set_output_stream::<T>);
    }
}

unsafe extern "C" fn project_filter_builder_get_output_stream<T: ProjectFilterBuilderImpl>(
    p_this: *mut ffi::OmniProjectFilterBuilder,
) -> i32 {
    let this_instance = &*(p_this as *mut T::Instance);
    this_instance.imp().output_stream()
}

unsafe extern "C" fn project_filter_builder_set_output_stream<T: ProjectFilterBuilderImpl>(
    p_this: *mut ffi::OmniProjectFilterBuilder,
    value: i32,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    this_instance.imp().set_output_stream(value);
}
