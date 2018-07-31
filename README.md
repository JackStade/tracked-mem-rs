# Tracked Memory

Rust generally strives for zero cost abstractions, however there are some common things that are 
impossible to do without some overhead. This crate strives to provide some wrappers for some 
of these things.

Currently, there are two objects in this library.

## Uninitialized Memory

By keeping track of which items in an array have been initialzed, it is possible
to create an uninitialzed array that is entirely safe and that can be dropped correctly.

This requires allocating a vector of bools the length of the array. For large vectors, this 
can be quite expensive. 

## Runtime Ownership Checking

This provides an reference that may or may not own the value it points to. This is
potentially useful in a few cases.