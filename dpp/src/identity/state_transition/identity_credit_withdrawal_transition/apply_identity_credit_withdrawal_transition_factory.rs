use anyhow::Result;

use crate::{
    prelude::Identity,
    state_repository::{self, StateRepositoryLike},
};

use super::IdentityCreditWithdrawalTransition;

pub struct ApplyIdentityCreditWithdrawalTransition<SR>
where
    SR: StateRepositoryLike,
{
    state_repository: SR,
}

impl<SR> ApplyIdentityCreditWithdrawalTransition<SR>
where
    SR: StateRepositoryLike,
{
    pub fn new(state_repository: SR) -> Self {
        ApplyIdentityCreditWithdrawalTransition { state_repository }
    }

    pub async fn apply_data_contract_create_transition(
        &self,
        state_transition: &IdentityCreditWithdrawalTransition,
    ) -> Result<()> {
        let maybe_existing_identity: Option<Identity> = self
            .state_repository
            .fetch_identity(&state_transition.identity_id)
            .await?;

        let mut existing_identity = maybe_existing_identity.unwrap();

        existing_identity = existing_identity.reduce_balance(state_transition.amount);

        self.state_repository
            .update_identity(existing_identity)
            .await
    }
}
