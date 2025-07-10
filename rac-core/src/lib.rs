#![cfg_attr(not(feature = "std"), no_std)]

pub mod articulation;
pub mod knode;

enum Status {
    Uninitialized,
    Active,
    Inactive,
    Error(u32),
}
