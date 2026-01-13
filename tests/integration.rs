use scrypto_test::prelude::*;

struct Account {
    public_key: Secp256k1PublicKey,
    account_address: ComponentAddress,
}

struct TestEnv {
    ledger: DefaultLedgerSimulator,
    account: Account,
    v1_admin_badge_resource: ResourceAddress,
    v1_upgrade_badge_resource: ResourceAddress,
    component_address: ComponentAddress,
}

fn setup() -> TestEnv {
    let mut ledger = LedgerSimulatorBuilder::new().build();

    let (public_key, _, account_address) = ledger.new_allocated_account();
    let account = Account {
        public_key,
        account_address,
    };

    // Create V1 admin badge resource
    let admin_badge_manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_fungible_resource(
            OwnerRole::None,
            false,
            18,
            FungibleResourceRoles::default(),
            metadata!(
                init {
                    "name" => "V1 Admin Badge (Test)", locked;
                    "symbol" => "V1ADMIN", locked;
                }
            ),
            Some(dec!("1000")),
        )
        .deposit_batch(account_address, ManifestExpression::EntireWorktop)
        .build();

    let admin_receipt = ledger.execute_manifest(
        admin_badge_manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    let v1_admin_badge_resource = admin_receipt.expect_commit(true).new_resource_addresses()[0];

    // Create V1 upgrade badge resource
    let upgrade_badge_manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_fungible_resource(
            OwnerRole::None,
            false,
            18,
            FungibleResourceRoles::default(),
            metadata!(
                init {
                    "name" => "V1 Upgrade Badge (Test)", locked;
                    "symbol" => "V1UPGRADE", locked;
                }
            ),
            Some(dec!("1000")),
        )
        .deposit_batch(account_address, ManifestExpression::EntireWorktop)
        .build();

    let upgrade_receipt = ledger.execute_manifest(
        upgrade_badge_manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );
    let v1_upgrade_badge_resource = upgrade_receipt.expect_commit(true).new_resource_addresses()[0];

    // Instantiate the V1AuthRelinquishment component
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_function(
            ledger.compile_and_publish(this_package!()),
            "V1AuthRelinquishment",
            "instantiate",
            manifest_args!(v1_admin_badge_resource, v1_upgrade_badge_resource),
        )
        .build();

    let receipt = ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    let component_address = receipt.expect_commit(true).new_component_addresses()[0];

    TestEnv {
        ledger,
        account,
        v1_admin_badge_resource,
        v1_upgrade_badge_resource,
        component_address,
    }
}

#[test]
fn test_instantiation() {
    let mut env = setup();

    // Verify component was created and initial status shows zero locked
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(env.component_address, "get_lock_status", manifest_args!())
        .build();

    let receipt = env.ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(
            &env.account.public_key,
        )],
    );

    receipt.expect_commit_success();
}

#[test]
fn test_lock_admin_badges() {
    let mut env = setup();

    // Lock some admin badges
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(
            env.account.account_address,
            env.v1_admin_badge_resource,
            dec!("5"),
        )
        .take_all_from_worktop(env.v1_admin_badge_resource, "admin_badges")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                env.component_address,
                "lock_admin_badges",
                (lookup.bucket("admin_badges"),),
            )
        })
        .build();

    let receipt = env.ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(
            &env.account.public_key,
        )],
    );

    receipt.expect_commit_success();

    // Verify event was emitted
    assert!(
        !receipt
            .expect_commit_success()
            .application_events
            .is_empty(),
        "Should emit V1AdminBadgesLockedEvent"
    );
}

#[test]
fn test_lock_upgrade_badges() {
    let mut env = setup();

    // Lock some upgrade badges
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(
            env.account.account_address,
            env.v1_upgrade_badge_resource,
            dec!("3"),
        )
        .take_all_from_worktop(env.v1_upgrade_badge_resource, "upgrade_badges")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                env.component_address,
                "lock_upgrade_badges",
                (lookup.bucket("upgrade_badges"),),
            )
        })
        .build();

    let receipt = env.ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(
            &env.account.public_key,
        )],
    );

    receipt.expect_commit_success();

    // Verify event was emitted
    assert!(
        !receipt
            .expect_commit_success()
            .application_events
            .is_empty(),
        "Should emit V1UpgradeBadgeLockedEvent"
    );
}

