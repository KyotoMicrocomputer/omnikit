//! A sample plugin for Prism Profiler written in Rust

#![expect(unsafe_code, reason = "defines a plugin entry point")]
use std::cell::Cell;

use glib::{
    translate::{IntoGlibPtr as _, from_glib_none},
    variant::Handle,
};
use omni_api_rs::{
    LinkArray, LinkPolicyFlags, PluginRegistry, Tool, prelude::*, subclass::prelude::*,
};

#[unsafe(no_mangle)]
unsafe extern "C" fn omni_plugin_load(
    plugin_registry: *mut omni_api_rs::ffi::OmniPluginRegistry,
    err: *mut *mut glib::ffi::GError,
) {
    unsafe {
        match load(&from_glib_none(plugin_registry)) {
            Ok(()) => {}
            Err(e) => {
                if !err.is_null() {
                    *err = e.into_glib_ptr();
                }
            }
        }
    }
}

fn load(plugin_registry: &PluginRegistry) -> Result<(), glib::Error> {
    let source = plugin_registry.add_tool_of::<TestTool>();
    source.set_name("Plugin Test Tool");

    Ok(())
}

#[derive(Debug, Clone, Copy, glib::ErrorDomain)]
#[error_domain(name = "sample-plugin-error-quark")]
enum Error {
    Failed,
}

glib::wrapper! {
    /// A minimal class implementing [`Tool`].
    struct TestTool(ObjectSubclass<imp::TestTool>)
        @implements Tool;
}

mod imp {
    use super::*;

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::TestTool)]
    pub(super) struct TestTool {
        #[property(
            set,
            type = LinkArray,
            link_policy = |b| b
                .with_criteria("provides-byte-stream", true)
                .with_flags(LinkPolicyFlags::DEFAULT_QUANT_EACH),
        )]
        pub input: Cell<Option<LinkArray>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TestTool {
        const NAME: &'static str = "OmniPluginTestTool";
        type Type = super::TestTool;
        type Interfaces = (Tool,);
    }
}

#[glib::derived_properties]
impl ObjectImpl for imp::TestTool {}

impl ToolImpl for imp::TestTool {
    fn add_to_pipeline(&self, builder: &omni_api_rs::PipelineBuilder) -> Result<(), glib::Error> {
        // Get the input byte stream
        let input = self.input.take().expect("`input` is null").link(0).unwrap();
        let input_outs: glib::VariantDict = input.output_set().unwrap().into();
        let Ok(Some(Handle(input_byte_stream))) = input_outs.lookup("byte-stream") else {
            return Err(glib::Error::new(
                Error::Failed,
                "link output 'byte-stream' is missing or has incorrect type",
            ));
        };

        // Define a track
        let track = builder.append_track().unwrap();
        track.add_simple_summary(input_byte_stream, "record_time");

        Ok(())
    }
}
