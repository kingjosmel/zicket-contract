use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, BytesN, Env, Symbol};

const MOCK_EVENT_WASM: &[u8] =
    include_bytes!("../../../target/wasm32-unknown-unknown/release/mock_event_contract.wasm");

fn test_salt(env: &Env, fill: u8) -> BytesN<32> {
    BytesN::from_array(env, &[fill; 32])
}

fn setup_factory(env: &Env) -> FactoryContractClient<'static> {
    env.mock_all_auths();
    let contract_id = env.register(FactoryContract, ());
    let client = FactoryContractClient::new(env, &contract_id);

    let admin = Address::generate(env);
    let event_wasm_hash = env.deployer().upload_contract_wasm(MOCK_EVENT_WASM);
    let ticket_contract = Address::generate(env);
    let payments_contract = Address::generate(env);

    client.initialize(
        &admin,
        &event_wasm_hash,
        &ticket_contract,
        &payments_contract,
    );

    client
}

#[test]
fn test_initialize_stores_all_parameters() {
    let env = Env::default();
    let client = setup_factory(&env);
    let contract_id = client.address.clone();

    let is_init = env.as_contract(&contract_id, || storage::is_initialized(&env));
    assert!(is_init);

    env.as_contract(&contract_id, || {
        assert!(storage::get_admin(&env).is_ok());
        assert!(storage::get_event_wasm_hash(&env).is_ok());
        assert!(storage::get_ticket_contract(&env).is_ok());
        assert!(storage::get_payments_contract(&env).is_ok());
    });
}

#[test]
fn test_deploy_event_returns_address_and_stores_record() {
    let env = Env::default();
    let client = setup_factory(&env);

    let organizer = Address::generate(&env);
    let event_id = Symbol::new(&env, "event_1");
    let salt = test_salt(&env, 1);

    let contract_address = client.deploy_event(&organizer, &event_id, &salt);

    let deployed = client.get_deployed_event(&event_id);
    assert_eq!(deployed.event_id, event_id);
    assert_eq!(deployed.contract_address, contract_address);
    assert_eq!(deployed.organizer, organizer);

    assert_eq!(client.get_event_address(&event_id), contract_address);
}

#[test]
fn test_deploy_duplicate_event_fails() {
    let env = Env::default();
    let client = setup_factory(&env);

    let organizer = Address::generate(&env);
    let event_id = Symbol::new(&env, "event_1");

    client.deploy_event(&organizer, &event_id, &test_salt(&env, 1));

    let result = client.try_deploy_event(&organizer, &event_id, &test_salt(&env, 2));
    assert_eq!(
        result.err(),
        Some(Ok(FactoryError::EventAlreadyDeployed))
    );
}

#[test]
fn test_query_all_events() {
    let env = Env::default();
    let client = setup_factory(&env);

    let organizer = Address::generate(&env);
    let event_1 = Symbol::new(&env, "event_1");
    let event_2 = Symbol::new(&env, "event_2");

    client.deploy_event(&organizer, &event_1, &test_salt(&env, 1));
    client.deploy_event(&organizer, &event_2, &test_salt(&env, 2));

    let all = client.get_all_events();
    assert_eq!(all.len(), 2);
    assert!(all.contains(event_1));
    assert!(all.contains(event_2));
}

#[test]
fn test_query_organizer_events_across_organizers() {
    let env = Env::default();
    let client = setup_factory(&env);

    let org_1 = Address::generate(&env);
    let org_2 = Address::generate(&env);

    let e1 = Symbol::new(&env, "event_1");
    let e2 = Symbol::new(&env, "event_2");
    let e3 = Symbol::new(&env, "event_3");

    client.deploy_event(&org_1, &e1, &test_salt(&env, 1));
    client.deploy_event(&org_1, &e2, &test_salt(&env, 2));
    client.deploy_event(&org_2, &e3, &test_salt(&env, 3));

    let org_1_events = client.get_organizer_events(&org_1);
    let org_2_events = client.get_organizer_events(&org_2);

    assert_eq!(org_1_events.len(), 2);
    assert_eq!(org_2_events.len(), 1);
    assert!(org_1_events.contains(e1));
    assert!(org_1_events.contains(e2));
    assert!(org_2_events.contains(e3));
}

#[test]
fn test_query_nonexistent_event_fails() {
    let env = Env::default();
    let client = setup_factory(&env);

    let result = client.try_get_deployed_event(&Symbol::new(&env, "missing"));
    assert_eq!(
        result.err(),
        Some(Ok(FactoryError::EventNotFoundInRegistry))
    );
}