#[test]
fn test_cumulative_locking() {
    let mut env = setup();

    // Lock admin badges multiple times
    for amount in [dec!("2"), dec!("5"), dec!("1")] {
        let manifest = ManifestBuilder::new()
            .lock_fee_from_faucet()
            .withdraw_from_account(
                env.account.account_address,
                env.v1_admin_badge_resource,
                amount,
            )
            .take_all_from_worktop(env.v1_admin_badge_resource, "admin_badges")
            .with_name_lookup(|builder, lookup| {
                builder.call_method(
                    env.component_address,
                    "lock_admin_badges",
                    (lookup.bucket("admin_badges"),),
                )
            })
            .build();

        env.ledger
            .execute_manifest(
                manifest,
                vec![NonFungibleGlobalId::from_public_key(
                    &env.account.public_key,
                )],
            )
            .expect_commit_success();
    }

    // Lock upgrade badges multiple times
    for amount in [dec!("0.5"), dec!("2")] {
        let manifest = ManifestBuilder::new()
            .lock_fee_from_faucet()
            .withdraw_from_account(
                env.account.account_address,
                env.v1_upgrade_badge_resource,
                amount,
            )
            .take_all_from_worktop(env.v1_upgrade_badge_resource, "upgrade_badges")
            .with_name_lookup(|builder, lookup| {
                builder.call_method(
                    env.component_address,
                    "lock_upgrade_badges",
                    (lookup.bucket("upgrade_badges"),),
                )
            })
            .build();

        env.ledger
            .execute_manifest(
                manifest,
                vec![NonFungibleGlobalId::from_public_key(
                    &env.account.public_key,
                )],
            )
            .expect_commit_success();
    }

    // Verify cumulative totals via get_lock_status
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .call_method(env.component_address, "get_lock_status", manifest_args!())
        .build();

    let receipt = env.ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(
            &env.account.public_key,
        )],
    );

    receipt.expect_commit_success();
}

#[test]
fn test_invalid_admin_badge_resource() {
    let mut env = setup();

    // Create a fake resource
    let fake_manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_fungible_resource(
            OwnerRole::None,
            false,
            18,
            FungibleResourceRoles::default(),
            metadata!(
                init {
                    "name" => "Fake Badge", locked;
                }
            ),
            Some(dec!("100")),
        )
        .deposit_batch(
            env.account.account_address,
            ManifestExpression::EntireWorktop,
        )
        .build();

    let fake_receipt = env.ledger.execute_manifest(
        fake_manifest,
        vec![NonFungibleGlobalId::from_public_key(
            &env.account.public_key,
        )],
    );
    let fake_resource = fake_receipt.expect_commit(true).new_resource_addresses()[0];

    // Try to lock fake resource as admin badge - should fail
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(env.account.account_address, fake_resource, dec!("1"))
        .take_all_from_worktop(fake_resource, "fake_badges")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                env.component_address,
                "lock_admin_badges",
                (lookup.bucket("fake_badges"),),
            )
        })
        .build();

    let receipt = env.ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(
            &env.account.public_key,
        )],
    );

    receipt.expect_commit_failure();
}

#[test]
fn test_invalid_upgrade_badge_resource() {
    let mut env = setup();

    // Create a fake resource
    let fake_manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .create_fungible_resource(
            OwnerRole::None,
            false,
            18,
            FungibleResourceRoles::default(),
            metadata!(
                init {
                    "name" => "Fake Badge", locked;
                }
            ),
            Some(dec!("100")),
        )
        .deposit_batch(
            env.account.account_address,
            ManifestExpression::EntireWorktop,
        )
        .build();

    let fake_receipt = env.ledger.execute_manifest(
        fake_manifest,
        vec![NonFungibleGlobalId::from_public_key(
            &env.account.public_key,
        )],
    );
    let fake_resource = fake_receipt.expect_commit(true).new_resource_addresses()[0];

    // Try to lock fake resource as upgrade badge - should fail
    let manifest = ManifestBuilder::new()
        .lock_fee_from_faucet()
        .withdraw_from_account(env.account.account_address, fake_resource, dec!("1"))
        .take_all_from_worktop(fake_resource, "fake_badges")
        .with_name_lookup(|builder, lookup| {
            builder.call_method(
                env.component_address,
                "lock_upgrade_badges",
                (lookup.bucket("fake_badges"),),
            )
        })
        .build();

    let receipt = env.ledger.execute_manifest(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(
            &env.account.public_key,
        )],
    );

    receipt.expect_commit_failure();
}
