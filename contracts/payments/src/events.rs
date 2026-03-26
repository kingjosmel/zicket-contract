use privacy_utils::{mask_address, MaskedAddress, PrivacyLevel};
use soroban_sdk::{contractevent, Address, Env, Symbol};

#[contractevent(data_format = "vec", topics = ["payment"])]
pub struct PaymentReceived {
    pub payment_id: u64,
    pub event_id: Symbol,
    pub payer: MaskedAddress,
    pub amount: i128,
}

#[contractevent(data_format = "vec", topics = ["refund"])]
pub struct PaymentRefunded {
    pub payment_id: u64,
    pub event_id: Symbol,
    pub payer: MaskedAddress,
    pub amount: i128,
}

#[contractevent(data_format = "vec", topics = ["ticket_issued"])]
pub struct TicketIssued {
    pub ticket_id: u64,
    pub event_id: Symbol,
    pub owner: MaskedAddress,
}

#[contractevent(data_format = "vec", topics = ["withdrawal"])]
pub struct RevenueWithdrawn {
    pub event_id: Symbol,
    pub organizer: MaskedAddress,
    pub amount: i128,
}

pub fn emit_payment_received(
    env: &Env,
    payment_id: u64,
    event_id: Symbol,
    payer: Address,
    amount: i128,
    privacy_level: PrivacyLevel,
) {
    PaymentReceived {
        payment_id,
        event_id,
        payer: mask_address(env, &payer, privacy_level),
        amount,
    }
    .publish(env);
}

pub fn emit_revenue_withdrawn(
    env: &Env,
    event_id: Symbol,
    organizer: Address,
    amount: i128,
    privacy_level: PrivacyLevel,
) {
    RevenueWithdrawn {
        event_id,
        organizer: mask_address(env, &organizer, privacy_level),
        amount,
    }
    .publish(env);
}

pub fn emit_payment_refunded(
    env: &Env,
    payment_id: u64,
    event_id: Symbol,
    payer: Address,
    amount: i128,
    privacy_level: PrivacyLevel,
) {
    PaymentRefunded {
        payment_id,
        event_id,
        payer: mask_address(env, &payer, privacy_level),
        amount,
    }
    .publish(env);
}

pub fn emit_ticket_issued(
    env: &Env,
    ticket_id: u64,
    event_id: Symbol,
    owner: Address,
    privacy_level: PrivacyLevel,
) {
    TicketIssued {
        ticket_id,
        event_id,
        owner: mask_address(env, &owner, privacy_level),
    }
    .publish(env);
}
