//! Iroha Queries provides declarative API for Iroha Queries.

#![allow(clippy::missing_inline_in_public_items)]

use std::time::SystemTime;

use iroha_crypto::prelude::*;
use iroha_derive::{FromVariant, Io};
use iroha_error::Result;
#[cfg(feature = "http_error")]
use iroha_http_server::http::HttpResponse;
use iroha_version::prelude::*;
use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[cfg(feature = "roles")]
use self::role::*;
use self::{account::*, asset::*, domain::*, peer::*, permissions::*, transaction::*};
use crate::Value;
use iroha_introspect::prelude::*;

/// Sized container for all possible Queries.
#[allow(clippy::pub_enum_variant_names)]
#[derive(
    Debug,
    Clone,
    Io,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    PartialEq,
    Eq,
    FromVariant,
    PartialOrd,
    Ord,
    Introspect
)]
pub enum QueryBox {
    /// `FindAllAccounts` variant.
    FindAllAccounts(FindAllAccounts),
    /// `FindAccountById` variant.
    FindAccountById(FindAccountById),
    /// `FindAccountKeyValueByIdAndKey` variant.
    FindAccountKeyValueByIdAndKey(FindAccountKeyValueByIdAndKey),
    /// `FindAccountsByName` variant.
    FindAccountsByName(FindAccountsByName),
    /// `FindAccountsByDomainName` variant.
    FindAccountsByDomainName(FindAccountsByDomainName),
    /// `FindAllAssets` variant.
    FindAllAssets(FindAllAssets),
    /// `FindAllAssetsDefinitions` variant.
    FindAllAssetsDefinitions(FindAllAssetsDefinitions),
    /// `FindAssetById` variant.
    FindAssetById(FindAssetById),
    /// `FindAssetByName` variant.
    FindAssetsByName(FindAssetsByName),
    /// `FindAssetsByAccountId` variant.
    FindAssetsByAccountId(FindAssetsByAccountId),
    /// `FindAssetsByAssetDefinitionId` variant.
    FindAssetsByAssetDefinitionId(FindAssetsByAssetDefinitionId),
    /// `FindAssetsByDomainName` variant.
    FindAssetsByDomainName(FindAssetsByDomainName),
    /// `FindAssetsByAccountIdAndAssetDefinitionId` variant.
    FindAssetsByAccountIdAndAssetDefinitionId(FindAssetsByAccountIdAndAssetDefinitionId),
    /// `FindAssetsByDomainNameAndAssetDefinitionId` variant.
    FindAssetsByDomainNameAndAssetDefinitionId(FindAssetsByDomainNameAndAssetDefinitionId),
    /// `FindAssetQuantityById` variant.
    FindAssetQuantityById(FindAssetQuantityById),
    /// `FindAssetQuantityById` variant.
    FindAssetKeyValueByIdAndKey(FindAssetKeyValueByIdAndKey),
    /// `FindAllDomains` variant.
    FindAllDomains(FindAllDomains),
    /// `FindDomainByName` variant.
    FindDomainByName(FindDomainByName),
    /// `FindAllPeers` variant.
    FindAllPeers(FindAllPeers),
    /// `FindTransactionsByAccountId` variant.
    FindTransactionsByAccountId(FindTransactionsByAccountId),
    /// `FindAllRoles` variant.
    #[cfg(feature = "roles")]
    FindAllRoles(FindAllRoles),
    /// `FindRolesByAccountId` variant.
    #[cfg(feature = "roles")]
    FindRolesByAccountId(FindRolesByAccountId),
    /// `FindPermissionTokensByAccountId` variant.
    FindPermissionTokensByAccountId(FindPermissionTokensByAccountId),
}

/// I/O ready structure to send queries.
#[derive(Debug, Io, Encode, Decode, Clone)]
pub struct QueryRequest {
    /// Timestamp of the query creation.
    #[codec(compact)]
    pub timestamp_ms: u128,
    /// Query definition.
    pub query: QueryBox,
}

declare_versioned_with_scale!(VersionedSignedQueryRequest 1..2);

