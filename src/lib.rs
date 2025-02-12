use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData, Clone)]
struct Badge {
    id: u64,
    holder: ComponentAddress,
}

#[blueprint]
mod access_control {
    enable_method_auth! {
        roles {
            admin => updatable_by: [OWNER];
            manager => updatable_by: [admin, OWNER];
        },
        methods {
            mint_badge => restrict_to: [admin];
            send_badge => restrict_to: [admin, OWNER];
            recall_badge => restrict_to: [admin, OWNER];
            hello_world => restrict_to: [admin];
        }
    }

    struct AccessControl {
        badge_resource_manager: NonFungibleResourceManager,
        badges: NonFungibleVault,
    }

    impl AccessControl {
        pub fn instantiate_access_control() -> (Global<AccessControl>, FungibleBucket) {
            // Reserve an address for the component
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(AccessControl::blueprint_id());

            // Create an Owner Badge
            let owner_badge = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata!(
                    init {
                        "name" => "Access Control Owner Badge", locked;
                    }
                ))
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);

                info!("HERE IS THE ADDRESS YOU HAVE BEEN LOOKING FOR: {:?}", owner_badge.resource_address());

            // Create a new Badge resource manager
            let badge_resource_manager = ResourceBuilder::new_integer_non_fungible::<Badge>(OwnerRole::None)
                .metadata(metadata!(
                    init {
                        "name" => "Access Badge", locked;
                    }
                ))
                .mint_roles(mint_roles! {
                    minter => rule!(require(owner_badge.resource_address()));
                    minter_updater => rule!(deny_all);
                })
                .recall_roles(recall_roles! {
                    recaller => rule!(require(owner_badge.resource_address()));
                    recaller_updater => rule!(deny_all);
                })
                .burn_roles(burn_roles! {
                    burner => rule!(require(owner_badge.resource_address()));
                    burner_updater => rule!(deny_all);
                })
                .create_with_no_initial_supply();

            // Populate an AccessControl struct and instantiate a new component
            let component = Self {
                badge_resource_manager: badge_resource_manager.clone(),
                badges: NonFungibleVault::with_bucket(badge_resource_manager.create_empty_bucket()),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(
                owner_badge.resource_address()
            ))))
            .roles(roles!(
                admin => rule!(require(owner_badge.resource_address()));
                manager => rule!(require(badge_resource_manager.address()));
            ))
            .with_address(address_reservation)
            .globalize();

            // Return the component and the owner badge
            (component, owner_badge)
        }

        pub fn mint_badge(&mut self, id: u64, holder: ComponentAddress) -> NonFungibleBucket {
            // Mint a new badge
            let badge_bucket = self.badge_resource_manager.mint_non_fungible(
                &NonFungibleLocalId::integer(id),
                Badge { id, holder },
            );
            badge_bucket
        }

        pub fn send_badge(&mut self, recipient: Global<Account>, badge_id: NonFungibleLocalId) {
            // Send a badge to another account
            
            
        }

        pub fn recall_badge(&mut self, account: Global<Account>, badge: Bucket) {
            // Recall a badge from another account
            // self.badge_resource_manager.recall(badge);
        }

        pub fn hello_world(&self) {
            // Function accessible only by managers with a badge
            info!("Hello, World!");
        }
    }
}

// New account
/*
    Account component address: account_sim1c956qr3kxlgypxwst89j9yf24tjc7zxd4up38x37zr6q4jxdx9rhma
    Public key: 02cb8ae5f186ef94854599722f6608e1cd66d0497492ac7d5a581f0a7e3d666708
    Private key: d71b9ffaf9669b100a9a83e89bb7a96a7972ac419f42513bb74f8f29aa040811
    Owner badge: resource_sim1nfzf2h73frult99zd060vfcml5kncq3mxpthusm9lkglvhsr0guahy:#1#

     resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3: 10000 Radix (XRD)
    resource_sim1t4ttnpe6myf86e2z59e45a0kyvw5gujeljh9xr9xpqlzms3k36t3xv: 1 Access Control Owner Badge
    Owned Non-fungibles Resources: 1
    resource_sim1nfzf2h73frult99zd060vfcml5kncq3mxpthusm9lkglvhsr0guahy: 1 Owner Badge
*/

// Package
/*
    package_sim1pk3cmat8st4ja2ms8mjqy2e9ptk8y6cx40v4qnfrkgnxcp2krkpr92
*/

// Account 2
/*
    Account component address: account_sim1cyyavav59dl55jur4eyxqz9wqyjycp2aua9dzduflfeefrfl623wgc
Public key: 03be34fbfe3ece98ed73894898042358cf7f7565cfe7378612a685f11baccc7fa7
Private key: 6fef5d74b2c0804851677f780bf5955e99fdf836640559e246de1166df412c1a
Owner badge: resource_sim1n2q4le7dpzucmpnksxj5ku28r3t776pgk879cahgm76c2kfpz48fpj:#1#
*/

// Creating a component
// resim call-function package_sim1pk3cmat8st4ja2ms8mjqy2e9ptk8y6cx40v4qnfrkgnxcp2krkpr92 AccessControl instantiate_access_control

// Component address 
// component_sim1cq48kwm3pd0s7vcx32nvw3urlf4q3pwlh2mch83s4ewhpfmdjmaqde