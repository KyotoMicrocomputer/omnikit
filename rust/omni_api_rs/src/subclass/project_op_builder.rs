use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::{
    ProjectFieldBuilder, ProjectFilterBuilder, ProjectInputStreamBuilder, ProjectJoinBuilder,
    ProjectOpBuilder, ProjectOutputStreamBuilder, ProjectUnionBuilder,
};

pub trait ProjectOpBuilderImpl: ObjectImpl {
    fn name(&self) -> Option<glib::GString>;
    fn set_name(&self, value: Option<&str>);
    fn append_output_stream(&self, stream_handle: i32) -> ProjectOutputStreamBuilder;
    fn append_input_stream(
        &self,
        stream_handle: i32,
        time_field: &str,
    ) -> (u32, ProjectInputStreamBuilder);
    fn filter_input_with_sql(&self, input_node: u32, sql: &str) -> (u32, ProjectFilterBuilder);
    fn union_inputs(&self, input_node1: u32, input_node2: u32) -> (u32, ProjectUnionBuilder);
    fn join_inputs(&self, input_node1: u32, input_node2: u32) -> (u32, ProjectJoinBuilder);
    fn set_source_input(&self, input_node: u32);
    fn append_field_copy_all(&self, input_stream_index: u32);
    fn append_field_copy(&self, input_stream_index: u32, name: &str) -> ProjectFieldBuilder;
    fn append_field_with_sql(&self, sql: &str) -> ProjectFieldBuilder;
}

unsafe impl<T: ProjectOpBuilderImpl> IsImplementable<T> for ProjectOpBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.get_name = Some(project_op_builder_get_name::<T>);
        iface.set_name = Some(project_op_builder_set_name::<T>);
        iface.append_output_stream = Some(project_op_builder_append_output_stream::<T>);
        iface.append_input_stream = Some(project_op_builder_append_input_stream::<T>);
        iface.filter_input_with_sql = Some(project_op_builder_filter_input_with_sql::<T>);
        iface.union_inputs = Some(project_op_builder_union_inputs::<T>);
        iface.join_inputs = Some(project_op_builder_join_inputs::<T>);
        iface.set_source_input = Some(project_op_builder_set_source_input::<T>);
        iface.append_field_copy_all = Some(project_op_builder_append_field_copy_all::<T>);
        iface.append_field_copy = Some(project_op_builder_append_field_copy::<T>);
        iface.append_field_with_sql = Some(project_op_builder_append_field_with_sql::<T>);
    }
}

unsafe extern "C" fn project_op_builder_get_name<T: ProjectOpBuilderImpl>(
    p_this: *mut ffi::OmniProjectOpBuilder,
) -> *const c_char {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    stash_ptr!(
        p_this,
        "omni-api-rs-subclass-project-op-builder-get-name",
        this_imp.name().into_glib_ptr(),
        |ptr| glib::ffi::g_free(ptr.cast()),
    )
}

unsafe extern "C" fn project_op_builder_set_name<T: ProjectOpBuilderImpl>(
    p_this: *mut ffi::OmniProjectOpBuilder,
    value: *const c_char,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let value_borrowed;
    this_imp.set_name(if value.is_null() {
        None
    } else {
        value_borrowed = glib::GString::from_glib_borrow(value);
        Some(&value_borrowed)
    });
}

unsafe extern "C" fn project_op_builder_append_output_stream<T: ProjectOpBuilderImpl>(
    p_this: *mut ffi::OmniProjectOpBuilder,
    stream_handle: i32,
    p_builder: *mut *mut ffi::OmniProjectOutputStreamBuilder,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    let builder = this_imp.append_output_stream(stream_handle);
    if !p_builder.is_null() {
        *p_builder = builder.into_glib_ptr();
    }
}

unsafe extern "C" fn project_op_builder_append_input_stream<T: ProjectOpBuilderImpl>(
    p_this: *mut ffi::OmniProjectOpBuilder,
    stream_handle: i32,
    time_field: *const c_char,
    p_builder: *mut *mut ffi::OmniProjectInputStreamBuilder,
) -> u32 {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    let (node, builder) =
        this_imp.append_input_stream(stream_handle, &glib::GString::from_glib_borrow(time_field));
    if !p_builder.is_null() {
        *p_builder = builder.into_glib_ptr();
    }
    node
}

unsafe extern "C" fn project_op_builder_filter_input_with_sql<T: ProjectOpBuilderImpl>(
    p_this: *mut ffi::OmniProjectOpBuilder,
    input_node: u32,
    sql: *const c_char,
    p_builder: *mut *mut ffi::OmniProjectFilterBuilder,
) -> u32 {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    let (node, builder) =
        this_imp.filter_input_with_sql(input_node, &glib::GString::from_glib_borrow(sql));
    if !p_builder.is_null() {
        *p_builder = builder.into_glib_ptr();
    }
    node
}

unsafe extern "C" fn project_op_builder_union_inputs<T: ProjectOpBuilderImpl>(
    p_this: *mut ffi::OmniProjectOpBuilder,
    input_node1: u32,
    input_node2: u32,
    p_builder: *mut *mut ffi::OmniProjectUnionBuilder,
) -> u32 {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    let (node, builder) = this_imp.union_inputs(input_node1, input_node2);
    if !p_builder.is_null() {
        *p_builder = builder.into_glib_ptr();
    }
    node
}

unsafe extern "C" fn project_op_builder_join_inputs<T: ProjectOpBuilderImpl>(
    p_this: *mut ffi::OmniProjectOpBuilder,
    input_node1: u32,
    input_node2: u32,
    p_builder: *mut *mut ffi::OmniProjectJoinBuilder,
) -> u32 {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    let (node, builder) = this_imp.join_inputs(input_node1, input_node2);
    if !p_builder.is_null() {
        *p_builder = builder.into_glib_ptr();
    }
    node
}

unsafe extern "C" fn project_op_builder_set_source_input<T: ProjectOpBuilderImpl>(
    p_this: *mut ffi::OmniProjectOpBuilder,
    input_node: u32,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    this_instance.imp().set_source_input(input_node);
}

unsafe extern "C" fn project_op_builder_append_field_copy_all<T: ProjectOpBuilderImpl>(
    p_this: *mut ffi::OmniProjectOpBuilder,
    input_stream_index: u32,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    this_instance
        .imp()
        .append_field_copy_all(input_stream_index);
}

unsafe extern "C" fn project_op_builder_append_field_copy<T: ProjectOpBuilderImpl>(
    p_this: *mut ffi::OmniProjectOpBuilder,
    input_stream_index: u32,
    name: *const c_char,
    p_builder: *mut *mut ffi::OmniProjectFieldBuilder,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let builder = this_instance
        .imp()
        .append_field_copy(input_stream_index, &glib::GString::from_glib_borrow(name));
    if !p_builder.is_null() {
        *p_builder = builder.into_glib_ptr();
    }
}

unsafe extern "C" fn project_op_builder_append_field_with_sql<T: ProjectOpBuilderImpl>(
    p_this: *mut ffi::OmniProjectOpBuilder,
    sql: *const c_char,
    p_builder: *mut *mut ffi::OmniProjectFieldBuilder,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let builder = this_instance
        .imp()
        .append_field_with_sql(&glib::GString::from_glib_borrow(sql));
    if !p_builder.is_null() {
        *p_builder = builder.into_glib_ptr();
    }
}
