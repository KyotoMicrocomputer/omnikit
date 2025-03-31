use std::ffi::{c_char, c_int};

use glib::{prelude::*, subclass::prelude::*, translate::*};

use crate::{PipelineBuilder, Tool};

/// The subclassing trait for [`Tool`].
pub trait ToolImpl: ObjectImpl {
    fn list_active_port_properties(&self) -> Result<glib::StrV, glib::Error> {
        self.parent_list_active_port_properties()
    }
    fn modify_links(&self) -> Result<(), glib::Error> {
        self.parent_modify_links()
    }
    fn add_to_pipeline(&self, builder: &PipelineBuilder) -> Result<(), glib::Error>;
}

pub trait ToolImplExt: ToolImpl + ObjectSubclass {
    fn parent_list_active_port_properties(&self) -> Result<glib::StrV, glib::Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Tool>() as *const ffi::OmniToolIface;
            let func = (*parent_iface)
                .list_active_port_properties
                .expect("no parent \"list_active_port_properties\" implementation");
            let mut error = std::ptr::null_mut();
            let mut ret_len = std::mem::MaybeUninit::uninit();
            let ret = func(
                self.obj().unsafe_cast_ref::<Tool>().to_glib_none().0,
                ret_len.as_mut_ptr(),
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibContainer::from_glib_full_num(
                    ret,
                    ret_len.assume_init() as _,
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_modify_links(&self) -> Result<(), glib::Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface =
                type_data.as_ref().parent_interface::<Tool>() as *const ffi::OmniToolIface;
            let modify_links = (*parent_iface)
                .modify_links
                .expect("no parent \"modify_links\" implementation");
            let mut error = std::ptr::null_mut();
            modify_links(
                self.obj().unsafe_cast_ref::<Tool>().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl<T: ToolImpl> ToolImplExt for T {}

unsafe impl<T: ToolImpl> IsImplementable<T> for Tool {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.list_active_port_properties = Some(tool_list_active_port_properties::<T>);
        iface.modify_links = Some(tool_modify_links::<T>);
        iface.add_to_pipeline = Some(tool_add_to_pipeline::<T>);
    }
}

unsafe extern "C" fn tool_list_active_port_properties<T: ToolImpl>(
    p_this: *mut ffi::OmniTool,
    p_result_length: *mut c_int,
    err: *mut *mut glib::ffi::GError,
) -> *mut *mut c_char {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    match this_imp.list_active_port_properties() {
        Ok(result) => {
            if !p_result_length.is_null() {
                *p_result_length = result.len().try_into().unwrap();
            }
            result.into_raw()
        }
        Err(e) => {
            if !err.is_null() {
                *err = e.into_glib_ptr();
            }
            std::ptr::null_mut()
        }
    }
}

unsafe extern "C" fn tool_modify_links<T: ToolImpl>(
    p_this: *mut ffi::OmniTool,
    err: *mut *mut glib::ffi::GError,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    match this_imp.modify_links() {
        Ok(()) => {}
        Err(e) => {
            if !err.is_null() {
                *err = e.into_glib_ptr();
            }
        }
    }
}

unsafe extern "C" fn tool_add_to_pipeline<T: ToolImpl>(
    p_this: *mut ffi::OmniTool,
    builder: *mut ffi::OmniPipelineBuilder,
    err: *mut *mut glib::ffi::GError,
) {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    match this_imp.add_to_pipeline(&from_glib_none(builder)) {
        Ok(()) => {}
        Err(e) => {
            if !err.is_null() {
                *err = e.into_glib_ptr();
            }
        }
    }
}
