#![feature(dropck_eyepatch)]
#![feature(lazy_cell)]
#![feature(ptr_metadata)]
#![feature(ptr_addr_eq)]
#![feature(slice_as_chunks)]

mod callbacks;
mod fmt;
mod io;
mod lang;
mod macros;
mod serde;
mod sort;
mod threads;
