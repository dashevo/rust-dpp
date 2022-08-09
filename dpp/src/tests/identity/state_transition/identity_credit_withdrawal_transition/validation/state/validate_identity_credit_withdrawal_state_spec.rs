use crate::{
    identity::state_transition::identity_credit_withdrawal_transition::{
        validation::state::validate_identity_credit_withdrawal_transition_state::IdentityCreditWithdrawalTransitionValidator,
        IdentityCreditWithdrawalTransition,
    },
    state_repository::{MockStateRepositoryLike, StateRepositoryLike},
};

pub fn setup_test<SR: StateRepositoryLike>(
    state_repository_mock: SR,
) -> (
    IdentityCreditWithdrawalTransition,
    IdentityCreditWithdrawalTransitionValidator<SR>,
) {
    (
        IdentityCreditWithdrawalTransition::default(),
        IdentityCreditWithdrawalTransitionValidator::new(state_repository_mock),
    )
}

#[cfg(test)]
mod validate_identity_credit_withdrawal_transition_state_factory {
    use crate::prelude::Identity;

    use super::*;

    #[tokio::test]
    async fn should_return_invalid_result_if_identity_not_found() {
        let mut state_repository = MockStateRepositoryLike::default();

        state_repository
            .expect_fetch_identity::<Identity>()
            .returning(|_| anyhow::Ok(None));

        let (state_transition, validator) = setup_test(state_repository);

        let result = validator
            .validate_identity_credit_withdrawal_transition_state(&state_transition)
            .await;
    }
}
