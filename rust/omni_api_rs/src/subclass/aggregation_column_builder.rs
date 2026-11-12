use glib::subclass::prelude::*;

use crate::AggregationColumnBuilder;

pub trait AggregationColumnBuilderImpl: ObjectImpl {}

unsafe impl<T: AggregationColumnBuilderImpl> IsImplementable<T> for AggregationColumnBuilder {}