/// I/O ready structure to send queries.
#[version_with_scale(n = 1, versioned = "VersionedSignedQueryRequest")]
#[derive(Debug, Clone, Io, Encode, Decode, Introspect)]
pub struct SignedQueryRequest {
    /// Timestamp of the query creation.
    #[codec(compact)]
    pub timestamp_ms: u128,
    /// Signature of the client who sends this query.
    pub signature: Signature,
    /// Query definition.
    pub query: QueryBox,
}

declare_versioned_with_scale!(VersionedQueryResult 1..2);

/// Sized container for all possible Query results.
#[version_with_scale(n = 1, versioned = "VersionedQueryResult")]
#[derive(Debug, Clone, Io, Serialize, Deserialize, Encode, Decode, Introspect)]
pub struct QueryResult(pub Value);

#[cfg(feature = "http_error")]
impl From<&QueryResult> for HttpResponse {
    fn from(result: &QueryResult) -> Self {
        use std::collections::BTreeMap;
        Self::ok(BTreeMap::default(), result.into())
    }
}
#[cfg(feature = "http_error")]
impl From<QueryResult> for HttpResponse {
    fn from(result: QueryResult) -> Self {
        (&result).into()
    }
}

impl QueryRequest {
    /// Constructs a new request with the `query`.
    #[allow(clippy::expect_used)]
    pub fn new(query: QueryBox) -> Self {
        let timestamp_ms = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Failed to get System Time.")
            .as_millis();
        QueryRequest {
            timestamp_ms,
            query,
        }
    }

    /// `Hash` of this request.
    pub fn hash(&self) -> Hash {
        let mut payload: Vec<u8> = self.query.clone().into();
        payload.extend_from_slice(&self.timestamp_ms.to_le_bytes());
        Hash::new(&payload)
    }

    /// Consumes self and returns a signed `QueryReuest`.
    ///
    /// # Errors
    /// Fails if signature creation fails
    pub fn sign(self, key_pair: &KeyPair) -> Result<SignedQueryRequest> {
        Ok(SignedQueryRequest {
            timestamp_ms: self.timestamp_ms,
            signature: Signature::new(key_pair.clone(), self.hash().as_ref())?,
            query: self.query,
        })
    }
}

impl SignedQueryRequest {
    /// `Hash` of this request.
    pub fn hash(&self) -> Hash {
        let mut payload: Vec<u8> = self.query.clone().into();
        payload.extend_from_slice(&self.timestamp_ms.to_le_bytes());
        Hash::new(&payload)
    }
}

#[cfg(feature = "roles")]
pub mod role {
    //! Queries related to `Role`.

    use iroha_derive::Io;
    use parity_scale_codec::{Decode, Encode};
    use serde::{Deserialize, Serialize};

    use crate::prelude::*;
    use iroha_introspect::prelude::*;

    /// `FindAllRoles` Iroha Query will find all `Roles`s presented.
    #[derive(
        Default,
        Copy,
        Clone,
        Debug,
        Io,
        Serialize,
        Deserialize,
        Encode,
        Decode,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
    Introspect
    )]
    pub struct FindAllRoles {}

    /// `FindRolesByAccountId` Iroha Query will find an `Role`s for a specified account.
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Introspect
    )]
    pub struct FindRolesByAccountId {
        /// `Id` of an account to find.
        pub id: EvaluatesTo<AccountId>,
    }

    /// The prelude re-exports most commonly used traits, structs and macros from this module.
    pub mod prelude {
        pub use super::{FindAllRoles, FindRolesByAccountId};
    }
}

pub mod permissions {
    //! Queries related to `PermissionToken`.

    use iroha_derive::Io;
    use parity_scale_codec::{Decode, Encode};
    use serde::{Deserialize, Serialize};

    use crate::prelude::*;
    use iroha_introspect::prelude::*;

    /// `FindPermissionTokensByAccountId` Iroha Query will find an `Role`s for a specified account.
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Introspect
    )]
    pub struct FindPermissionTokensByAccountId {
        /// `Id` of an account to find.
        pub id: EvaluatesTo<AccountId>,
    }

    /// The prelude re-exports most commonly used traits, structs and macros from this module.
    pub mod prelude {
        pub use super::FindPermissionTokensByAccountId;
    }
}

pub mod account {
    //! Queries related to `Account`.

