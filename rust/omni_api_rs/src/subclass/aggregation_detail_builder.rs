use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::{
    AggregationColumnBuilder, AggregationDetailBuilder, AggregationGroupBuilder,
    AggregationStreamBuilder, GroupColumnBuilder,
};

pub trait AggregationDetailBuilderImpl: ObjectImpl {
    fn append_stream(
        &self,
        stream_handle: i32,
        time_field_name: &str,
    ) -> Option<AggregationStreamBuilder>;

    fn append_column_with_sql(&self, sql_expr: &str) -> Option<AggregationColumnBuilder>;

    fn append_group_column(&self) -> Option<GroupColumnBuilder>;

    fn append_field_group(&self, field_name: &str) -> Option<AggregationGroupBuilder>;

    fn set_call_stack_field_name(&self, value: Option<&str>);

    fn set_call_stack_missing_caller(&self, value: bool);
}

unsafe impl<T: AggregationDetailBuilderImpl> IsImplementable<T> for AggregationDetailBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.append_stream = Some(aggregation_detail_builder_append_stream::<T>);
        iface.append_column_with_sql = Some(aggregation_detail_builder_append_column_with_sql::<T>);
        iface.append_group_column = Some(aggregation_detail_builder_append_group_column::<T>);
        iface.append_field_group = Some(aggregation_detail_builder_append_field_group::<T>);
        iface.set_call_stack_field_name =
            Some(aggregation_detail_builder_set_call_stack_field_name::<T>);
        iface.set_call_stack_missing_caller =
            Some(aggregation_detail_builder_set_call_stack_missing_caller::<T>);
    }
}

unsafe extern "C" fn aggregation_detail_builder_append_stream<T: AggregationDetailBuilderImpl>(
    p_this: *mut ffi::OmniAggregationDetailBuilder,
    stream_handle: i32,
    time_field_name: *const c_char,
) -> *mut ffi::OmniAggregationStreamBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp
        .append_stream(
            stream_handle,
            &glib::GString::from_glib_borrow(time_field_name),
        )
        .into_glib_ptr()
}

unsafe extern "C" fn aggregation_detail_builder_append_column_with_sql<
    T: AggregationDetailBuilderImpl,
>(
    p_this: *mut ffi::OmniAggregationDetailBuilder,
    value: *const c_char,
) -> *mut ffi::OmniAggregationColumnBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp
        .append_column_with_sql(&glib::GString::from_glib_borrow(value))
        .into_glib_ptr()
}

unsafe extern "C" fn aggregation_detail_builder_append_group_column<
    T: AggregationDetailBuilderImpl,
>(
    p_this: *mut ffi::OmniAggregationDetailBuilder,
) -> *mut ffi::OmniGroupColumnBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.append_group_column().into_glib_ptr()
}

unsafe extern "C" fn aggregation_detail_builder_append_field_group<
    T: AggregationDetailBuilderImpl,
>(
    p_this: *mut ffi::OmniAggregationDetailBuilder,
    value: *const c_char,
) -> *mut ffi::OmniAggregationGroupBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp
        .append_field_group(&glib::GString::from_glib_borrow(value))
        .into_glib_ptr()
}

unsafe extern "C" fn aggregation_detail_builder_set_call_stack_field_name<
    T: AggregationDetailBuilderImpl,
>(
    p_this: *mut ffi::OmniAggregationDetailBuilder,
    value: *const c_char,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let value_borrowed;
    this_imp.set_call_stack_field_name(if value.is_null() {
        None
    } else {
        value_borrowed = glib::GString::from_glib_borrow(value);
        Some(&value_borrowed)
    });
}

unsafe extern "C" fn aggregation_detail_builder_set_call_stack_missing_caller<
    T: AggregationDetailBuilderImpl,
>(
    p_this: *mut ffi::OmniAggregationDetailBuilder,
    value: glib::ffi::gboolean,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.set_call_stack_missing_caller(from_glib(value));
}
