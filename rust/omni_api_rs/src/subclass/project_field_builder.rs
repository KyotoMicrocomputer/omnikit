use glib::subclass::prelude::*;

use crate::ProjectFieldBuilder;

pub trait ProjectFieldBuilderImpl: ObjectImpl {
    fn output_stream(&self) -> i32;
    fn set_output_stream(&self, value: i32);
}

unsafe impl<T: ProjectFieldBuilderImpl> IsImplementable<T> for ProjectFieldBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.get_output_stream = Some(project_field_builder_get_output_stream::<T>);
        iface.set_output_stream = Some(project_field_builder_set_output_stream::<T>);
    }
}

unsafe extern "C" fn project_field_builder_get_output_stream<T: ProjectFieldBuilderImpl>(
    p_this: *mut ffi::OmniProjectFieldBuilder,
) -> i32 {
    let this_instance = &*(p_this as *mut T::Instance);
    this_instance.imp().output_stream()
}

unsafe extern "C" fn project_field_builder_set_output_stream<T: ProjectFieldBuilderImpl>(
    p_this: *mut ffi::OmniProjectFieldBuilder,
    value: i32,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    this_instance.imp().set_output_stream(value);
}
