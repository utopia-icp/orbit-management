//! Canister lifecycle hooks.
use crate::core::{canister_config, ic_cdk::api::time, write_canister_config, CanisterConfig};
use candid::Principal;
use control_panel_api::{CanisterInit, DefaultWalletInit};
use ic_cdk_macros::{init, post_upgrade};

#[init]
async fn initialize(input: Option<CanisterInit>) {
    let init = input.unwrap_or_default();
    let shared_wallet_canister = match init.default_wallet {
        // todo: update shared wallet canister to the correct one
        DefaultWalletInit::InitSharedWalletCanister => Principal::anonymous(),
        DefaultWalletInit::SpecifiedWalletCanister(canister) => canister,
    };
    let config = CanisterConfig::new(shared_wallet_canister, time());

    write_canister_config(config);
}

#[post_upgrade]
async fn post_upgrade() {
    let current_config = canister_config();
    let updated_config = CanisterConfig::new(current_config.shared_wallet_canister, time());

    write_canister_config(updated_config);
}