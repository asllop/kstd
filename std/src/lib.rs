#![no_std]

//TODO:
// - HashMap and other maps.
// - io: stdio, bufread, read/write/seek traits.
// - fake env:args. hardcoded, from a file, etc.

#![feature(concat_idents)]
#![feature(format_args_nl)]
#![feature(llvm_asm)]
#![feature(log_syntax)]
#![feature(trace_macros)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(custom_test_frameworks)]
#![feature(cfg_accessible)]
#![feature(cfg_eval)]
#![feature(alloc_prelude)]
#![feature(core_intrinsics)]
#![feature(assert_matches)]
#![feature(async_stream)]
#![feature(const_format_args)]
#![feature(hashmap_internals)]
#![feature(try_reserve_kind)]
#![feature(extend_one)]

#[allow(unused_imports)] // macros from `alloc` are not used on all platforms
#[macro_use]
extern crate alloc as alloc_crate;

pub mod prelude;

// Public module declarations and re-exports
pub use alloc_crate::borrow;
pub use alloc_crate::boxed;
pub use alloc_crate::fmt;
pub use alloc_crate::format;
pub use alloc_crate::rc;

pub use alloc_crate::slice;

pub use alloc_crate::str;

pub use alloc_crate::string;

pub use alloc_crate::vec;

pub use core::any;

// The `no_inline`-attribute is required to make the documentation of all
// targets available.
// See https://github.com/rust-lang/rust/pull/57808#issuecomment-457390549 for
// more information.
#[doc(no_inline)] // Note (#82861): required for correct documentation
pub use core::arch;

pub use core::array;

pub use core::cell;

pub use core::char;

pub use core::clone;

pub use core::cmp;

pub use core::convert;

pub use core::default;

pub use core::future;

pub use core::hash;

pub use core::hint;

#[allow(deprecated, deprecated_in_future)]
pub use core::i128;

#[allow(deprecated, deprecated_in_future)]
pub use core::i16;

#[allow(deprecated, deprecated_in_future)]
pub use core::i32;

#[allow(deprecated, deprecated_in_future)]
pub use core::i64;

#[allow(deprecated, deprecated_in_future)]
pub use core::i8;

pub use core::intrinsics;

#[allow(deprecated, deprecated_in_future)]
pub use core::isize;

pub use core::iter;

pub use core::marker;

pub use core::mem;

pub use core::ops;

pub use core::option;

pub use core::pin;

pub use core::ptr;

pub use core::result;

pub use core::stream;

#[allow(deprecated, deprecated_in_future)]
pub use core::u128;

#[allow(deprecated, deprecated_in_future)]
pub use core::u16;

#[allow(deprecated, deprecated_in_future)]
pub use core::u32;

#[allow(deprecated, deprecated_in_future)]
pub use core::u64;

#[allow(deprecated, deprecated_in_future)]
pub use core::u8;

#[allow(deprecated, deprecated_in_future)]
pub use core::usize;

pub mod task {
    //! Types and Traits for working with asynchronous tasks.

    #[doc(inline)]
    pub use core::task::*;

    #[doc(inline)]
    pub use alloc_crate::task::*;
}

// Re-export macros defined in libcore.

#[allow(deprecated, deprecated_in_future)]
pub use core::{
    assert_eq, assert_ne, debug_assert, debug_assert_eq, debug_assert_ne, matches, r#try, todo,
    unimplemented, unreachable, write, writeln,
};

// Re-export built-in macros defined through libcore.
#[allow(deprecated)]
pub use core::{
    assert, assert_matches, cfg, column, compile_error, concat, concat_idents, const_format_args,
    env, file, format_args, format_args_nl, include, include_bytes, include_str, line, llvm_asm,
    log_syntax, module_path, option_env, stringify, trace_macros,
};

pub use core::primitive;

pub mod alloc;

pub mod collections;

// The standard macros that are not built-in to the compiler.
#[macro_use]
mod macros;