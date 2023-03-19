// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::pin::Pin;

use autocxx::cxx::private::{UniquePtrTarget, SharedPtrTarget};

pub mod impls;
pub mod external_types;

pub unsafe fn cxx_const_cast<T: UniquePtrTarget>(value: &T) -> Pin<&mut T> {
    #![inline]
    //! Presents an immutable reference as a mutable one for the purpose of calling a CXX bridge
    //! function (casts the constness away). The mutable reference must not actually be mutated!
    //! (Otherwise, bring mutability into the Rust code.)
    //!
    //! This is meant as a last resort to avoid having to write a C++ wrapper function every
    //! time some API function isn't declared as `const` on the C++ side, even though it should
    //! be. In that wrapper, the same thing would be done with a C++ `const_cast<...>(...)`
    //! anyway.

    #[allow(clippy::cast_ref_to_mut)]
    Pin::new_unchecked(&mut *(value as *const T as *mut T))
}

pub unsafe fn cxx_shared_cast<T: SharedPtrTarget>(value: &T) -> Pin<&mut T> {
    #![inline]
    //! Presents an immutable reference as a mutable one for the purpose of calling a CXX bridge
    //! function (casts the constness away). The mutable reference must not actually be mutated!
    //! (Otherwise, bring mutability into the Rust code.)
    //!
    //! This is meant as a last resort to avoid having to write a C++ wrapper function every
    //! time some API function isn't declared as `const` on the C++ side, even though it should
    //! be. In that wrapper, the same thing would be done with a C++ `const_cast<...>(...)`
    //! anyway.

    #[allow(clippy::cast_ref_to_mut)]
    Pin::new_unchecked(&mut *(value as *const T as *mut T))
}

#[macro_export]
macro_rules! pin_mut {
    ($caller:tt) => {
        {
            let p = unsafe { cxx_shared_cast($caller.as_ref().unwrap()) };
            p
        }
    }
}