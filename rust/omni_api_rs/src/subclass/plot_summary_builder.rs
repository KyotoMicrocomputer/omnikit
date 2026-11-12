use std::ffi::{c_char, c_double};

use glib::{subclass::prelude::*, translate::*};

use crate::{
    PlotLayerBuilder, PlotLayerType, PlotSummaryBuilder, ValueLineLayerBuilder, ValueTickBuilder,
};

pub trait PlotSummaryBuilderImpl: ObjectImpl {
    fn add_plot_layer(
        &self,
        stream_handle: i32,
        time_field_name: &str,
        plot_type: PlotLayerType,
    ) -> Option<PlotLayerBuilder>;
    fn add_value_line_layer(&self, value: c_double) -> Option<ValueLineLayerBuilder>;
    fn set_value_range_min(&self, value: f64);
    fn set_value_range_max(&self, value: f64);
    fn set_show_value_range_min(&self, value: bool);
    fn set_show_value_range_max(&self, value: bool);
    fn add_value_tick(&self, value: c_double) -> Option<ValueTickBuilder>;
}

unsafe impl<T: PlotSummaryBuilderImpl> IsImplementable<T> for PlotSummaryBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.add_plot_layer = Some(plot_summary_builder_add_plot_layer::<T>);
        iface.add_value_line_layer = Some(plot_summary_builder_add_value_line_layer::<T>);
        iface.set_value_range_min = Some(plot_summary_builder_set_value_range_min::<T>);
        iface.set_value_range_max = Some(plot_summary_builder_set_value_range_max::<T>);
        iface.set_show_value_range_min = Some(plot_summary_builder_set_show_value_range_min::<T>);
        iface.set_show_value_range_max = Some(plot_summary_builder_set_show_value_range_max::<T>);
        iface.add_value_tick = Some(plot_summary_builder_add_value_tick::<T>);
    }
}

unsafe extern "C" fn plot_summary_builder_add_plot_layer<T: PlotSummaryBuilderImpl>(
    p_this: *mut ffi::OmniPlotSummaryBuilder,
    stream_handle: i32,
    time_field_name: *const c_char,
    plot_type: ffi::OmniPlotLayerType,
) -> *mut ffi::OmniPlotLayerBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp
        .add_plot_layer(
            stream_handle,
            &glib::GString::from_glib_borrow(time_field_name),
            from_glib(plot_type),
        )
        .into_glib_ptr()
}

unsafe extern "C" fn plot_summary_builder_add_value_line_layer<T: PlotSummaryBuilderImpl>(
    p_this: *mut ffi::OmniPlotSummaryBuilder,
    value: c_double,
) -> *mut ffi::OmniValueLineLayerBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.add_value_line_layer(value).into_glib_ptr()
}

unsafe extern "C" fn plot_summary_builder_set_value_range_min<T: PlotSummaryBuilderImpl>(
    p_this: *mut ffi::OmniPlotSummaryBuilder,
    value: c_double,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.set_value_range_min(value);
}

unsafe extern "C" fn plot_summary_builder_set_value_range_max<T: PlotSummaryBuilderImpl>(
    p_this: *mut ffi::OmniPlotSummaryBuilder,
    value: c_double,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.set_value_range_max(value);
}

unsafe extern "C" fn plot_summary_builder_set_show_value_range_min<T: PlotSummaryBuilderImpl>(
    p_this: *mut ffi::OmniPlotSummaryBuilder,
    value: glib::ffi::gboolean,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.set_show_value_range_min(from_glib(value));
}

unsafe extern "C" fn plot_summary_builder_set_show_value_range_max<T: PlotSummaryBuilderImpl>(
    p_this: *mut ffi::OmniPlotSummaryBuilder,
    value: glib::ffi::gboolean,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.set_show_value_range_max(from_glib(value));
}

unsafe extern "C" fn plot_summary_builder_add_value_tick<T: PlotSummaryBuilderImpl>(
    p_this: *mut ffi::OmniPlotSummaryBuilder,
    value: c_double,
) -> *mut ffi::OmniValueTickBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.add_value_tick(value).into_glib_ptr()
}
