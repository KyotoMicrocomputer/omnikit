use glib::{object::IsClass, prelude::*};

use crate::{prelude::*, PluginRegistry, Source, SourceInfo, Tool, ToolInfo};

pub trait PluginRegistryExtManual: IsA<PluginRegistry> + Sized {
    /// Register `T` by calling [`PluginRegistryExt::add_source`].
    ///
    /// This is safer than calling the method directly because the trait bounds
    /// ensure that `T` is compatible.
    fn add_source_of<T: IsA<Source> + IsClass>(&self) -> SourceInfo {
        self.add_source(T::static_type()).unwrap()
    }

    /// Register `T` by calling [`PluginRegistryExt::add_tool`].
    ///
    /// This is safer than calling the method directly because the trait bounds
    /// ensure that `T` is compatible.
    fn add_tool_of<T: IsA<Tool> + IsClass>(&self) -> ToolInfo {
        self.add_tool(T::static_type()).unwrap()
    }
}

impl<T: IsA<PluginRegistry>> PluginRegistryExtManual for T {}
