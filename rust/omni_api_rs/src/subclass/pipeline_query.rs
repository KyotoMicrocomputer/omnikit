use glib::{subclass::prelude::*, translate::*};

use crate::{ArrowSchema, PipelineQuery};

pub trait PipelineQueryImpl: ObjectImpl + Send + Sync {
    fn schema(&self) -> Option<ArrowSchema>;
}

unsafe impl<T: PipelineQueryImpl> IsImplementable<T> for PipelineQuery {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        iface.as_mut().get_schema = Some(pipeline_query_get_schema::<T>);
    }
}

unsafe extern "C" fn pipeline_query_get_schema<T: PipelineQueryImpl>(
    p_this: *mut ffi::OmniPipelineQuery,
) -> *mut ffi::OmniArrowSchema {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    stash_ptr!(
        p_this,
        "omni-api-rs-subclass-pipeline-query-get-schema",
        this_imp.schema().into_glib_ptr(),
        |ptr| if !ptr.is_null() {
            glib::gobject_ffi::g_object_unref(ptr.cast())
        },
    )
}
