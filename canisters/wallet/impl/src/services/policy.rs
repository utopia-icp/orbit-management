use crate::{
    core::generate_uuid_v4,
    errors::{AccessControlError, ProposalError},
    models::{
        access_control::{AccessControlPolicy, ResourceSpecifier, UserSpecifier},
        criteria::Criteria,
        specifier::ProposalSpecifier,
        ProposalPolicy,
    },
    repositories::{
        access_control::{AccessControlRepository, ACCESS_CONTROL_REPOSITORY},
        policy::{ProposalPolicyRepository, PROPOSAL_POLICY_REPOSITORY},
    },
};
use ic_canister_core::repository::Repository;
use ic_canister_core::{api::ServiceResult, types::UUID};
use lazy_static::lazy_static;
use std::sync::Arc;
use uuid::Uuid;

lazy_static! {
    pub static ref POLICY_SERVICE: Arc<PolicyService> = Arc::new(PolicyService::new(
        Arc::clone(&ACCESS_CONTROL_REPOSITORY),
        Arc::clone(&PROPOSAL_POLICY_REPOSITORY),
    ));
}

#[derive(Default, Debug)]
pub struct PolicyService {
    access_control_policy_repository: Arc<AccessControlRepository>,
    proposal_policy_repository: Arc<ProposalPolicyRepository>,
}

impl PolicyService {
    pub fn new(
        access_control_policy_repository: Arc<AccessControlRepository>,
        proposal_policy_repository: Arc<ProposalPolicyRepository>,
    ) -> Self {
        Self {
            access_control_policy_repository,
            proposal_policy_repository,
        }
    }

    pub fn get_proposal_policy(&self, id: &UUID) -> ServiceResult<ProposalPolicy> {
        let policy =
            self.proposal_policy_repository
                .get(id)
                .ok_or(ProposalError::PolicyNotFound {
                    id: Uuid::from_bytes(*id).hyphenated().to_string(),
                })?;

        Ok(policy)
    }

    pub async fn add_proposal_policy(
        &self,
        specifier: ProposalSpecifier,
        criteria: Criteria,
    ) -> ServiceResult<ProposalPolicy> {
        let id: uuid::Uuid = generate_uuid_v4().await;
        let policy = ProposalPolicy {
            id: *id.as_bytes(),
            specifier,
            criteria,
        };

        self.proposal_policy_repository
            .insert(policy.id, policy.clone());

        Ok(policy)
    }

    pub async fn edit_proposal_policy(
        &self,
        id: &UUID,
        specifier: ProposalSpecifier,
        criteria: Criteria,
    ) -> ServiceResult<ProposalPolicy> {
        let mut policy = self.get_proposal_policy(id)?;

        policy.specifier = specifier;
        policy.criteria = criteria;

        self.proposal_policy_repository
            .insert(policy.id, policy.to_owned());

        Ok(policy)
    }

    pub fn get_access_policy(&self, id: &UUID) -> ServiceResult<AccessControlPolicy> {
        let policy = self.access_control_policy_repository.get(id).ok_or(
            AccessControlError::PolicyNotFound {
                id: Uuid::from_bytes(*id).hyphenated().to_string(),
            },
        )?;

        Ok(policy)
    }

    pub async fn add_access_policy(
        &self,
        specifier: UserSpecifier,
        resource: ResourceSpecifier,
    ) -> ServiceResult<AccessControlPolicy> {
        let id: uuid::Uuid = generate_uuid_v4().await;
        let policy = AccessControlPolicy {
            id: *id.as_bytes(),
            user: specifier,
            resource,
        };

        self.access_control_policy_repository
            .insert(policy.id, policy.clone());

        Ok(policy)
    }

    pub async fn edit_access_policy(
        &self,
        id: &UUID,
        specifier: UserSpecifier,
        resource: ResourceSpecifier,
    ) -> ServiceResult<AccessControlPolicy> {
        let mut policy = self.get_access_policy(id)?;

        policy.user = specifier;
        policy.resource = resource;

        self.access_control_policy_repository
            .insert(policy.id, policy.to_owned());

        Ok(policy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::access_control::ProposalActionSpecifier;

    #[tokio::test]
    async fn test_proposal_policy_operations() {
        let service = POLICY_SERVICE.clone();
        let policy = service
            .add_proposal_policy(ProposalSpecifier::AddAccount, Criteria::AutoAdopted)
            .await;

        assert!(policy.is_ok());

        let policy = policy.unwrap();
        let fetched_policy = service.get_proposal_policy(&policy.id).unwrap();

        assert_eq!(fetched_policy.specifier, policy.specifier);
        assert_eq!(fetched_policy.criteria, policy.criteria);

        let policy = service
            .edit_proposal_policy(
                &policy.id,
                ProposalSpecifier::AddAccount,
                Criteria::AutoAdopted,
            )
            .await;

        assert!(policy.is_ok());

        let policy = policy.unwrap();
        let updated_policy = service.get_proposal_policy(&policy.id).unwrap();

        assert_eq!(updated_policy.specifier, policy.specifier);
        assert_eq!(updated_policy.criteria, policy.criteria);
    }

    #[tokio::test]
    async fn test_access_policy_operations() {
        let service = POLICY_SERVICE.clone();
        let policy = service
            .add_access_policy(
                UserSpecifier::Any,
                ResourceSpecifier::Proposal(ProposalActionSpecifier::List),
            )
            .await;

        assert!(policy.is_ok());

        let policy = policy.unwrap();
        let fetched_policy = service.get_access_policy(&policy.id).unwrap();

        assert_eq!(fetched_policy.user, policy.user);
        assert_eq!(fetched_policy.resource, policy.resource);

        let policy = service
            .edit_access_policy(
                &policy.id,
                UserSpecifier::Id(vec![[1; 16]]),
                ResourceSpecifier::Proposal(ProposalActionSpecifier::List),
            )
            .await;

        assert!(policy.is_ok());

        let policy = policy.unwrap();
        let updated_policy = service.get_access_policy(&policy.id).unwrap();

        assert_eq!(updated_policy.user, policy.user);
        assert_eq!(updated_policy.resource, policy.resource);
    }

    #[test]
    fn test_get_proposal_policy_not_found() {
        let service = POLICY_SERVICE.clone();
        let result = service.get_proposal_policy(&[1; 16]);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_access_policy_not_found() {
        let service = POLICY_SERVICE.clone();
        let result = service.get_access_policy(&[1; 16]);

        assert!(result.is_err());
    }
}