    use iroha_derive::Io;
    use parity_scale_codec::{Decode, Encode};
    use serde::{Deserialize, Serialize};
    use iroha_introspect::prelude::*;

    use crate::prelude::*;

    // TODO: Better to have find all account ids query instead.
    /// `FindAllAccounts` Iroha Query will find all `Account`s presented.
    #[derive(
        Default,
        Copy,
        Clone,
        Debug,
        Io,
        Serialize,
        Deserialize,
        Encode,
        Decode,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Introspect,
    )]
    pub struct FindAllAccounts {}

    /// `FindAccountById` Iroha Query will find an `Account` by it's identification.
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord,
        Introspect,
    )]
    pub struct FindAccountById {
        /// `Id` of an account to find.
        pub id: EvaluatesTo<AccountId>,
    }

    /// `FindAccountById` Iroha Query will find a [`Value`] of the key-value metadata pair
    /// in the specified account.
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Introspect,
    )]
    pub struct FindAccountKeyValueByIdAndKey {
        /// `Id` of an account to find.
        pub id: EvaluatesTo<AccountId>,
        /// Key of the specific key-value in the Account's metadata.
        pub key: EvaluatesTo<String>,
    }

    /// `FindAccountsByName` Iroha Query will get `Account`s name as input and
    /// find all `Account`s with this name.
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Introspect,
    )]
    pub struct FindAccountsByName {
        /// `name` of accounts to find.
        pub name: EvaluatesTo<Name>,
    }

    /// `FindAccountsByDomainName` Iroha Query will get `Domain`s name as input and
    /// find all `Account`s under this `Domain`.
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Introspect,
    )]
    pub struct FindAccountsByDomainName {
        /// `domain_name` under which accounts should be found.
        pub domain_name: EvaluatesTo<Name>,
    }

    impl FindAllAccounts {
        /// Default `FindAllAccounts` constructor.
        pub const fn new() -> Self {
            FindAllAccounts {}
        }
    }

    impl FindAccountById {
        /// Default `FindAccountById` constructor.
        pub fn new(id: impl Into<EvaluatesTo<AccountId>>) -> Self {
            let id = id.into();
            FindAccountById { id }
        }
    }

    impl FindAccountKeyValueByIdAndKey {
        /// Default `FindAccountById` constructor.
        pub fn new(
            id: impl Into<EvaluatesTo<AccountId>>,
            key: impl Into<EvaluatesTo<String>>,
        ) -> Self {
            let id = id.into();
            let key = key.into();
            FindAccountKeyValueByIdAndKey { id, key }
        }
    }

    impl FindAccountsByName {
        /// Default `FindAccountsByName` constructor.
        pub fn new(name: impl Into<EvaluatesTo<Name>>) -> Self {
            let name = name.into();
            FindAccountsByName { name }
        }
    }

    impl FindAccountsByDomainName {
        /// Default `FindAccountsByDomainName` constructor.
        pub fn new(domain_name: impl Into<EvaluatesTo<Name>>) -> Self {
            let domain_name = domain_name.into();
            FindAccountsByDomainName { domain_name }
        }
    }

    /// The prelude re-exports most commonly used traits, structs and macros from this crate.
    pub mod prelude {
        pub use super::{
            FindAccountById, FindAccountKeyValueByIdAndKey, FindAccountsByDomainName,
            FindAccountsByName, FindAllAccounts,
        };
    }
}

pub mod asset {
    //! Queries related to `Asset`.

    #![allow(clippy::missing_inline_in_public_items)]

    use iroha_derive::Io;
    use parity_scale_codec::{Decode, Encode};
    use serde::{Deserialize, Serialize};

    use crate::prelude::*;
    use iroha_introspect::prelude::*;

