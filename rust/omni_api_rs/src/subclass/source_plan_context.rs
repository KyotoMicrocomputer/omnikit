use glib::{subclass::prelude::*, translate::*};

use crate::{ArrowSchema, SourcePlanContext, StreamBuilder};

pub trait SourcePlanContextImpl: ObjectImpl + Send + Sync {
    fn is_dry_run(&self) -> bool;
    fn add_table_stream(&self, schema: &ArrowSchema) -> Option<StreamBuilder>;
}

unsafe impl<T: SourcePlanContextImpl> IsImplementable<T> for SourcePlanContext {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.get_dry_run = Some(source_plan_context_get_dry_run::<T>);
        iface.add_table_stream = Some(source_plan_context_add_table_stream::<T>);
    }
}

unsafe extern "C" fn source_plan_context_get_dry_run<T: SourcePlanContextImpl>(
    p_this: *mut ffi::OmniSourcePlanContext,
) -> glib::ffi::gboolean {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.is_dry_run().into_glib()
}

unsafe extern "C" fn source_plan_context_add_table_stream<T: SourcePlanContextImpl>(
    p_this: *mut ffi::OmniSourcePlanContext,
    schema: *mut ffi::OmniArrowSchema,
) -> *mut ffi::OmniStreamBuilder {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp
        .add_table_stream(&from_glib_none(schema))
        .into_glib_ptr()
}
