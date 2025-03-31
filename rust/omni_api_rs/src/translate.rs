//! Supplements to [`glib::translate`]. Provides additional traits to
//! facilitate translation between GLib/GLib-based FFI types and their Rust
//! counterparts.

use glib::translate::{FromGlibPtrFull, IntoGlibPtr};

/// Translate an array of owned objects to a `GPtrArray`,
/// transferring ownership.
///
/// gir generates code that requires this in spite of the fact that it's not
/// provided by the `glib` crate.
///
/// ```text
/// // Vala
/// void return_batch(owned GenericArray<Omni.TransformSingleOutputBatch> batches);
/// ```
///
/// ```text
/// <!-- GIR, generated -->
/// <method name="return_batch" c:identifier="omni_transform_single_output_sink_return_batch">
///   <return-value transfer-ownership="full">
///     <type name="none" c:type="void"/>
///   </return-value>
///   <parameters>
///     <instance-parameter name="self" transfer-ownership="none">
///       <type name="Omni.TransformSingleOutputSink" c:type="OmniTransformSingleOutputSink*"/>
///     </instance-parameter>
///     <parameter name="batches" transfer-ownership="full">
///       <array name="GLib.PtrArray" c:type="GPtrArray*">
///         <type name="Omni.TransformSingleOutputBatch" c:type="OmniTransformSingleOutputBatch*"/>
///       </array>
///     </parameter>
///   </parameters>
/// </method>
/// ```
///
/// ```rust,ignore
/// // Rust, generated
/// #[doc(alias = "omni_transform_single_output_sink_return_batch")]
/// fn return_batch(&self, batches: Vec<TransformSingleOutputBatch>) {
///     unsafe {
///         ffi::omni_transform_single_output_sink_return_batch(
///             self.as_ref().to_glib_none().0, batches.into_glib_ptr());
///         //                                          ^^^^^^^^^^^^^
///     }
/// }
/// ```
///
/// # Example
///
/// ```rust
/// use glib::{ffi, GString};
/// use omni_api_rs::translate::IntoGPtrArray;
/// let objects: Vec<GString> = vec!["hello".into(); 10];
/// let objects: *mut ffi::GPtrArray = unsafe { objects.into_glib_ptr() };
/// unsafe { ffi::g_ptr_array_unref(objects) };
/// ```
pub trait IntoGPtrArray<P> {
    /// Translate an array of owned objects to a `GPtrArray`, transferring
    /// ownership.
    ///
    /// # Safety
    ///
    /// See [`IntoGlibPtr::into_glib_ptr`].
    // Actually `IntoGlibPtr::into_glib_ptr` is undocumented, though
    unsafe fn into_glib_ptr(self) -> *mut glib::ffi::GPtrArray;
}

impl<T: IntoGlibPtr<*mut Ffi> + FromGlibPtrFull<*mut Ffi>, Ffi: 'static> IntoGPtrArray<Ffi>
    for Vec<T>
{
    unsafe fn into_glib_ptr(self) -> *mut glib::ffi::GPtrArray {
        let arr = glib::ffi::g_ptr_array_sized_new(self.len() as _);

        unsafe extern "C" fn free<T: FromGlibPtrFull<*mut Ffi>, Ffi: 'static>(
            p: glib::ffi::gpointer,
        ) {
            T::from_glib_full(p.cast());
        }
        glib::ffi::g_ptr_array_set_free_func(arr, Some(free::<T, Ffi>));

        for ptr in self.into_iter() {
            glib::ffi::g_ptr_array_add(arr, ptr.into_glib_ptr().cast());
        }

        arr
    }
}
