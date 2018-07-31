//! Contains box types that may or may not own the data they point to.
//!
//! This allows for more adaptive ownership behavior at the cost requiring aditional
//! overhead to kept track of whether or not the value is owned. These types obey all
//! of rusts ownership rules.

use std::boxed::Box;
use std::fmt;
use std::marker::PhantomData;

/// This acts as a box type that may or may not own the data it points to.
/// The destructor for this type will check if the value is owned or not,
/// and will decide whether to drop it. 
pub struct MightOwn<'a, T: ?Sized + 'a> {
    ptr: *mut T,
    owned: bool,
    phantom: PhantomData<&'a T>,
}

impl<'a, T: ?Sized + 'a> Drop for MightOwn<'a, T> {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                // drops the pointer and the heap allocated memory allocated to it
                Box::from_raw(self.ptr);
            }
        }
        // if the value is not owned, then the drop will be managed by is owner
    }
}

impl<'a, T: ?Sized> MightOwn<'a, T> {
    /// Create an owned mightown value.
    ///
    /// ## Note:
    /// This allows for the lifetime to be chosen pretty much arbitrarily
    /// Since the lifetime does not matter for an owned value
    pub fn owned(val: Box<T>) -> MightOwn<'a, T> {
        MightOwn {
            ptr: Box::into_raw(val),
            owned: true,
            phantom: PhantomData,
        }
    }

    /// Create an unowned mightown value.
    pub fn unowned(val: &'a mut T) -> MightOwn<'a, T> {
        MightOwn {
            ptr: val as *mut T,
            owned: false,
            phantom: PhantomData,
        }
    }

    /// Get a mutable reference to the data pointed to by this value.
    pub fn get_mut(&mut self) -> &'a mut T {
        unsafe { &mut *self.ptr }
    }

    /// Get a nonmutable reference to the data pointed to by this value.
    pub fn get_const(&self) -> &'a T {
        unsafe { &*self.ptr }
    }

    /// Get a box wrapping the value stored in the mightown.
    /// If the value is not owned, then this will fail.
    pub fn get_owned(self) -> Result<Box<T>, NotOwnedError<'a, T>> {
        if self.owned {
            unsafe { Ok(Box::from_raw(self.ptr)) }
        } else {
            Err(NotOwnedError { val: self })
        }
    }
}

/// An error type for MightOwn. This contains the object
/// so it can be used after a failure. 
pub struct NotOwnedError<'a, T: ?Sized + 'a> {
    val: MightOwn<'a, T>,
}

impl<'a, T: ?Sized + 'a> NotOwnedError<'a, T> {
    pub fn get(self) -> MightOwn<'a, T> {
        self.val
    }
}

impl<'a, T: ?Sized + 'a> fmt::Debug for NotOwnedError<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Attempted to call get_owned on a MightOwn that does not contain owned data."
        )
    }
}

// TODO: Implement traits for MightOwn
