use super::User;
use crate::core::CanisterConfig;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BankSettings {
    /// The current configuration of the bank canister.
    pub config: CanisterConfig,
    /// The list of users that are considered as owners of the bank canister.
    pub owners: Vec<User>,
}