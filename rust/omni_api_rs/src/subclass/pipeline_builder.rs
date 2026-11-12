use std::ffi::c_char;

use glib::{subclass::prelude::*, translate::*};

use crate::{
    ArrowSchema, PipelineBuilder, PipelineFeatureFlags, ProjectOpBuilder, StreamBuilder,
    StreamDebugTrackBuilder, TrackBuilder, TransformOpBuilder, Transformer,
};

pub trait PipelineBuilderImpl: ObjectImpl {
    fn pipeline_feature_flags(&self) -> PipelineFeatureFlags;
    fn stream_by_handle(&self, stream_handle: i32) -> Option<StreamBuilder>;
    fn add_table_stream(&self, schema: &ArrowSchema) -> Option<StreamBuilder>;
    fn add_transform_op(&self, transformer: &Transformer) -> Option<TransformOpBuilder>;
    fn add_project_op(&self) -> Option<ProjectOpBuilder>;
    fn append_track(&self) -> Option<TrackBuilder>;
    fn add_stream_debug_track(
        &self,
        stream_handle: i32,
        time_field_name: &str,
    ) -> Option<StreamDebugTrackBuilder>;
}

unsafe impl<T: PipelineBuilderImpl> IsImplementable<T> for PipelineBuilder {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.get_pipeline_feature_flags = Some(pipeline_builder_get_pipeline_feature_flags::<T>);
        iface.dup_stream_by_handle = Some(pipeline_builder_dup_stream_by_handle::<T>);
        iface.add_table_stream = Some(pipeline_builder_add_table_stream::<T>);
        iface.add_transform_op = Some(pipeline_builder_add_transform_op::<T>);
        iface.add_project_op = Some(pipeline_builder_add_project_op::<T>);
        iface.append_track = Some(pipeline_builder_append_track::<T>);
        iface.add_stream_debug_track = Some(pipeline_builder_add_stream_debug_track::<T>);
    }
}

unsafe extern "C" fn pipeline_builder_get_pipeline_feature_flags<T: PipelineBuilderImpl>(
    p_this: *mut ffi::OmniPipelineBuilder,
) -> ffi::OmniPipelineFeatureFlags {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.pipeline_feature_flags().into_glib()
}

unsafe extern "C" fn pipeline_builder_dup_stream_by_handle<T: PipelineBuilderImpl>(
    p_this: *mut ffi::OmniPipelineBuilder,
    stream_handle: i32,
) -> *mut ffi::OmniStreamBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.stream_by_handle(stream_handle).into_glib_ptr()
}

unsafe extern "C" fn pipeline_builder_add_table_stream<T: PipelineBuilderImpl>(
    p_this: *mut ffi::OmniPipelineBuilder,
    schema: *mut ffi::OmniArrowSchema,
) -> *mut ffi::OmniStreamBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp
        .add_table_stream(&from_glib_none(schema))
        .into_glib_ptr()
}

unsafe extern "C" fn pipeline_builder_add_transform_op<T: PipelineBuilderImpl>(
    p_this: *mut ffi::OmniPipelineBuilder,
    transformer: *mut ffi::OmniTransformer,
) -> *mut ffi::OmniTransformOpBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp
        .add_transform_op(&from_glib_none(transformer))
        .into_glib_ptr()
}

unsafe extern "C" fn pipeline_builder_add_project_op<T: PipelineBuilderImpl>(
    p_this: *mut ffi::OmniPipelineBuilder,
) -> *mut ffi::OmniProjectOpBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.add_project_op().into_glib_ptr()
}

unsafe extern "C" fn pipeline_builder_append_track<T: PipelineBuilderImpl>(
    p_this: *mut ffi::OmniPipelineBuilder,
) -> *mut ffi::OmniTrackBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.append_track().into_glib_ptr()
}

unsafe extern "C" fn pipeline_builder_add_stream_debug_track<T: PipelineBuilderImpl>(
    p_this: *mut ffi::OmniPipelineBuilder,
    stream_handle: i32,
    time_field_name: *const c_char,
) -> *mut ffi::OmniStreamDebugTrackBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp
        .add_stream_debug_track(
            stream_handle,
            &glib::GString::from_glib_borrow(time_field_name),
        )
        .into_glib_ptr()
}
