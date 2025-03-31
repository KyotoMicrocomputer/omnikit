use glib::prelude::*;

use crate::{LinkArray, LinkPolicy, LinkPolicyFlags};

pub trait LinkPolicyExtManual: IsA<LinkPolicy> + Sized {
    /// Call [`LinkPolicy::set_flags`] and return `self`.
    fn with_flags(self, value: LinkPolicyFlags) -> Self;

    /// Add a port attribute criterion and return `self` to allow for method
    /// chaining.
    ///
    /// # Examples
    ///
    /// See [`ParamSpecBuilderLinkPolicyExt`].
    #[must_use = "consider using `insert_criteria` method if the intent is to modify a \
                  `LinkPolicy` without consuming it"]
    fn with_criteria(self, key: impl Into<glib::GString>, value: impl Into<glib::Variant>) -> Self;
}

impl LinkPolicyExtManual for LinkPolicy {
    #[inline]
    fn with_flags(self, value: LinkPolicyFlags) -> Self {
        self.set_flags(value);
        self
    }

    #[inline]
    fn with_criteria(self, key: impl Into<glib::GString>, value: impl Into<glib::Variant>) -> Self {
        self.insert_criteria(&key.into(), &value.into());
        self
    }
}

/// An extension trait for
/// [`glib::ParamSpecObjectBuilder`]`<`[`LinkArray`]`>`. Provides a
/// method to attach a [`LinkPolicy`] to a built [`glib::ParamSpec`].
///
/// # Examples
///
/// ```rust,no_run
/// use omni_api_rs::{prelude::*, LinkArray, LinkPolicy, LinkPolicyFlags};
/// let _param = glib::ParamSpecObject::builder("input")
///     .link_policy(|b| {
///         b.with_flags(LinkPolicyFlags::MULTIPLE)
///             .with_criteria("provides-byte-stream", true)
///     })
///     .build();
/// ```
pub trait ParamSpecBuilderLinkPolicyExt<'a>: Sized {
    /// Convert `self` into [`ParamSpecLinkArrayBuilder`] to associate
    /// the built [`glib::ParamSpec`] with a provided [`LinkPolicy`].
    fn link_policy(self, b: impl FnOnce(LinkPolicy) -> LinkPolicy)
        -> ParamSpecLinkArrayBuilder<'a>;
}

impl<'a> ParamSpecBuilderLinkPolicyExt<'a> for glib::ParamSpecObjectBuilder<'a, LinkArray> {
    #[inline]
    fn link_policy(
        self,
        b: impl FnOnce(LinkPolicy) -> LinkPolicy,
    ) -> ParamSpecLinkArrayBuilder<'a> {
        ParamSpecLinkArrayBuilder {
            builder: self,
            bounds: b(LinkPolicy::new()),
        }
    }
}

/// Wraps [`glib::ParamSpecObjectBuilder`] to call
/// [`LinkPolicy::set_to_param`] during `build()`.
pub struct ParamSpecLinkArrayBuilder<'a> {
    builder: glib::ParamSpecObjectBuilder<'a, LinkArray>,
    bounds: LinkPolicy,
}

impl<'a> ParamSpecBuilderExt<'a> for ParamSpecLinkArrayBuilder<'a> {
    fn set_nick(&mut self, nick: Option<&'a str>) {
        self.builder.set_nick(nick);
    }

    fn set_blurb(&mut self, blurb: Option<&'a str>) {
        self.builder.set_blurb(blurb);
    }

    fn set_flags(&mut self, flags: glib::ParamFlags) {
        self.builder.set_flags(flags);
    }

    fn current_flags(&self) -> glib::ParamFlags {
        self.builder.current_flags()
    }
}

impl ParamSpecLinkArrayBuilder<'_> {
    /// Build a [`glib::ParamSpec`].
    pub fn build(self) -> glib::ParamSpec {
        let spec = self.builder.build();
        LinkPolicy::set_to_param(&self.bounds, &spec);
        spec
    }
}
