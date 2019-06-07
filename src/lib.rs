mod u128_func;
mod u16_func;
mod u32_func;
mod u64_func;
mod u8_func;
mod usize_func;

mod i128_func;
mod i16_func;
mod i32_func;
mod i64_func;
mod i8_func;
mod isize_func;

pub use usize_func::*;

pub mod u128 {
    pub use super::u128_func::*;
}

pub mod usize {
    pub use super::usize_func::*;
}

pub mod u64 {
    pub use super::u64_func::*;
}

pub mod u32 {
    pub use super::u32_func::*;
}

pub mod u16 {
    pub use super::u16_func::*;
}

pub mod u8 {
    pub use super::u8_func::*;
}

pub mod i128 {
    pub use super::i128_func::*;
}

pub mod isize {
    pub use super::isize_func::*;
}

pub mod i64 {
    pub use super::i64_func::*;
}

pub mod i32 {
    pub use super::i32_func::*;
}

pub mod i16 {
    pub use super::i16_func::*;
}

pub mod i8 {
    pub use super::i8_func::*;
}
