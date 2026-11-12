use glib::{subclass::prelude::*, translate::*};

use crate::{PluginRegistry, SourceInfo, ToolInfo};

pub trait PluginRegistryImpl: ObjectImpl {
    fn add_source(&self, ty: glib::Type) -> Option<SourceInfo>;
    fn add_tool(&self, ty: glib::Type) -> Option<ToolInfo>;
}

unsafe impl<T: PluginRegistryImpl> IsImplementable<T> for PluginRegistry {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.add_source = Some(plugin_registry_add_source::<T>);
        iface.add_tool = Some(plugin_registry_add_tool::<T>);
    }
}

unsafe extern "C" fn plugin_registry_add_source<T: PluginRegistryImpl>(
    p_this: *mut ffi::OmniPluginRegistry,
    ty: glib::ffi::GType,
) -> *mut ffi::OmniSourceInfo {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.add_source(from_glib(ty)).into_glib_ptr()
}

unsafe extern "C" fn plugin_registry_add_tool<T: PluginRegistryImpl>(
    p_this: *mut ffi::OmniPluginRegistry,
    ty: glib::ffi::GType,
) -> *mut ffi::OmniToolInfo {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();
    this_imp.add_tool(from_glib(ty)).into_glib_ptr()
}
