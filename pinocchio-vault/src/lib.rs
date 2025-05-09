#![no_std]

pub mod constants;
pub mod instruction;

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "no-bpf-entrypoint"))]
mod entrypoint;

pinocchio_pubkey::declare_id!("CEDgceYQMqc2RxpZRcKaSxHfLLtVw5BbKwwukyeJoyQV");
