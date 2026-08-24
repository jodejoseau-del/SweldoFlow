#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Stream,
}

#[contracttype]
#[derive(Clone)]
pub struct StreamConfig {
    pub employer: Address,
    pub employee: Address,
    pub token: Address,
    pub total_amount: i128,
    pub start_time: u64,
    pub end_time: u64,
    pub claimed_amount: i128,
}

#[contract]
pub struct SweldoFlowContract;

#[contractimpl]
impl SweldoFlowContract {
    /// Initializes a salary streaming escrow between an employer and employee.
    pub fn initialize_stream(
        env: Env,
        employer: Address,
        employee: Address,
        token: Address,
        total_amount: i128,
        start_time: u64,
        end_time: u64,
    ) {
        employer.require_auth();

        if env.storage().instance().has(&DataKey::Stream) {
            panic!("Stream already initialized");
        }
        if end_time <= start_time {
            panic!("Invalid time bounds");
        }
        if total_amount <= 0 {
            panic!("Invalid amount");
        }

        let config = StreamConfig {
            employer: employer.clone(),
            employee,
            token: token.clone(),
            total_amount,
            start_time,
            end_time,
            claimed_amount: 0,
        };

        // Transfer full payroll allocation into contract escrow
        let client = token::Client::new(&env, &token);
        client.transfer(&employer, &env.current_contract_address(), &total_amount);

        env.storage().instance().set(&DataKey::Stream, &config);
    }

    /// Allows employee to claim accrued earned wages based on elapsed stream time.
    pub fn claim_earned_salary(env: Env, employee: Address) -> i128 {
        employee.require_auth();

        let mut config: StreamConfig = env
            .storage()
            .instance()
            .get(&DataKey::Stream)
            .expect("Stream not initialized");

        if employee != config.employee {
            panic!("Unauthorized caller");
        }

        let current_time = env.ledger().timestamp();
        if current_time <= config.start_time {
            return 0;
        }

        let elapsed = if current_time >= config.end_time {
            config.end_time - config.start_time
        } else {
            current_time - config.start_time
        };

        let total_duration = config.end_time - config.start_time;
        let vested = (config.total_amount * elapsed as i128) / (total_duration as i128);
        let claimable = vested - config.claimed_amount;

        if claimable <= 0 {
            return 0;
        }

        config.claimed_amount += claimable;
        env.storage().instance().set(&DataKey::Stream, &config);

        let client = token::Client::new(&env, &config.token);
        client.transfer(&env.current_contract_address(), &employee, &claimable);

        claimable
    }

    /// Fetches details about the active stream and calculates currently claimable tokens.
    pub fn get_stream(env: Env) -> (StreamConfig, i128) {
        let config: StreamConfig = env
            .storage()
            .instance()
            .get(&DataKey::Stream)
            .expect("Stream not initialized");

        let current_time = env.ledger().timestamp();
        let claimable = if current_time <= config.start_time {
            0
        } else {
            let elapsed = if current_time >= config.end_time {
                config.end_time - config.start_time
            } else {
                current_time - config.start_time
            };
            let total_duration = config.end_time - config.start_time;
            let vested = (config.total_amount * elapsed as i128) / (total_duration as i128);
            vested - config.claimed_amount
        };

        (config, claimable)
    }
}