    /// `FindAllAssets` Iroha Query will find all `Asset`s presented in Iroha Peer.
    #[derive(
        Copy,
        Clone,
        Debug,
        Default,
        Io,
        Serialize,
        Deserialize,
        Encode,
        Decode,
        PartialEq,
        Eq,
        PartialOrd,
        Ord, Introspect,
    )]
    pub struct FindAllAssets {}

    /// `FindAllAssetsDefinitions` Iroha Query will find all `AssetDefinition`s presented
    /// in Iroha Peer.
    #[derive(
        Copy,
        Clone,
        Debug,
        Default,
        Io,
        Serialize,
        Deserialize,
        Encode,
        Decode,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
    Introspect,
    )]
    pub struct FindAllAssetsDefinitions {}

    /// `FindAssetById` Iroha Query will find an `Asset` by it's identification in Iroha `Peer`.
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Introspect,
    )]
    pub struct FindAssetById {
        /// `Id` of an `Asset` to find.
        pub id: EvaluatesTo<AssetId>,
    }

    /// `FindAssetsByName` Iroha Query will get `Asset`s name as input and
    /// find all `Asset`s with it in Iroha `Peer`.
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Introspect,
    )]
    pub struct FindAssetsByName {
        /// `Name` of `Asset`s to find.
        pub name: EvaluatesTo<Name>,
    }

    /// `FindAssetsByAccountId` Iroha Query will get `AccountId` as input and find all `Asset`s
    /// owned by the `Account` in Iroha Peer.
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Introspect,
    )]
    pub struct FindAssetsByAccountId {
        /// `AccountId` under which assets should be found.
        pub account_id: EvaluatesTo<AccountId>,
    }

    /// `FindAssetsByAssetDefinitionId` Iroha Query will get `AssetDefinitionId` as input and
    /// find all `Asset`s with this `AssetDefinition` in Iroha Peer.
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Introspect,
    )]
    pub struct FindAssetsByAssetDefinitionId {
        /// `AssetDefinitionId` with type of `Asset`s should be found.
        pub asset_definition_id: EvaluatesTo<AssetDefinitionId>,
    }

    /// `FindAssetsByDomainName` Iroha Query will get `Domain`s name as input and
    /// find all `Asset`s under this `Domain` in Iroha `Peer`.
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Introspect,
    )]
    pub struct FindAssetsByDomainName {
        /// `Name` of the domain under which assets should be found.
        pub domain_name: EvaluatesTo<Name>,
    }

    // TODO: remove as it is the same as `FindAssetById`
    /// `FindAssetsByAccountIdAndAssetDefinitionId` Iroha Query will get `AccountId` and
    /// `AssetDefinitionId` as inputs and find all `Asset`s owned by the `Account`
    /// with this `AssetDefinition` in Iroha Peer.
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Introspect,
    )]
    pub struct FindAssetsByAccountIdAndAssetDefinitionId {
        /// `AccountId` under which assets should be found.
        pub account_id: EvaluatesTo<AccountId>,
        /// `AssetDefinitionId` which assets should be found.
        pub asset_definition_id: EvaluatesTo<AssetDefinitionId>,
    }

    /// `FindAssetsByDomainNameAndAssetDefinitionId` Iroha Query will get `Domain`'s name and
    /// `AssetDefinitionId` as inputs and find all `Asset`s under the `Domain`
    /// with this `AssetDefinition` in Iroha `Peer`.
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Introspect,
    )]
    pub struct FindAssetsByDomainNameAndAssetDefinitionId {
        /// `Name` of the domain under which assets should be found.
        pub domain_name: EvaluatesTo<Name>,
        /// `AssetDefinitionId` assets of which type should be found.
        pub asset_definition_id: EvaluatesTo<AssetDefinitionId>,
    }

    /// `FindAssetQuantityById` Iroha Query will get `AssetId` as input and find `Asset::quantity`
    /// parameter's value if `Asset` is presented in Iroha Peer.
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Introspect,
    )]
    pub struct FindAssetQuantityById {
        /// `Id` of an `Asset` to find quantity of.
        pub id: EvaluatesTo<AssetId>,
    }

    /// `FindAssetQuantityById` Iroha Query will get `AssetId` and key as input and find [`Value`]
    /// of the key-value pair stored in this asset.
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Introspect,
    )]
    pub struct FindAssetKeyValueByIdAndKey {
        /// `Id` of an `Asset` acting as `Store`.
        pub id: EvaluatesTo<AssetId>,
        /// The key of the key-value pair stored in the asset.
        pub key: EvaluatesTo<Name>,
    }

    impl FindAllAssets {
        /// Default `FindAllAssets` constructor.
        pub const fn new() -> Self {
            FindAllAssets {}
        }
    }

    impl FindAllAssetsDefinitions {
        /// Default `FindAllAssetsDefinitions` constructor.
        pub const fn new() -> Self {
            FindAllAssetsDefinitions {}
        }
    }

    impl FindAssetById {
        /// Default `FindAssetById` constructor
        pub fn new(id: impl Into<EvaluatesTo<AssetId>>) -> Self {
            let id = id.into();
            Self { id }
        }
    }

    impl FindAssetsByName {
        /// Default `FindAssetsByName` constructor
        pub fn new(name: impl Into<EvaluatesTo<Name>>) -> Self {
            let name = name.into();
            Self { name }
        }
    }

    impl FindAssetsByAccountId {
        /// Default `FindAssetsByAccountId` constructor.
        pub fn new(account_id: impl Into<EvaluatesTo<AccountId>>) -> Self {
            let account_id = account_id.into();
            FindAssetsByAccountId { account_id }
        }
    }

    impl FindAssetsByAssetDefinitionId {
        /// Default `FindAssetsByAssetDefinitionId` constructor.
        pub fn new(asset_definition_id: impl Into<EvaluatesTo<AssetDefinitionId>>) -> Self {
            let asset_definition_id = asset_definition_id.into();
            FindAssetsByAssetDefinitionId {
                asset_definition_id,
            }
        }
    }

    impl FindAssetsByDomainName {
        /// Default `FindAssetsByDomainName` constructor
        pub fn new(domain_name: impl Into<EvaluatesTo<Name>>) -> Self {
            let domain_name = domain_name.into();
            Self { domain_name }
        }
    }

    impl FindAssetsByAccountIdAndAssetDefinitionId {
        /// Default `FindAssetsByAccountIdAndAssetDefinitionId` constructor.
        pub fn new(
            account_id: impl Into<EvaluatesTo<AccountId>>,
            asset_definition_id: impl Into<EvaluatesTo<AssetDefinitionId>>,
        ) -> Self {
            let account_id = account_id.into();
            let asset_definition_id = asset_definition_id.into();
            FindAssetsByAccountIdAndAssetDefinitionId {
                account_id,
                asset_definition_id,
            }
        }
    }

    impl FindAssetsByDomainNameAndAssetDefinitionId {
        /// Default `FindAssetsByDomainNameAndAssetDefinitionId` constructor
        pub fn new(
            domain_name: impl Into<EvaluatesTo<Name>>,
            asset_definition_id: impl Into<EvaluatesTo<AssetDefinitionId>>,
        ) -> Self {
            let domain_name = domain_name.into();
            let asset_definition_id = asset_definition_id.into();
            Self {
                domain_name,
                asset_definition_id,
            }
        }
    }

    impl FindAssetQuantityById {
        /// Default `FindAssetQuantityById` constructor.
        pub fn new(id: impl Into<EvaluatesTo<AssetId>>) -> Self {
            let id = id.into();
            FindAssetQuantityById { id }
        }
    }

    impl FindAssetKeyValueByIdAndKey {
        /// Default [`FindAssetKeyValueByIdAndKey`] constructor.
        pub fn new(id: impl Into<EvaluatesTo<AssetId>>, key: impl Into<EvaluatesTo<Name>>) -> Self {
            let id = id.into();
            let key = key.into();
            Self { id, key }
        }
    }

    /// The prelude re-exports most commonly used traits, structs and macros from this crate.
    pub mod prelude {
        pub use super::{
            FindAllAssets, FindAllAssetsDefinitions, FindAssetById, FindAssetKeyValueByIdAndKey,
            FindAssetQuantityById, FindAssetsByAccountId,
            FindAssetsByAccountIdAndAssetDefinitionId, FindAssetsByAssetDefinitionId,
            FindAssetsByDomainName, FindAssetsByDomainNameAndAssetDefinitionId, FindAssetsByName,
        };
    }
}

