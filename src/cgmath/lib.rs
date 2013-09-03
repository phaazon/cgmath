// Copyright 2013 The CGMath Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[link(name = "cgmath",
       vers = "0.1",
       author = "Brendan Zabarauskas",
       url = "https://github.com/bjz/cgmath-rs")];

#[comment = "A mathematics library for computer graphics."];
#[license = "ASL2"];
#[crate_type = "lib"];

pub mod array;
pub mod matrix;
pub mod point;
pub mod quaternion;
pub mod vector;

pub mod ray;

pub mod projection;

pub mod util {
    use std::num::one;

    // These functions are horrific! We really need better from-int support
    // in std::num.

    #[inline]
    pub fn two<T: Num>() -> T { one::<T>() + one::<T>() }
    #[inline]
    pub fn half<T: Real>() -> T { one::<T>() / two::<T>() }
}