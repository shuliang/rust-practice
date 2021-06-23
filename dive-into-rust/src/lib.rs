#![feature(generators, generator_trait, dropck_eyepatch)]

// 第一部分：基础知识
pub mod macrodemo;
pub mod traits;
pub mod typesize;

// 第二部分：内存安全
pub mod borrow;
pub mod dereference;
pub mod drop;
pub mod drop_check;
pub mod interior_mut;
pub mod lifetime;
pub mod nll;
pub mod unsafe_demo;
pub mod variance;

// 第三部分：高级抽象
pub mod closure;
pub mod dispatch;
pub mod generator;

// 第四部分：线程安全
pub mod arc;
pub mod barrier;
pub mod channel;
pub mod condvar;
pub mod crossbeam;
pub mod global;
pub mod parkinglot;
pub mod philosopher;
pub mod rayon;
pub mod subnormal;
pub mod threadpool;
