use privacy_utils::{mask_address, MaskedAddress, PrivacyLevel};
use soroban_sdk::{contractevent, Address, Env};

#[contractevent(data_format = "vec", topics = ["ticket_transferred"])]
pub struct TicketTransferred {
    #[topic]
    pub ticket_id: u64,
    pub from: MaskedAddress,
    pub to: MaskedAddress,
}

#[contractevent(data_format = "single-value", topics = ["ticket_used"])]
pub struct TicketUsed {
    #[topic]
    pub ticket_id: u64,
}

#[contractevent(data_format = "single-value", topics = ["ticket_minted"])]
pub struct TicketMinted {
    #[topic]
    pub ticket_id: u64,
}

#[contractevent(data_format = "single-value", topics = ["ticket_cancelled"])]
pub struct TicketCancelled {
    #[topic]
    pub ticket_id: u64,
}

/// Emit a ticket transfer event. Addresses are masked according to `privacy_level`.
pub fn emit_ticket_transferred(
    env: &Env,
    ticket_id: u64,
    from: Address,
    to: Address,
    privacy_level: PrivacyLevel,
) {
    TicketTransferred {
        ticket_id,
        from: mask_address(env, &from, privacy_level.clone()),
        to: mask_address(env, &to, privacy_level),
    }
    .publish(env);
}

pub fn emit_ticket_used(env: &Env, ticket_id: u64) {
    TicketUsed { ticket_id }.publish(env);
}

pub fn emit_ticket_minted(env: &Env, ticket_id: u64) {
    TicketMinted { ticket_id }.publish(env);
}

pub fn emit_ticket_cancelled(env: &Env, ticket_id: u64) {
    TicketCancelled { ticket_id }.publish(env);
}
