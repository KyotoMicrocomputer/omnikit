use glib::subclass::prelude::*;

use crate::AggregationStreamBuilder;

pub trait AggregationStreamBuilderImpl: ObjectImpl {}

unsafe impl<T: AggregationStreamBuilderImpl> IsImplementable<T> for AggregationStreamBuilder {}
