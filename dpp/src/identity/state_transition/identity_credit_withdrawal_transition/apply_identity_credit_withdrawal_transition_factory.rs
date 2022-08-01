use anyhow::Result;

use crate::{prelude::Identity, state_repository::StateRepositoryLike};

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
    pub async fn apply_data_contract_create_transition(
        &self,
        state_transition: &IdentityCreditWithdrawalTransition,
    ) -> Result<()> {
        let maybe_existing_identity: Option<Identity> = self
            .state_repository
            .fetch_identity(&state_transition.identity_id)
            .await?;

        let mut existing_identity = maybe_existing_identity.unwrap();

        // TODO: create special transaction for Core

        // TODO: what about negative balances yo???
        existing_identity.balance -= state_transition.amount as i64;

        self.state_repository
            .update_identity(existing_identity)
            .await
    }
}
