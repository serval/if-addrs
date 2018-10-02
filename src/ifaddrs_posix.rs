// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

#[cfg(target_os = "android")]
use get_if_addrs_sys::{freeifaddrs, getifaddrs, ifaddrs};
#[cfg(not(target_os = "android"))]
use libc::{freeifaddrs, getifaddrs, ifaddrs};
use sockaddr;
use std::net::IpAddr;
use std::{io, mem};

#[cfg(
    any(
        target_os = "linux",
        target_os = "android",
        target_os = "nacl"
    )
)]
pub fn do_broadcast(ifaddr: &ifaddrs) -> Option<IpAddr> {
    sockaddr::to_ipaddr(ifaddr.ifa_ifu)
}

#[cfg(
    any(
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "openbsd"
    )
)]
pub fn do_broadcast(ifaddr: &ifaddrs) -> Option<IpAddr> {
    sockaddr::to_ipaddr(ifaddr.ifa_dstaddr)
}

pub struct IfAddrs {
    inner: *mut ifaddrs,
}

impl IfAddrs {
    #[allow(unsafe_code)]
    pub fn new() -> io::Result<Self> {
        let mut ifaddrs: *mut ifaddrs;

        unsafe {
            ifaddrs = mem::uninitialized();
            if -1 == getifaddrs(&mut ifaddrs) {
                return Err(io::Error::last_os_error());
            }
        }

        Ok(Self { inner: ifaddrs })
    }

    pub fn iter(&self) -> IfAddrsIterator {
        IfAddrsIterator { next: self.inner }
    }
}

impl Drop for IfAddrs {
    #[allow(unsafe_code)]
    fn drop(&mut self) {
        unsafe {
            freeifaddrs(self.inner);
        }
    }
}

pub struct IfAddrsIterator {
    next: *mut ifaddrs,
}

impl Iterator for IfAddrsIterator {
    type Item = ifaddrs;

    #[allow(unsafe_code)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
            return None;
        };

        Some(unsafe {
            let result = *self.next;
            self.next = (*self.next).ifa_next;

            result
        })
    }
}