pub mod domain {
    //! Queries related to `Domain`.

    #![allow(clippy::missing_inline_in_public_items)]

    use iroha_derive::Io;
    use parity_scale_codec::{Decode, Encode};
    use serde::{Deserialize, Serialize};

    use crate::prelude::*;
    use iroha_introspect::prelude::*;

    /// `FindAllDomains` Iroha Query will find all `Domain`s presented in Iroha `Peer`.
    #[derive(
        Copy,
        Clone,
        Debug,
        Default,
        Io,
        Serialize,
        Deserialize,
        Encode,
        Decode,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Introspect,
    )]
    pub struct FindAllDomains {}

    /// `FindDomainByName` Iroha Query will find a `Domain` by it's identification in Iroha `Peer`.
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Introspect,
    )]
    pub struct FindDomainByName {
        /// Name of the domain to find.
        pub name: EvaluatesTo<Name>,
    }

    impl FindAllDomains {
        /// Default `FindAllDomains` constructor.
        pub const fn new() -> Self {
            FindAllDomains {}
        }
    }

    impl FindDomainByName {
        /// Default `FindDomainByName` constructor.
        pub fn new(name: impl Into<EvaluatesTo<Name>>) -> Self {
            let name = name.into();
            FindDomainByName { name }
        }
    }

    /// The prelude re-exports most commonly used traits, structs and macros from this crate.
    pub mod prelude {
        pub use super::{FindAllDomains, FindDomainByName};
    }
}

