use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::{RichSummaryBuilder, SpanLayerBuilder};

pub trait RichSummaryBuilderImpl: ObjectImpl {
    fn append_span_layer(
        &self,
        start_stream_handle: i32,
        end_stream_handle: i32,
        time_field_name: &str,
        id_field_name: &str,
    ) -> Option<SpanLayerBuilder>;
}

unsafe impl<T: RichSummaryBuilderImpl> IsImplementable<T> for RichSummaryBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.append_span_layer = Some(rich_summary_builder_append_span_layer::<T>);
    }
}

unsafe extern "C" fn rich_summary_builder_append_span_layer<T: RichSummaryBuilderImpl>(
    p_this: *mut ffi::OmniRichSummaryBuilder,
    start_stream_handle: i32,
    end_stream_handle: i32,
    time_field_name: *const c_char,
    id_field_name: *const c_char,
) -> *mut ffi::OmniSpanLayerBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp
        .append_span_layer(
            start_stream_handle,
            end_stream_handle,
            &glib::GString::from_glib_borrow(time_field_name),
            &glib::GString::from_glib_borrow(id_field_name),
        )
        .into_glib_ptr()
}
