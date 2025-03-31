use glib::subclass::prelude::*;

use crate::PhysStreamId;

pub trait PhysStreamIdImpl: ObjectImpl + Send + Sync {}

unsafe impl<T: PhysStreamIdImpl> IsImplementable<T> for PhysStreamId {}
