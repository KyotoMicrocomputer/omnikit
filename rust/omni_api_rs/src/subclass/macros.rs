//! The common code for the default implementation of `*_async` methods.
//! `[tagref redacted]`
use glib::translate::IntoGlib as _;

macro_rules! default_async {
    (
        $callback:ident,
        move || $self:ident.$method:ident( $($tt:tt)* ) $(,)?
    ) => {
        let this = $self.ref_counted();
        let call_fut = async move {
            this.$method($($tt)*).await
        };
        glib::spawn_future_local(
            tracing::Instrument::in_current_span(
                async move {
                    // Using `AssertUnwindSafe` here is okay because we
                    // abort the process after catching panic
                    let call_fut = std::panic::AssertUnwindSafe(call_fut);
                    let Ok(result) = futures::FutureExt::catch_unwind(call_fut).await else {
                        tracing::error!(
                            "Async method '{}' panicked; aborting the process",
                            stringify!($method),
                        );
                        std::process::abort();
                    };
                    $callback(result);
                },
            )
        );
    };
}

/// Stash `$ptr` in a `GObject` quark slot. Intended to be used to implement
/// `none` transfer for a return value.
///
/// The pointer will remain valid until the macro-expanded code is evaluated
/// again with a different `$ptr` or the object referenced by `$self` is
/// destroyed.
/// The pointer is released by the closure or function specified by
/// `$destroy_ptr`.
macro_rules! stash_ptr {
    (
        $self:expr,
        $quark_str:expr,
        $ptr:expr,
        $destroy_ptr:expr $(,)?
    ) => {{
        static QUARK: std::sync::OnceLock<glib::Quark> = std::sync::OnceLock::new();
        let quark = *QUARK.get_or_init(|| glib::Quark::from_str($quark_str));
        let ptr = $ptr;
        crate::subclass::macros::stash_ptr_inner($self.cast(), quark, ptr, $destroy_ptr);
        ptr
    }};
}

/// An inner function of [`stash_ptr!`].
#[inline]
pub(super) unsafe fn stash_ptr_inner<P, Destroy>(
    this: *mut glib::gobject_ffi::GObject,
    quark: glib::Quark,
    ptr: *mut P,
    _destroy: Destroy,
) where
    Destroy: FnOnce(*mut P) + Send + Sync + Copy + 'static,
{
    unsafe extern "C" fn destroy_trampoline<P, Destroy>(ptr: glib::ffi::gpointer)
    where
        Destroy: FnOnce(*mut P),
    {
        const { assert!(size_of::<Destroy>() == 0) };
        let destroy: Destroy = std::mem::transmute_copy(&());
        destroy(ptr as *mut P);
    }

    glib::gobject_ffi::g_object_set_qdata_full(
        this,
        quark.into_glib(),
        ptr as _,
        Some(destroy_trampoline::<P, Destroy>),
    );
}
