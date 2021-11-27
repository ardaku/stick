compile_error!(include_str!(concat!(env!("OUT_DIR"), "/unsupported.rs")));

pub(super) type Device<T> = core::marker::PhantomData<T>;

pub(super) fn start() -> Device<Device<crate::packet::Midi>> {
    core::marker::PhantomData
}
