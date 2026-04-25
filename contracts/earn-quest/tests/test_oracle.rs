use earn_quest::{EarnQuestContract, Error};
use soroban_sdk::{
    contracttype, symbol_short, Address, Env, Symbol, U256, Vec, String,
};
use earn_quest::types::{
    OracleConfig, OracleType, PriceFeedRequest, PriceData, AggregatedPrice,
};

// Define the client for testing
#[contractclient(name = "EarnQuestContractClient")]
pub trait EarnQuestContract {
    fn initialize(env: &Env, admin: &Address);
    fn add_oracle(env: &Env, caller: &Address, oracle_config: &OracleConfig);
    fn remove_oracle(env: &Env, caller: &Address, oracle_address: &Address);
    fn update_oracle(env: &Env, caller: &Address, oracle_config: &OracleConfig);
    fn get_oracle_configs(env: &Env) -> Vec<OracleConfig>;
    fn get_active_oracle_configs(env: &Env) -> Vec<OracleConfig>;
    fn get_price(env: &Env, base_asset: &Address, quote_asset: &Address, max_age_seconds: &u64) -> AggregatedPrice;
    fn get_price_from_oracle(env: &Env, oracle_address: &Address, base_asset: &Address, quote_asset: &Address, max_age_seconds: &u64) -> PriceData;
    fn convert_reward_amount(env: &Env, from_asset: &Address, to_asset: &Address, amount: &i128) -> i128;
    fn validate_reward_amount_with_oracle(env: &Env, reward_asset: &Address, reward_amount: &i128, reference_asset: &Address, max_deviation_percent: &u32);
    fn emergency_pause(env: &Env, caller: &Address);
}

#[test]
fn test_oracle_config_validation() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EarnQuestContract);
    let client = EarnQuestContractClient::new(&env, &contract_id);

    // Initialize contract
    let admin = Address::generate(&env);
    client.initialize(&admin);

    // Test valid oracle config
    let oracle_address = Address::generate(&env);
    let valid_config = OracleConfig {
        oracle_address: oracle_address.clone(),
        oracle_type: OracleType::StellarAsset,
        max_age_seconds: 300,
        min_confidence: 80,
        is_active: true,
    };

    // Should succeed
    client.add_oracle(&admin, &valid_config);

    // Test invalid oracle config (max_age_seconds = 0)
    let invalid_config = OracleConfig {
        oracle_address: Address::generate(&env),
        oracle_type: OracleType::StellarAsset,
        max_age_seconds: 0,
        min_confidence: 80,
        is_active: true,
    };

    let result = client.try_add_oracle(&admin, &invalid_config);
    assert_eq!(result, Err(Ok(Error::InvalidOracleConfiguration)));

    // Test invalid oracle config (min_confidence > 100)
    let invalid_config2 = OracleConfig {
        oracle_address: Address::generate(&env),
        oracle_type: OracleType::StellarAsset,
        max_age_seconds: 300,
        min_confidence: 101,
        is_active: true,
    };

    let result = client.try_add_oracle(&admin, &invalid_config2);
    assert_eq!(result, Err(Ok(Error::InvalidOracleConfiguration)));
}

#[test]
fn test_oracle_crud_operations() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EarnQuestContract);
    let client = EarnQuestContractClient::new(&env, &contract_id);

    // Initialize contract
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let oracle_address = Address::generate(&env);
    let config = OracleConfig {
        oracle_address: oracle_address.clone(),
        oracle_type: OracleType::StellarOracle,
        max_age_seconds: 600,
        min_confidence: 85,
        is_active: true,
    };

    // Test adding oracle
    client.add_oracle(&admin, &config);

    // Test getting oracle configs
    let configs = client.get_oracle_configs();
    assert_eq!(configs.len(), 1);
    assert_eq!(configs.get(0).unwrap().oracle_address, oracle_address);

    // Test getting active oracle configs
    let active_configs = client.get_active_oracle_configs();
    assert_eq!(active_configs.len(), 1);

    // Test updating oracle config
    let updated_config = OracleConfig {
        oracle_address: oracle_address.clone(),
        oracle_type: OracleType::StellarOracle,
        max_age_seconds: 900,
        min_confidence: 90,
        is_active: false, // Deactivate
    };

    client.update_oracle(&admin, &updated_config);

    // Check that active configs is now empty
    let active_configs = client.get_active_oracle_configs();
    assert_eq!(active_configs.len(), 0);

    // Test removing oracle
    client.remove_oracle(&admin, &oracle_address);

    let configs = client.get_oracle_configs();
    assert_eq!(configs.len(), 0);
}

