use scrypto::prelude::*;

// Status of V1 badge locking
#[derive(ScryptoSbor, Debug)]
pub struct V1LockStatus {
    pub admin_badges_locked: Decimal,
    pub upgrade_badges_locked: Decimal,
    pub admin_badge_resource: ResourceAddress,
    pub upgrade_badge_resource: ResourceAddress,
}

// Event emitted when V1 admin badges are locked
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct V1AdminBadgesLockedEvent {
    pub badges_locked: Decimal,
    pub total_locked_now: Decimal,
    pub timestamp: Instant,
}

// Event emitted when V1 upgrade badges are locked
#[derive(ScryptoSbor, ScryptoEvent)]
pub struct V1UpgradeBadgeLockedEvent {
    pub badges_locked: Decimal,
    pub total_locked_now: Decimal,
    pub timestamp: Instant,
}

#[blueprint]
#[events(V1AdminBadgesLockedEvent, V1UpgradeBadgeLockedEvent)]
mod rns_v1_badge_lockers {
    use super::*;

    // V1 Auth Relinquishment Contract
    //
    // A minimal, standalone contract for permanently locking RNS V1 admin and upgrade badges.
    // This demonstrates irreversible commitment to the V2 upgrade by accepting V1 badges
    // which can never be withdrawn.

    pub struct V1AuthRelinquishment {
        // Vault holding permanently locked V1 admin badges
        v1_admin_badges_vault: Vault,

        // Vault holding permanently locked V1 upgrade badges
        v1_upgrade_badges_vault: Vault,

        // Expected resource address for V1 admin badges (validated on deposit)
        v1_admin_badge_resource: ResourceAddress,

        // Expected resource address for V1 upgrade badges (validated on deposit)
        v1_upgrade_badge_resource: ResourceAddress,
    }

    impl V1AuthRelinquishment {
        // Instantiates the V1 Auth Relinquishment contract.
        //
        // # Arguments
        // * `v1_admin_badge_resource` - Resource address of V1 admin badges that can be locked
        // * `v1_upgrade_badge_resource` - Resource address of V1 upgrade badges that can be locked
        //
        // # Returns
        // The instantiated component (no admin badge - zero admin capability)
        pub fn instantiate(
            v1_admin_badge_resource: ResourceAddress,
            v1_upgrade_badge_resource: ResourceAddress,
        ) -> Global<V1AuthRelinquishment> {
            Self {
                v1_admin_badges_vault: Vault::new(v1_admin_badge_resource),
                v1_upgrade_badges_vault: Vault::new(v1_upgrade_badge_resource),
                v1_admin_badge_resource,
                v1_upgrade_badge_resource,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .metadata(metadata! {
                init {
                    "name" => "RNS V1 Auth Relinquishment", locked;
                    "description" => "Permanently locks RNS V1 admin and upgrade badges to demonstrate irreversible commitment to V2.", locked;
                    "tags" => ["rns", "v1", "deprecation", "lock"], locked;
                }
            })
            .globalize()
        }

        // Permanently locks V1 admin badges into this contract.
        //
        // Any community member can contribute V1 admin badges which will be locked indefinitely.
        // This is part of the V1 deprecation process to demonstrate commitment to V2.
        //
        // # Arguments
        // * `v1_admin_badges` - Bucket containing V1 admin badges to lock
        //
        // # Panics
        // * If the bucket contains the wrong resource type
        pub fn lock_admin_badges(&mut self, v1_admin_badges: Bucket) {
            assert_eq!(
                v1_admin_badges.resource_address(),
                self.v1_admin_badge_resource,
                "Invalid V1 admin badge resource. Expected {:?}, received {:?}",
                self.v1_admin_badge_resource,
                v1_admin_badges.resource_address()
            );

            let locked_count = v1_admin_badges.amount();

            self.v1_admin_badges_vault.put(v1_admin_badges);

            Runtime::emit_event(V1AdminBadgesLockedEvent {
                badges_locked: locked_count,
                total_locked_now: self.v1_admin_badges_vault.amount(),
                timestamp: Clock::current_time_rounded_to_minutes(),
            });
        }

        // Permanently locks V1 upgrade badges into this contract.
        //
        // Any community member can contribute V1 upgrade badges which will be locked indefinitely.
        // This is part of the V1 deprecation process to demonstrate commitment to V2.
        //
        // # Arguments
        // * `v1_upgrade_badges` - Bucket containing V1 upgrade badges to lock
        //
        // # Panics
        // * If the bucket contains the wrong resource type
        pub fn lock_upgrade_badges(&mut self, v1_upgrade_badges: Bucket) {
            assert_eq!(
                v1_upgrade_badges.resource_address(),
                self.v1_upgrade_badge_resource,
                "Invalid V1 upgrade badge resource. Expected {:?}, received {:?}",
                self.v1_upgrade_badge_resource,
                v1_upgrade_badges.resource_address()
            );

            let locked_count = v1_upgrade_badges.amount();

            self.v1_upgrade_badges_vault.put(v1_upgrade_badges);

            Runtime::emit_event(V1UpgradeBadgeLockedEvent {
                badges_locked: locked_count,
                total_locked_now: self.v1_upgrade_badges_vault.amount(),
                timestamp: Clock::current_time_rounded_to_minutes(),
            });
        }

        // Returns the current lock status showing how many V1 badges are locked.
        //
        // # Returns
        // `V1LockStatus` containing counts of locked badges and their resource addresses
        pub fn get_lock_status(&self) -> V1LockStatus {
            V1LockStatus {
                admin_badges_locked: self.v1_admin_badges_vault.amount(),
                upgrade_badges_locked: self.v1_upgrade_badges_vault.amount(),
                admin_badge_resource: self.v1_admin_badge_resource,
                upgrade_badge_resource: self.v1_upgrade_badge_resource,
            }
        }
    }
}
