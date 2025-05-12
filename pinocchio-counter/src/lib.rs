#![no_std]

pub mod constants;
pub mod instruction;
pub mod state;

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "no-bpf-entrypoint"))]
mod entrypoint;

pinocchio_pubkey::declare_id!("9p4JPtbkdNjoHLsZ9Ht3221bE9DoPUTzQPaxogfvW7DF");
