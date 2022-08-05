use anyhow::Result;

use crate::{
    consensus::basic::{identity::IdentityInsufficientBalanceError, BasicError},
    identity::state_transition::identity_credit_withdrawal_transition::IdentityCreditWithdrawalTransition,
    prelude::Identity,
    state_repository::StateRepositoryLike,
    validation::ValidationResult,
};

pub struct IdentityCreditWithdrawalTransitionValidator<SR>
where
    SR: StateRepositoryLike,
{
    state_repository: SR,
}

impl<SR> IdentityCreditWithdrawalTransitionValidator<SR>
where
    SR: StateRepositoryLike,
{
    pub fn new(state_repository: SR) -> Self {
        IdentityCreditWithdrawalTransitionValidator { state_repository }
    }

    pub async fn validate_identity_credit_withdrawal_transition_state(
        &self,
        state_transition: &IdentityCreditWithdrawalTransition,
    ) -> ValidationResult<()> {
        let mut result: ValidationResult<()> = ValidationResult::default();

        let maybe_existing_identity: Result<Option<Identity>> = self
            .state_repository
            .fetch_identity(&state_transition.identity_id)
            .await;

        let existing_identity = match maybe_existing_identity {
            Ok(None) | Err(_) => {
                let err = BasicError::IdentityNotFoundError {
                    identity_id: state_transition.identity_id.clone(),
                };

                result.add_error(err);

                return result;
            }
            Ok(Some(ei)) => ei,
        };

        if existing_identity.get_balance() < state_transition.amount as i64 {
            let err = IdentityInsufficientBalanceError {
                identity_id: state_transition.identity_id.clone(),
                balance: existing_identity.balance,
            };

            result.add_error(err);

            return result;
        }

        result
    }
}
