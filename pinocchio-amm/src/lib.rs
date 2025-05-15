pub mod constants;
pub mod instruction;
pub mod state;

#[cfg(not(feature = "no-bpf-entrypoint"))]
mod entrypoint;

pinocchio_pubkey::declare_id!("6g5XmJou1kK2SX2QJN5rn4Ebuwiq1r6SoMDL8MUjaTHH");
