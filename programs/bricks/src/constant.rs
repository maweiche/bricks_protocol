pub const GOLD_MEMBERSHIP: u64 = 100;
pub const PLATINUM_MEMBERSHIP: u64 = 1000;

use anchor_lang::declare_id;

pub const ED25519_PROGRAM_ID: &str = "Ed25519SigVerify111111111111111111111111111";

pub mod admin_wallet {
    use super::*;
    declare_id!("6KuX26FZqzqpsHDLfkXoBXbQRPEDEbstqNiPBKHNJQ9e");
}
pub mod signing_authority {
    use super::*;
    declare_id!("S1GvFEzpWUM5EwYZMFLcEdMEXEjhUZzhhQeN5AvG6mw");
}
pub mod membership_wallet {
    use super::*;
    declare_id!("BDEECMrE5dv4cc5na6Fi8sNkfzYxckd6ZjsuEzp7hXnJ");
}

pub mod protocol_currency {
    use super::*;
    declare_id!("5yRcNyhKR7BpAx8DUrqfuhjcpMEVKxdQT1KAS8o72ZAW");
}