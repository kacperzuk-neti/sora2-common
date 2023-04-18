use super::{EthSpec, FixedVector, Hash256, SyncCommittee};
use crate::light_client_header::LightClientHeaderRef;
use crate::light_client_update::*;
use crate::prelude::*;
use crate::LightClientHeaderCapella;
use crate::LightClientHeaderMerge;
use serde::{Deserialize, Serialize};

/// A LightClientBootstrap is the initializer we send over to lightclient nodes
/// that are trying to generate their basic storage when booting up.
#[superstruct(
    variants(Merge, Capella),
    variant_attributes(
        derive(
            Debug,
            Clone,
            Serialize,
            Deserialize,
            Derivative,
            ScaleEncode,
            ScaleDecode,
            TypeInfo,
            MaxEncodedLen,
        ),
        derivative(PartialEq),
        serde(bound = "T: EthSpec", deny_unknown_fields),
        scale_info(skip_type_params(T))
    ),
    ref_attributes(derive(PartialEq))
)]
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Derivative,
    ScaleEncode,
    ScaleDecode,
    TypeInfo,
    MaxEncodedLen,
)]
#[derivative(PartialEq)]
#[serde(bound = "T: EthSpec", untagged)]
#[scale_info(skip_type_params(T))]
pub struct LightClientBootstrap<T: EthSpec> {
    /// Requested beacon block header.
    #[superstruct(only(Merge), partial_getter(rename = "header_merge"))]
    pub header: LightClientHeaderMerge<T>,
    #[superstruct(only(Capella), partial_getter(rename = "header_capella"))]
    pub header: LightClientHeaderCapella<T>,
    /// The `SyncCommittee` used in the requested period.
    pub current_sync_committee: SyncCommittee<T>,
    /// Merkle proof for sync committee
    pub current_sync_committee_branch: FixedVector<Hash256, CurrentSyncCommitteeProofLen>,
}

impl<T: EthSpec> LightClientBootstrap<T> {
    pub fn header(&self) -> LightClientHeaderRef<T> {
        match self {
            Self::Merge(update) => LightClientHeaderRef::Merge(&update.header),
            Self::Capella(update) => LightClientHeaderRef::Capella(&update.header),
        }
    }
}

impl<'a, T: EthSpec> LightClientBootstrapRef<'a, T> {
    pub fn owned(&self) -> LightClientBootstrap<T> {
        match *self {
            Self::Merge(update) => LightClientBootstrap::Merge(update.clone()),
            Self::Capella(update) => LightClientBootstrap::Capella(update.clone()),
        }
    }
}

#[cfg(feature = "std")]
impl<T: EthSpec> crate::ForkVersionDeserialize for LightClientBootstrap<T> {
    fn deserialize_by_fork<'de, D: serde::Deserializer<'de>>(
        value: serde_json::value::Value,
        fork_name: crate::ForkName,
    ) -> Result<Self, D::Error> {
        let convert_err = |e| {
            serde::de::Error::custom(format!(
                "ExecutionPayloadHeader failed to deserialize: {:?}",
                e
            ))
        };

        Ok(match fork_name {
            crate::ForkName::Merge => {
                Self::Merge(serde_json::from_value(value).map_err(convert_err)?)
            }
            crate::ForkName::Capella => {
                Self::Capella(serde_json::from_value(value).map_err(convert_err)?)
            }
            crate::ForkName::Base | crate::ForkName::Altair => {
                return Err(serde::de::Error::custom(format!(
                    "ExecutionPayloadHeader failed to deserialize: unsupported fork '{}'",
                    fork_name
                )));
            }
        })
    }
}
