// Copyright 2019 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! Implementation for ESP-IDF (ESP32, ESP8266)

use crate::{Error};
pub fn getrandom_inner(dest: &mut [u8]) -> Result<(), Error> {
    extern "C" {
        fn esp_fill_random(buf: *mut libc::c_void, len: libc::size_t) -> libc::c_void;
    }

    unsafe {
        esp_fill_random(dest.as_mut_ptr() as *mut libc::c_void, dest.len());
    }

    Ok(()) // TODO: Return false if ESP32's WiFi or Bluetooth is not initialized
}