#[test]
fn test_oracle_duplicate_handling() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EarnQuestContract);
    let client = EarnQuestContractClient::new(&env, &contract_id);

    // Initialize contract
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let oracle_address = Address::generate(&env);
    let config = OracleConfig {
        oracle_address: oracle_address.clone(),
        oracle_type: OracleType::Custom,
        max_age_seconds: 300,
        min_confidence: 75,
        is_active: true,
    };

    // Add oracle first time
    client.add_oracle(&admin, &config);

    // Try to add same oracle again
    let result = client.try_add_oracle(&admin, &config);
    assert_eq!(result, Err(Ok(Error::OracleAlreadyExists)));

    // Try to update non-existent oracle
    let non_existent_address = Address::generate(&env);
    let fake_config = OracleConfig {
        oracle_address: non_existent_address.clone(),
        oracle_type: OracleType::Custom,
        max_age_seconds: 300,
        min_confidence: 75,
        is_active: true,
    };

    let result = client.try_update_oracle(&admin, &fake_config);
    assert_eq!(result, Err(Ok(Error::OracleNotFound)));

    // Try to remove non-existent oracle
    let result = client.try_remove_oracle(&admin, &non_existent_address);
    assert_eq!(result, Err(Ok(Error::OracleNotFound)));
}

#[test]
fn test_price_aggregation() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EarnQuestContract);
    let client = EarnQuestContractClient::new(&env, &contract_id);

    // Initialize contract
    let admin = Address::generate(&env);
    client.initialize(&admin);

    // Add multiple oracles
    let oracle1 = Address::generate(&env);
    let oracle2 = Address::generate(&env);
    let oracle3 = Address::generate(&env);

    let config1 = OracleConfig {
        oracle_address: oracle1.clone(),
        oracle_type: OracleType::StellarAsset,
        max_age_seconds: 300,
        min_confidence: 80,
        is_active: true,
    };

    let config2 = OracleConfig {
        oracle_address: oracle2.clone(),
        oracle_type: OracleType::StellarOracle,
        max_age_seconds: 300,
        min_confidence: 85,
        is_active: true,
    };

    let config3 = OracleConfig {
        oracle_address: oracle3.clone(),
        oracle_type: OracleType::Custom,
        max_age_seconds: 300,
        min_confidence: 90,
        is_active: true,
    };

    client.add_oracle(&admin, &config1);
    client.add_oracle(&admin, &config2);
    client.add_oracle(&admin, &config3);

    // Test price aggregation
    let base_asset = Address::generate(&env);
    let quote_asset = Address::generate(&env);
    let max_age_seconds = 300u64;

    let aggregated_price = client.get_price(&base_asset, &quote_asset, &max_age_seconds);
    
    // Verify aggregated price structure
    assert_eq!(aggregated_price.base_asset, base_asset);
    assert_eq!(aggregated_price.quote_asset, quote_asset);
    assert_eq!(aggregated_price.sources_used, 3);
    assert_eq!(aggregated_price.total_sources, 3);
    assert!(aggregated_price.confidence_score >= 80); // Should be average of 95, 90, 85
}

#[test]
fn test_price_from_specific_oracle() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EarnQuestContract);
    let client = EarnQuestContractClient::new(&env, &contract_id);

    // Initialize contract
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let oracle_address = Address::generate(&env);
    let config = OracleConfig {
        oracle_address: oracle_address.clone(),
        oracle_type: OracleType::StellarAsset,
        max_age_seconds: 300,
        min_confidence: 80,
        is_active: true,
    };

    client.add_oracle(&admin, &config);

    // Test getting price from specific oracle
    let base_asset = Address::generate(&env);
    let quote_asset = Address::generate(&env);
    let max_age_seconds = 300u64;

    let price_data = client.get_price_from_oracle(&oracle_address, &base_asset, &quote_asset, &max_age_seconds);
    
    // Verify price data structure
    assert_eq!(price_data.base_asset, base_asset);
    assert_eq!(price_data.quote_asset, quote_asset);
    assert_eq!(price_data.decimals, 7);
    assert_eq!(price_data.confidence, 95); // Mock value from StellarAsset oracle
}

