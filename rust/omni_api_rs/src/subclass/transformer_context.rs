use std::ffi::c_uint;

use glib::{subclass::prelude::*, translate::*};

use crate::{PhysStreamId, PipelineLayout, TransformerContext};

pub trait TransformerContextImpl: ObjectImpl + Send + Sync {
    fn input_stream(&self, input_stream_i: c_uint) -> Option<PhysStreamId>;
    fn pipeline_layout(&self) -> Option<PipelineLayout>;
}

unsafe impl<T: TransformerContextImpl> IsImplementable<T> for TransformerContext {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.input_stream = Some(transformer_context_input_stream::<T>);
        iface.pipeline_layout = Some(transformer_context_pipeline_layout::<T>);
    }
}

unsafe extern "C" fn transformer_context_input_stream<T: TransformerContextImpl>(
    p_this: *mut ffi::OmniTransformerContext,
    input_stream_i: c_uint,
) -> *mut ffi::OmniPhysStreamId {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.input_stream(input_stream_i).into_glib_ptr()
}

unsafe extern "C" fn transformer_context_pipeline_layout<T: TransformerContextImpl>(
    p_this: *mut ffi::OmniTransformerContext,
) -> *mut ffi::OmniPipelineLayout {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.pipeline_layout().into_glib_ptr()
}