pub mod peer {
    //! Queries related to `Domain`.

    use iroha_derive::Io;
    use parity_scale_codec::{Decode, Encode};
    use serde::{Deserialize, Serialize};
    use iroha_introspect::prelude::*;

    /// `FindAllPeers` Iroha Query will find all trusted `Peer`s presented in current Iroha `Peer`.
    #[derive(
        Copy,
        Clone,
        Debug,
        Default,
        Io,
        Serialize,
        Deserialize,
        Encode,
        Decode,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Introspect,
    )]
    pub struct FindAllPeers {}

    /// `FindAllParameters` Iroha Query will find all `Peer`s parameters.
    #[derive(
        Copy,
        Clone,
        Debug,
        Default,
        Io,
        Serialize,
        Deserialize,
        Encode,
        Decode,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Introspect
    )]
    pub struct FindAllParameters {}

    impl FindAllPeers {
        ///Default `FindAllPeers` constructor.
        pub const fn new() -> Self {
            FindAllPeers {}
        }
    }

    impl FindAllParameters {
        /// Default `FindAllParameters` constructor.
        pub const fn new() -> Self {
            FindAllParameters {}
        }
    }
    /// The prelude re-exports most commonly used traits, structs and macros from this crate.
    pub mod prelude {
        pub use super::{FindAllParameters, FindAllPeers};
    }
}

pub mod transaction {
    //! Queries related to `Transaction`.

    #![allow(clippy::missing_inline_in_public_items)]

    use iroha_derive::Io;
    use parity_scale_codec::{Decode, Encode};
    use serde::{Deserialize, Serialize};

    use crate::account::prelude::AccountId;
    use crate::expression::EvaluatesTo;
    use iroha_introspect::prelude::*;

    /// `FindTransactionsByAccountId` Iroha Query will find all transaction included in blockchain
    /// for the account
    #[derive(
        Clone, Debug, Io, Serialize, Deserialize, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Introspect
    )]
    pub struct FindTransactionsByAccountId {
        /// Signer's `AccountId` under which transactions should be found.
        pub account_id: EvaluatesTo<AccountId>,
    }

    impl FindTransactionsByAccountId {
        ///Default [`FindTransactionsByAccountId`] constructor.
        pub fn new(account_id: impl Into<EvaluatesTo<AccountId>>) -> Self {
            let account_id = account_id.into();
            FindTransactionsByAccountId { account_id }
        }
    }
    /// The prelude re-exports most commonly used traits, structs and macros from this crate.
    pub mod prelude {
        pub use super::FindTransactionsByAccountId;
    }
}

/// The prelude re-exports most commonly used traits, structs and macros from this crate.
pub mod prelude {
    #[cfg(feature = "roles")]
    pub use super::role::prelude::*;
    pub use super::{
        account::prelude::*, asset::prelude::*, domain::prelude::*, peer::prelude::*,
        permissions::prelude::*, transaction::*, QueryBox, QueryRequest, QueryResult,
        SignedQueryRequest, VersionedQueryResult, VersionedSignedQueryRequest,
    };
}
