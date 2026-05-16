use soroban_sdk::{contracttype, Address};

/// Opaque identifier for a child wallet account managed by a guardian.
pub type ChildId = Address;

/// Maximum amount (in stroops) a child may spend in a single transaction.
pub type SpendLimit = u64;

/// Period over which the spend limit resets.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LimitPeriod {
    Daily,
    Weekly,
    Monthly,
}

/// Full spending policy attached to a child account.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpendPolicy {
    pub child: ChildId,
    pub limit: SpendLimit,
    pub period: LimitPeriod,
}
