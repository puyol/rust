// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Verify that UnsafeCell is *always* sync regardless if `T` is sync.

// ignore-tidy-linelength

use std::cell::UnsafeCell;
use std::marker;

struct MySync<T> {
    u: UnsafeCell<T>
}

struct NoSync {
    m: marker::NoSync
}

fn test<T: Sync>(s: T){

}

fn main() {
    let us = UnsafeCell::new(MySync{u: UnsafeCell::new(0is)});
    test(us);
    //~^ ERROR `core::marker::Sync` is not implemented

    let uns = UnsafeCell::new(NoSync{m: marker::NoSync});
    test(uns);
    //~^ ERROR `core::marker::Sync` is not implemented

    let ms = MySync{u: uns};
    test(ms);
    //~^ ERROR `core::marker::Sync` is not implemented

    let ns = NoSync{m: marker::NoSync};
    test(ns);
    //~^ ERROR `core::marker::Sync` is not implemented
}
