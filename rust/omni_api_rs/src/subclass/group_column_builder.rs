use glib::subclass::prelude::*;

use crate::GroupColumnBuilder;

pub trait GroupColumnBuilderImpl: ObjectImpl {}

unsafe impl<T: GroupColumnBuilderImpl> IsImplementable<T> for GroupColumnBuilder {}
