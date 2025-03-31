use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::{PlotLayerBuilder, PlotSeriesBuilder};

pub trait PlotLayerBuilderImpl: ObjectImpl {
    fn add_series(&self, value_field_name: &str) -> Option<PlotSeriesBuilder>;
}

unsafe impl<T: PlotLayerBuilderImpl> IsImplementable<T> for PlotLayerBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.add_series = Some(plot_layer_builder_add_series::<T>);
    }
}

unsafe extern "C" fn plot_layer_builder_add_series<T: PlotLayerBuilderImpl>(
    p_this: *mut ffi::OmniPlotLayerBuilder,
    value_field_name: *const c_char,
) -> *mut ffi::OmniPlotSeriesBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp
        .add_series(&glib::GString::from_glib_borrow(value_field_name))
        .into_glib_ptr()
}