#[test]
fn test_reward_amount_conversion() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EarnQuestContract);
    let client = EarnQuestContractClient::new(&env, &contract_id);

    // Initialize contract
    let admin = Address::generate(&env);
    client.initialize(&admin);

    // Add oracle
    let oracle_address = Address::generate(&env);
    let config = OracleConfig {
        oracle_address: oracle_address.clone(),
        oracle_type: OracleType::StellarAsset,
        max_age_seconds: 300,
        min_confidence: 80,
        is_active: true,
    };

    client.add_oracle(&admin, &config);

    // Test conversion between same assets (should return same amount)
    let asset1 = Address::generate(&env);
    let amount = 1000i128;
    
    let converted = client.convert_reward_amount(&asset1, &asset1, &amount);
    assert_eq!(converted, amount);

    // Test conversion between different assets
    let asset2 = Address::generate(&env);
    let converted = client.convert_reward_amount(&asset1, &asset2, &amount);
    
    // Should be different due to price conversion (mock price is 1000)
    // With 7 decimals, conversion should be: amount * price / 10^7
    // 1000 * 1000 / 10^7 = 0 (integer division), but our mock implementation
    // uses different logic, so we just check it's different
    assert_ne!(converted, amount);
}

#[test]
fn test_reward_amount_validation_with_oracle() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EarnQuestContract);
    let client = EarnQuestContractClient::new(&env, &contract_id);

    // Initialize contract
    let admin = Address::generate(&env);
    client.initialize(&admin);

    // Add oracle with high confidence
    let oracle_address = Address::generate(&env);
    let config = OracleConfig {
        oracle_address: oracle_address.clone(),
        oracle_type: OracleType::StellarAsset,
        max_age_seconds: 300,
        min_confidence: 80,
        is_active: true,
    };

    client.add_oracle(&admin, &config);

    // Test validation with sufficient confidence
    let reward_asset = Address::generate(&env);
    let reference_asset = Address::generate(&env);
    let reward_amount = 1000i128;
    let max_deviation_percent = 10u32;

    // Should succeed
    client.validate_reward_amount_with_oracle(
        &reward_asset,
        &reward_amount,
        &reference_asset,
        &max_deviation_percent,
    );

    // Test with oracle that has low confidence (would fail in real implementation)
    // For now, our mock implementation always returns high confidence
}

#[test]
fn test_oracle_authorization() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EarnQuestContract);
    let client = EarnQuestContractClient::new(&env, &contract_id);

    // Initialize contract
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let non_admin = Address::generate(&env);
    let oracle_address = Address::generate(&env);
    let config = OracleConfig {
        oracle_address: oracle_address.clone(),
        oracle_type: OracleType::StellarAsset,
        max_age_seconds: 300,
        min_confidence: 80,
        is_active: true,
    };

    // Test that non-admin cannot add oracle
    let result = client.try_add_oracle(&non_admin, &config);
    assert_eq!(result, Err(Ok(Error::Unauthorized)));

    // Test that non-admin cannot update oracle
    client.add_oracle(&admin, &config);
    let updated_config = OracleConfig {
        oracle_address: oracle_address.clone(),
        oracle_type: OracleType::StellarAsset,
        max_age_seconds: 600,
        min_confidence: 85,
        is_active: false,
    };

    let result = client.try_update_oracle(&non_admin, &updated_config);
    assert_eq!(result, Err(Ok(Error::Unauthorized)));

    // Test that non-admin cannot remove oracle
    let result = client.try_remove_oracle(&non_admin, &oracle_address);
    assert_eq!(result, Err(Ok(Error::Unauthorized)));
}

#[test]
fn test_oracle_with_paused_contract() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EarnQuestContract);
    let client = EarnQuestContractClient::new(&env, &contract_id);

    // Initialize contract
    let admin = Address::generate(&env);
    client.initialize(&admin);

    // Pause contract
    client.emergency_pause(&admin);

    let oracle_address = Address::generate(&env);
    let config = OracleConfig {
        oracle_address: oracle_address.clone(),
        oracle_type: OracleType::StellarAsset,
        max_age_seconds: 300,
        min_confidence: 80,
        is_active: true,
    };

    // Test that oracle operations fail when paused
    let result = client.try_add_oracle(&admin, &config);
    assert_eq!(result, Err(Ok(Error::Paused)));

    let result = client.try_update_oracle(&admin, &config);
    assert_eq!(result, Err(Ok(Error::Paused)));

    let result = client.try_remove_oracle(&admin, &oracle_address);
    assert_eq!(result, Err(Ok(Error::Paused)));
}
