use glib::subclass::prelude::*;

use crate::{RecordClock, Time};

pub trait RecordClockImpl: ObjectImpl {
    fn now(&self) -> Time;
}

unsafe impl<T: RecordClockImpl> IsImplementable<T> for RecordClock {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.now = Some(record_clock_now::<T>);
    }
}

unsafe extern "C" fn record_clock_now<T: RecordClockImpl>(
    p_this: *mut ffi::OmniRecordClock,
) -> ffi::OmniTime {
    let this_instance = &*(p_this as *mut T::Instance);
    let this_imp = this_instance.imp();

    this_imp.now()
}
