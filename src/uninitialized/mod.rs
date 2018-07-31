//! Provides safe wrappers for uninitialzed memory.
//! 
//! These track whether values have been uninitialzed. This
//! adds some overhead, but is still faster than other safe workarounds
//! for uninitialzed data (e.g. default, linked lists, etc) in some cases.

use std::fmt;
use std::mem;
use std::ptr;

/// Used to store an uninitialized array.
///
/// This keeps track of which values have been initialized, allowing it to be used safely and dropped safely.
pub struct SafeUninitializedVec<T> {
    // THIS VEC CAN CONTAIN UNINITIALIZED DATA
    vals: Vec<T>,
    initialized: Vec<bool>,
}

impl<T> Drop for SafeUninitializedVec<T> {
    fn drop(&mut self) {
        let mut len = (&self.vals).len();
        let checked_len = (&self.initialized).len();
        // note that the vec that was originally passed using from_vec could be longer than
        // the length of this vec
        while len > checked_len {
            // all values outside the checked range cannot be uninitialized
            self.vals.pop();
            len -= 1;
        }
        while let Some(init) = self.initialized.pop() {
            len -= 1;
            if init {
                // the popped value will be dropped when it goes out of scope
                // this is only safe to do if the value is initialized
                self.vals.pop();
            } else {
                unsafe {
                    // if the value is uninitialized, then we decrease the length of vals
                    // this will not drop the value when it goes out of scope
                    self.vals.set_len(len);
                }
            }
        }
        // vals now has length 0, but still retains its capacity, so the allocated memory can be freed correctly.
    }
}

impl<T> SafeUninitializedVec<T> {
    /// Creates a new `SafeUninitialzedVec` with a set length.
    pub fn new(len: usize) -> SafeUninitializedVec<T> {
        let mut vec = Vec::with_capacity(len);
        unsafe {
            vec.set_len(len);
        }
        SafeUninitializedVec {
            vals: vec,
            initialized: vec![false; len],
        }
    }

    /// Uses and existing `Vec` to create a SafeUninitializedVec.
    ///
    /// If the length of the `Vec` is smaller than `len`,
    /// then if will reserve additional capacity and increase the length of the `Vec` without
    /// initializing the further elements.
    /// The struct keeps track of these elements, allowing it to be used safely
    pub fn from_vec(mut vec: Vec<T>, len: usize) -> SafeUninitializedVec<T> {
        let mut init_vals;
        let vec_len = (&vec).len();
        if len > vec_len {
            init_vals = vec![false; len];
            for i in 0..(&vec).len() {
                init_vals[i] = true;
            }
            vec.reserve(len - vec_len);
            unsafe {
                vec.set_len(len);
            }
        } else {
            init_vals = vec![true; len];
        }
        SafeUninitializedVec {
            vals: vec,
            initialized: init_vals,
        }
    }

    /// Returns either the backing vector or an error that contains self.
    /// This error allows the vector to continue to be used even if this fails.
    pub fn into_vec(mut self) -> Result<Vec<T>, UninitializedError<T>> {
        // Note: While none of this is marked as unsafe, it is nevertheless VERY UNSAFE
        // This is because self.vals can contain UNINITIALIZED DATA

        // Here, we check to see if all the values that are being returned are initialized
        let len = (&self.initialized).len();
        for i in 0..len {
            if !self.initialized[i] {
                return Err(UninitializedError::new(self));
            }
        }
        self.initialized = Vec::new();
        Ok(mem::replace(&mut self.vals, Vec::new()))
    }

    /// Gets the values and a vec that contains a value of true for every initialized value
    /// and false for every uninitialized value.
    pub unsafe fn get_parts(mut self) -> (Vec<T>, Vec<bool>) {
        (
            mem::replace(&mut self.vals, Vec::new()),
            mem::replace(&mut self.initialized, Vec::new()),
        )
    }

    /// Sets a value in the array to the provided value. This will initialize the
    /// value if it is uninitialized, and drops an existing value if present.
    pub fn set_value(&mut self, i: usize, val: T) {
        if self.initialized[i] {
            // replace vals[i] with val, running the destructor on the existing value
            self.vals[i] = val;
        } else {
            unsafe {
                // write to vals[i] without running a destructor on uninitialzed memory
                ptr::write(&mut self.vals[i], val);
                self.initialized[i] = true;
            }
        }
    }

    /// Gets a reference to an element of the vector. Will return none
    /// if the value is not initialized.
    pub fn get_value<'a>(&'a self, i: usize) -> Option<&'a T> {
        if self.initialized[i] {
            Some(&self.vals[i])
        } else {
            None
        }
    }

    /// Gets a mutable reference to an element of the vector. Will
    /// return none if the value is not initialized.
    pub fn get_value_mut<'a>(&'a mut self, i: usize) -> Option<&'a T> {
        if self.initialized[i] {
            Some(&mut self.vals[i])
        } else {
            None
        }
    }

    /// Swaps two elements.
    pub fn swap(&mut self, x: usize, y: usize) {
        self.initialized.swap(x, y);
        self.vals.swap(x, y);
    }

    /// Moves a value out of the array, marking its space in the array as uninitialized
    pub fn take(&mut self, i: usize) -> Option<T> {
        // first check that the value being requested is initialized
        if self.initialized[i] {
            // mark that the value has been deinitialized
            self.initialized[i] = false;
            unsafe {
                // create memory on the stack for the value to be copied into
                let mut value = mem::uninitialized();
                // move the value in the array into the result
                ptr::copy(&self.vals[i], &mut value, 1);
                Some(value)
            }
        } else {
            None
        }
    }
}

pub struct UninitializedError<T> {
    vec: SafeUninitializedVec<T>,
}

impl<T> UninitializedError<T> {
    fn new(vec: SafeUninitializedVec<T>) -> UninitializedError<T> {
        UninitializedError { vec: vec }
    }

    pub fn unwrap(self) -> SafeUninitializedVec<T> {
        self.vec
    }
}

impl<T> fmt::Debug for UninitializedError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Attempted to call into_vec on a value that still contained uninitialized data."
        )
    }
}
