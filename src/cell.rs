// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use cmp::Ordering;
use fmt::{self, Debug, Display};

#[cfg(unstable)]
use marker::Unsize;

#[cfg(unstable)]
use ops::CoerceUnsized;

use ops::{Deref, DerefMut};
use std::cell::{UnsafeCell};

#[derive(Debug)]
pub struct RefCell<T: ?Sized> {
    value: UnsafeCell<T>,
}

pub struct BorrowError {
    _private: (),
}

impl Debug for BorrowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BorrowError").finish()
    }
}

impl Display for BorrowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt("already mutably borrowed", f)
    }
}

pub struct BorrowMutError {
    _private: (),
}


impl Debug for BorrowMutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BorrowMutError").finish()
    }
}

impl Display for BorrowMutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt("already borrowed", f)
    }
}

impl<T> RefCell<T> {
    #[inline]
    pub fn new(value: T) -> RefCell<T> {
        RefCell {
            value: UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn into_inner(self) -> T {
        unsafe { self.value.into_inner() }
    }
}

impl<T: ?Sized> RefCell<T> {
    #[inline]
    pub fn borrow(&self) -> Ref<T> {
        Ref {
            value: unsafe { &*self.value.get() },
        }
    }

    #[inline]
    pub fn borrow_mut(&self) -> RefMut<T> {
        RefMut {
            value: unsafe { &mut *self.value.get() },
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut T {
        self.value.get()
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        unsafe {
            &mut *self.value.get()
        }
    }
}

unsafe impl<T: ?Sized> Send for RefCell<T> where T: Send {}

#[cfg(unstable)]
impl<T: ?Sized> !Sync for RefCell<T> {}

impl<T: Clone> Clone for RefCell<T> {
    #[inline]
    fn clone(&self) -> RefCell<T> {
        RefCell::new(self.borrow().clone())
    }
}

impl<T:Default> Default for RefCell<T> {
    /// Creates a `RefCell<T>`, with the `Default` value for T.
    #[inline]
    fn default() -> RefCell<T> {
        RefCell::new(Default::default())
    }
}

impl<T: ?Sized + PartialEq> PartialEq for RefCell<T> {
    #[inline]
    fn eq(&self, other: &RefCell<T>) -> bool {
        *self.borrow() == *other.borrow()
    }
}

impl<T: ?Sized + Eq> Eq for RefCell<T> {}

impl<T: ?Sized + PartialOrd> PartialOrd for RefCell<T> {
    #[inline]
    fn partial_cmp(&self, other: &RefCell<T>) -> Option<Ordering> {
        self.borrow().partial_cmp(&*other.borrow())
    }

    #[inline]
    fn lt(&self, other: &RefCell<T>) -> bool {
        *self.borrow() < *other.borrow()
    }

    #[inline]
    fn le(&self, other: &RefCell<T>) -> bool {
        *self.borrow() <= *other.borrow()
    }

    #[inline]
    fn gt(&self, other: &RefCell<T>) -> bool {
        *self.borrow() > *other.borrow()
    }

    #[inline]
    fn ge(&self, other: &RefCell<T>) -> bool {
        *self.borrow() >= *other.borrow()
    }
}


impl<T: ?Sized + Ord> Ord for RefCell<T> {
    #[inline]
    fn cmp(&self, other: &RefCell<T>) -> Ordering {
        self.borrow().cmp(&*other.borrow())
    }
}

impl<T> From<T> for RefCell<T> {
    fn from(t: T) -> RefCell<T> {
        RefCell::new(t)
    }
}

#[cfg(unstable)]
impl<T: CoerceUnsized<U>, U> CoerceUnsized<RefCell<U>> for RefCell<T> {}

#[derive(Debug)]
pub struct Ref<'b, T: ?Sized + 'b> {
    value: &'b T,
}

impl<'b, T: ?Sized> Deref for Ref<'b, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.value
    }
}

impl<'b, T: ?Sized> Ref<'b, T> {
    #[inline]
    pub fn clone(orig: &Ref<'b, T>) -> Ref<'b, T> {
        Ref {
            value: orig.value,
        }
    }

    #[cfg(unstable)]
    #[inline]
    pub fn map<U: ?Sized, F>(orig: Ref<'b, T>, f: F) -> Ref<'b, U>
        where F: FnOnce(&T) -> &U
    {
        Ref {
            value: f(orig.value),
        }
    }
}

#[cfg(unstable)]
impl<'b, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Ref<'b, U>> for Ref<'b, T> {}

#[cfg(unstable)]
impl<'b, T: ?Sized> RefMut<'b, T> {
    #[inline]
    pub fn map<U: ?Sized, F>(orig: RefMut<'b, T>, f: F) -> RefMut<'b, U>
        where F: FnOnce(&mut T) -> &mut U
    {
        RefMut {
            value: f(orig.value),
        }
    }
}

#[derive(Debug)]
pub struct RefMut<'b, T: ?Sized + 'b> {
    value: &'b mut T,
}

impl<'b, T: ?Sized> Deref for RefMut<'b, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.value
    }
}

impl<'b, T: ?Sized> DerefMut for RefMut<'b, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}

#[cfg(unstable)]
impl<'b, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<RefMut<'b, U>> for RefMut<'b, T> {}

