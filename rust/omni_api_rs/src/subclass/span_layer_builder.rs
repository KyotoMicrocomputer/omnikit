use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::{SpanLayerBuilder, SpanStyle};

pub trait SpanLayerBuilderImpl: ObjectImpl {
    fn set_label_field_name(&self, value: Option<&str>);
    fn set_color(&self, value: u32);
    fn set_color_field_name(&self, value: Option<&str>);
    fn set_cross_subtrack(&self, value: bool);
    fn set_style(&self, value: SpanStyle);
    fn set_enhance_short_spans(&self, value: bool);
    fn set_ignore_invalid_end_id(&self, value: bool);
}

unsafe impl<T: SpanLayerBuilderImpl> IsImplementable<T> for SpanLayerBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.set_label_field_name = Some(span_layer_set_label_field_name::<T>);
        iface.set_color = Some(span_layer_set_color::<T>);
        iface.set_color_field_name = Some(span_layer_set_color_field_name::<T>);
        iface.set_cross_subtrack = Some(span_layer_set_cross_subtrack::<T>);
        iface.set_style = Some(span_layer_set_style::<T>);
        iface.set_enhance_short_spans = Some(span_layer_set_enhance_short_spans::<T>);
        iface.set_ignore_invalid_end_id = Some(span_layer_set_ignore_invalid_end_id::<T>);
    }
}

unsafe extern "C" fn span_layer_set_label_field_name<T: SpanLayerBuilderImpl>(
    p_this: *mut ffi::OmniSpanLayerBuilder,
    value: *const c_char,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let value_borrowed;
    this_imp.set_label_field_name(if value.is_null() {
        None
    } else {
        value_borrowed = glib::GString::from_glib_borrow(value);
        Some(&value_borrowed)
    });
}

unsafe extern "C" fn span_layer_set_color<T: SpanLayerBuilderImpl>(
    p_this: *mut ffi::OmniSpanLayerBuilder,
    value: u32,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.set_color(value);
}

unsafe extern "C" fn span_layer_set_color_field_name<T: SpanLayerBuilderImpl>(
    p_this: *mut ffi::OmniSpanLayerBuilder,
    value: *const c_char,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    let value_borrowed;
    this_imp.set_color_field_name(if value.is_null() {
        None
    } else {
        value_borrowed = glib::GString::from_glib_borrow(value);
        Some(&value_borrowed)
    });
}

unsafe extern "C" fn span_layer_set_cross_subtrack<T: SpanLayerBuilderImpl>(
    p_this: *mut ffi::OmniSpanLayerBuilder,
    value: glib::ffi::gboolean,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.set_cross_subtrack(from_glib(value));
}

unsafe extern "C" fn span_layer_set_style<T: SpanLayerBuilderImpl>(
    p_this: *mut ffi::OmniSpanLayerBuilder,
    value: ffi::OmniSpanStyle,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.set_style(from_glib(value));
}

unsafe extern "C" fn span_layer_set_enhance_short_spans<T: SpanLayerBuilderImpl>(
    p_this: *mut ffi::OmniSpanLayerBuilder,
    value: glib::ffi::gboolean,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.set_enhance_short_spans(from_glib(value));
}

unsafe extern "C" fn span_layer_set_ignore_invalid_end_id<T: SpanLayerBuilderImpl>(
    p_this: *mut ffi::OmniSpanLayerBuilder,
    value: glib::ffi::gboolean,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.set_ignore_invalid_end_id(from_glib(value));
}
