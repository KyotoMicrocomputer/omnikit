use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::{
    AggregationDetailBuilder, ArrowRecordBatch, GroupFieldBuilder, ListingDetailBuilder,
    PlotSummaryBuilder, RichSummaryBuilder, SimpleSummaryBuilder, TextDetailBuilder, TrackBuilder,
};

pub trait TrackBuilderImpl: ObjectImpl {
    fn set_group_values(&self, value: Option<ArrowRecordBatch>);
    fn append_group_field_with_sql(&self, sql: &str) -> Option<GroupFieldBuilder>;
    fn add_simple_summary(
        &self,
        stream_handle: i32,
        time_field_name: &str,
    ) -> Option<SimpleSummaryBuilder>;
    fn add_rich_summary(&self) -> Option<RichSummaryBuilder>;
    fn add_plot_summary(&self) -> Option<PlotSummaryBuilder>;
    fn add_listing_detail(
        &self,
        stream_handle: i32,
        time_field_name: &str,
    ) -> Option<ListingDetailBuilder>;
    fn add_aggregation_detail(&self) -> Option<AggregationDetailBuilder>;
    fn add_text_detail(
        &self,
        stream_handle: i32,
        time_field_name: &str,
        text_field_name: &str,
    ) -> Option<TextDetailBuilder>;
    fn set_id(&self, value: Option<&str>);
    fn set_title(&self, value: Option<&str>);
}

unsafe impl<T: TrackBuilderImpl> IsImplementable<T> for TrackBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.set_group_values = Some(track_builder_set_group_values::<T>);
        iface.append_group_field_with_sql = Some(track_builder_append_group_field_with_sql::<T>);
        iface.add_simple_summary = Some(track_builder_add_simple_summary::<T>);
        iface.add_rich_summary = Some(track_builder_add_rich_summary::<T>);
        iface.add_plot_summary = Some(track_builder_add_plot_summary::<T>);
        iface.add_listing_detail = Some(track_builder_add_listing_detail::<T>);
        iface.add_aggregation_detail = Some(track_builder_add_aggregation_detail::<T>);
        iface.add_text_detail = Some(track_builder_add_text_detail::<T>);
        iface.set_id = Some(track_builder_set_id::<T>);
        iface.set_title = Some(track_builder_set_title::<T>);
    }
}

unsafe extern "C" fn track_builder_append_group_field_with_sql<T: TrackBuilderImpl>(
    p_this: *mut ffi::OmniTrackBuilder,
    sql: *const c_char,
) -> *mut ffi::OmniGroupFieldBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp
        .append_group_field_with_sql(&glib::GString::from_glib_borrow(sql))
        .into_glib_ptr()
}

unsafe extern "C" fn track_builder_set_group_values<T: TrackBuilderImpl>(
    p_this: *mut ffi::OmniTrackBuilder,
    value: *mut ffi::OmniArrowRecordBatch,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.set_group_values(from_glib_none(value));
}

unsafe extern "C" fn track_builder_add_simple_summary<T: TrackBuilderImpl>(
    p_this: *mut ffi::OmniTrackBuilder,
    stream_handle: i32,
    time_field_name: *const c_char,
) -> *mut ffi::OmniSimpleSummaryBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp
        .add_simple_summary(
            stream_handle,
            &glib::GString::from_glib_borrow(time_field_name),
        )
        .into_glib_ptr()
}

unsafe extern "C" fn track_builder_add_rich_summary<T: TrackBuilderImpl>(
    p_this: *mut ffi::OmniTrackBuilder,
) -> *mut ffi::OmniRichSummaryBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.add_rich_summary().into_glib_ptr()
}

unsafe extern "C" fn track_builder_add_plot_summary<T: TrackBuilderImpl>(
    p_this: *mut ffi::OmniTrackBuilder,
) -> *mut ffi::OmniPlotSummaryBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.add_plot_summary().into_glib_ptr()
}

unsafe extern "C" fn track_builder_add_listing_detail<T: TrackBuilderImpl>(
    p_this: *mut ffi::OmniTrackBuilder,
    stream_handle: i32,
    time_field_name: *const c_char,
) -> *mut ffi::OmniListingDetailBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp
        .add_listing_detail(
            stream_handle,
            &glib::GString::from_glib_borrow(time_field_name),
        )
        .into_glib_ptr()
}

unsafe extern "C" fn track_builder_add_aggregation_detail<T: TrackBuilderImpl>(
    p_this: *mut ffi::OmniTrackBuilder,
) -> *mut ffi::OmniAggregationDetailBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.add_aggregation_detail().into_glib_ptr()
}

unsafe extern "C" fn track_builder_add_text_detail<T: TrackBuilderImpl>(
    p_this: *mut ffi::OmniTrackBuilder,
    stream_handle: i32,
    time_field_name: *const c_char,
    text_field_name: *const c_char,
) -> *mut ffi::OmniTextDetailBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp
        .add_text_detail(
            stream_handle,
            &glib::GString::from_glib_borrow(time_field_name),
            &glib::GString::from_glib_borrow(text_field_name),
        )
        .into_glib_ptr()
}

unsafe extern "C" fn track_builder_set_id<T: TrackBuilderImpl>(
    p_this: *mut ffi::OmniTrackBuilder,
    value: *const c_char,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let value_borrowed;
    this_imp.set_id(if value.is_null() {
        None
    } else {
        value_borrowed = glib::GString::from_glib_borrow(value);
        Some(&value_borrowed)
    });
}

unsafe extern "C" fn track_builder_set_title<T: TrackBuilderImpl>(
    p_this: *mut ffi::OmniTrackBuilder,
    value: *const c_char,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let value_borrowed;
    this_imp.set_title(if value.is_null() {
        None
    } else {
        value_borrowed = glib::GString::from_glib_borrow(value);
        Some(&value_borrowed)
    });
}
