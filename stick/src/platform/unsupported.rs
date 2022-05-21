// Copyright © 2017-2022 The Stick Contributors.
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// - MIT License (https://mit-license.org/)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.

compile_error!(include_str!(concat!(env!("OUT_DIR"), "/unsupported.rs")));

pub(super) type Device<T> = core::marker::PhantomData<T>;

pub(super) fn start() -> Device<Device<crate::packet::Midi>> {
    core::marker::PhantomData
}
