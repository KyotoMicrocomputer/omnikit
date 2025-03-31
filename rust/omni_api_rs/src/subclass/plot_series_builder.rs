use std::ffi::c_double;

use glib::subclass::prelude::*;

use crate::PlotSeriesBuilder;

pub trait PlotSeriesBuilderImpl: ObjectImpl {
    fn append_value_color_stop(&self, value: f64, color: u32);
}

unsafe impl<T: PlotSeriesBuilderImpl> IsImplementable<T> for PlotSeriesBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.append_value_color_stop = Some(plot_series_builder_append_value_color_stop::<T>);
    }
}

unsafe extern "C" fn plot_series_builder_append_value_color_stop<T: PlotSeriesBuilderImpl>(
    p_this: *mut ffi::OmniPlotSeriesBuilder,
    value: c_double,
    color: u32,
) {
    let this = &*(p_this as *mut T::Instance);
    let this_imp = this.imp();

    this_imp.append_value_color_stop(value, color);
}
