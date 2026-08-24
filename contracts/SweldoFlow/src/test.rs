#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, Ledger},
        token, Address, Env,
    };

    fn setup_test() -> (
        Env,
        Address,
        Address,
        Address,
        token::Client<'static>,
        token::AdminClient<'static>,
        SweldoFlowContractClient<'static>,
    ) {
        let env = Env::default();
        env.mock_all_signatures();

        let employer = Address::generate(&env);
        let employee = Address::generate(&env);
        let token_admin = Address::generate(&env);

        let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
        let token_client = token::Client::new(&env, &token_id.address());
        let token_admin_client = token::AdminClient::new(&env, &token_id.address());

        token_admin_client.mint(&employer, &1000_0000000);

        let contract_id = env.register(SweldoFlowContract, ());
        let client = SweldoFlowContractClient::new(&env, &contract_id);

        (
            env,
            employer,
            employee,
            token_id.address(),
            token_client,
            token_admin_client,
            client,
        )
    }

    #[test]
    fn test_1_happy_path_initialize_and_claim() {
        let (env, employer, employee, token, token_client, _, client) = setup_test();

        let start_time = 1000;
        let end_time = 2000;
        let total_amount = 1000_0000000;

        env.ledger().set_timestamp(start_time);
        client.initialize_stream(
            &employer,
            &employee,
            &token,
            &total_amount,
            &start_time,
            &end_time,
        );

        // Fast forward 50% of duration
        env.ledger().set_timestamp(1500);

        let claimed = client.claim_earned_salary(&employee);
        assert_eq!(claimed, 500_0000000);
        assert_eq!(token_client.balance(&employee), 500_0000000);
    }

    #[test]
    #[should_panic(expected = "Unauthorized caller")]
    fn test_2_unauthorized_caller_fails() {
        let (env, employer, employee, token, _, _, client) = setup_test();

        let start_time = 1000;
        let end_time = 2000;
        let total_amount = 1000_0000000;

        env.ledger().set_timestamp(start_time);
        client.initialize_stream(
            &employer,
            &employee,
            &token,
            &total_amount,
            &start_time,
            &end_time,
        );

        let attacker = Address::generate(&env);
        client.claim_earned_salary(&attacker);
    }

    #[test]
    fn test_3_state_verification_after_claim() {
        let (env, employer, employee, token, token_client, _, client) = setup_test();

        let start_time = 1000;
        let end_time = 2000;
        let total_amount = 1000_0000000;

        env.ledger().set_timestamp(start_time);
        client.initialize_stream(
            &employer,
            &employee,
            &token,
            &total_amount,
            &start_time,
            &end_time,
        );

        env.ledger().set_timestamp(1250); // 25% passed
        client.claim_earned_salary(&employee);

        let (config, claimable) = client.get_stream();
        assert_eq!(config.claimed_amount, 250_0000000);
        assert_eq!(claimable, 0);
        assert_eq!(token_client.balance(&employee), 250_0000000);
    }

    #[test]
    fn test_4_claim_before_start_time_returns_zero() {
        let (env, employer, employee, token, token_client, _, client) = setup_test();

        let start_time = 1000;
        let end_time = 2000;
        let total_amount = 1000_0000000;

        env.ledger().set_timestamp(500);
        client.initialize_stream(
            &employer,
            &employee,
            &token,
            &total_amount,
            &start_time,
            &end_time,
        );

        let claimed = client.claim_earned_salary(&employee);
        assert_eq!(claimed, 0);
        assert_eq!(token_client.balance(&employee), 0);
    }

    #[test]
    fn test_5_full_stream_drain_after_end_time() {
        let (env, employer, employee, token, token_client, _, client) = setup_test();

        let start_time = 1000;
        let end_time = 2000;
        let total_amount = 1000_0000000;

        env.ledger().set_timestamp(start_time);
        client.initialize_stream(
            &employer,
            &employee,
            &token,
            &total_amount,
            &start_time,
            &end_time,
        );

        env.ledger().set_timestamp(3000); // Past end time

        let claimed = client.claim_earned_salary(&employee);
        assert_eq!(claimed, 1000_0000000);
        assert_eq!(token_client.balance(&employee), 1000_0000000);
    }
}