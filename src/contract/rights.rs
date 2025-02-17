// LNP/BP Rust Library
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use amplify::Wrapper;
use core::fmt::Debug;
use std::collections::{btree_map, btree_set, BTreeMap, BTreeSet};

use commit_verify::{
    commit_encode, merkle::MerkleNode, ConsensusCommit, ConsensusMerkleCommit,
    MerkleSource, ToMerkleSource,
};
use strict_encoding::StrictEncode;

use super::{AssignmentVec, NodeId, EMPTY_ASSIGNMENT_VEC};
use crate::schema;

/// Holds definition of valencies for contract nodes, which is a set of
/// allowed valencies types
pub(crate) type PublicRightsInner = BTreeSet<schema::PublicRightType>;
pub(crate) type OwnedRightsInner =
    BTreeMap<schema::OwnedRightType, AssignmentVec>;
pub(crate) type ParentOwnedRightsInner =
    BTreeMap<NodeId, BTreeMap<schema::OwnedRightType, Vec<u16>>>;
pub(crate) type ParentPublicRightsInner =
    BTreeMap<NodeId, BTreeSet<schema::PublicRightType>>;

#[derive(Wrapper, Clone, PartialEq, Eq, Debug, Default, From)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
#[derive(StrictEncode, StrictDecode)]
pub struct OwnedRights(OwnedRightsInner);

impl OwnedRights {
    pub fn iter(
        &self,
    ) -> btree_map::Iter<'_, schema::OwnedRightType, AssignmentVec> {
        self.0.iter()
    }

    pub fn iter_mut(
        &mut self,
    ) -> btree_map::IterMut<'_, schema::OwnedRightType, AssignmentVec> {
        self.0.iter_mut()
    }

    pub fn assignments_by_type(
        &self,
        owned_rights_type: schema::OwnedRightType,
    ) -> &AssignmentVec {
        self.0
            .get(&owned_rights_type)
            .unwrap_or(&EMPTY_ASSIGNMENT_VEC)
    }
}

impl IntoIterator for OwnedRights {
    type Item = <OwnedRightsInner as IntoIterator>::Item;
    type IntoIter = <OwnedRightsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(
    Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, From,
)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
#[derive(StrictEncode, StrictDecode)]
pub struct PublicRights(PublicRightsInner);

impl PublicRights {
    pub fn iter(&self) -> btree_set::Iter<'_, schema::PublicRightType> {
        self.0.iter()
    }
}

impl IntoIterator for PublicRights {
    type Item = <PublicRightsInner as IntoIterator>::Item;
    type IntoIter = <PublicRightsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(
    Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, From,
)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
#[derive(StrictEncode, StrictDecode)]
pub struct ParentOwnedRights(ParentOwnedRightsInner);

impl ParentOwnedRights {
    pub fn iter(
        &self,
    ) -> btree_map::Iter<'_, NodeId, BTreeMap<schema::OwnedRightType, Vec<u16>>>
    {
        self.0.iter()
    }

    pub fn iter_mut(
        &mut self,
    ) -> btree_map::IterMut<
        '_,
        NodeId,
        BTreeMap<schema::OwnedRightType, Vec<u16>>,
    > {
        self.0.iter_mut()
    }
}

impl IntoIterator for ParentOwnedRights {
    type Item = <ParentOwnedRightsInner as IntoIterator>::Item;
    type IntoIter = <ParentOwnedRightsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(
    Wrapper, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, From,
)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate", transparent)
)]
#[derive(StrictEncode, StrictDecode)]
pub struct ParentPublicRights(ParentPublicRightsInner);

impl ParentPublicRights {
    pub fn iter(
        &self,
    ) -> btree_map::Iter<'_, NodeId, BTreeSet<schema::PublicRightType>> {
        self.0.iter()
    }

    pub fn iter_mut(
        &mut self,
    ) -> btree_map::IterMut<'_, NodeId, BTreeSet<schema::PublicRightType>> {
        self.0.iter_mut()
    }
}

impl IntoIterator for ParentPublicRights {
    type Item = <ParentPublicRightsInner as IntoIterator>::Item;
    type IntoIter = <ParentPublicRightsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(
    Clone,
    Copy,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Debug,
    StrictEncode,
    StrictDecode,
)]
pub struct PublicRightsLeaf(pub schema::PublicRightType);
impl commit_encode::Strategy for PublicRightsLeaf {
    type Strategy = commit_encode::strategies::UsingStrict;
}
impl ConsensusCommit for PublicRightsLeaf {
    type Commitment = MerkleNode;
}
impl ConsensusMerkleCommit for PublicRightsLeaf {
    const MERKLE_NODE_PREFIX: &'static str = "public_right";
}
impl ToMerkleSource for PublicRights {
    type Leaf = PublicRightsLeaf;

    fn to_merkle_source(&self) -> MerkleSource<Self::Leaf> {
        self.as_inner()
            .iter()
            .copied()
            .map(PublicRightsLeaf)
            .collect()
    }
}

#[derive(
    Clone,
    Copy,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Debug,
    StrictEncode,
    StrictDecode,
)]
pub struct OwnedRightsLeaf(pub schema::OwnedRightType, pub MerkleNode);
impl commit_encode::Strategy for OwnedRightsLeaf {
    type Strategy = commit_encode::strategies::UsingStrict;
}
impl ConsensusCommit for OwnedRightsLeaf {
    type Commitment = MerkleNode;
}
impl ConsensusMerkleCommit for OwnedRightsLeaf {
    const MERKLE_NODE_PREFIX: &'static str = "owned_right";
}
impl ToMerkleSource for OwnedRights {
    type Leaf = OwnedRightsLeaf;

    fn to_merkle_source(&self) -> MerkleSource<Self::Leaf> {
        self.as_inner()
            .iter()
            .flat_map(|(type_id, assignment)| {
                assignment.consensus_commitments().into_iter().map(
                    move |commitment| OwnedRightsLeaf(*type_id, commitment),
                )
            })
            .collect()
    }
}

#[derive(
    Clone,
    Copy,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Debug,
    StrictEncode,
    StrictDecode,
)]
pub struct ParentPublicRightsLeaf(pub NodeId, pub schema::PublicRightType);
impl commit_encode::Strategy for ParentPublicRightsLeaf {
    type Strategy = commit_encode::strategies::UsingStrict;
}
impl ConsensusCommit for ParentPublicRightsLeaf {
    type Commitment = MerkleNode;
}
impl ConsensusMerkleCommit for ParentPublicRightsLeaf {
    const MERKLE_NODE_PREFIX: &'static str = "parent_public_right";
}
impl ToMerkleSource for ParentPublicRights {
    type Leaf = ParentPublicRightsLeaf;

    fn to_merkle_source(&self) -> MerkleSource<Self::Leaf> {
        self.as_inner()
            .iter()
            .flat_map(|(node_id, i)| {
                i.iter().map(move |type_id| {
                    ParentPublicRightsLeaf(node_id.copy(), *type_id)
                })
            })
            .collect()
    }
}

#[derive(
    Clone,
    Copy,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Debug,
    StrictEncode,
    StrictDecode,
)]
pub struct ParentOwnedRightsLeaf(
    pub NodeId,
    pub schema::OwnedRightType,
    pub u16,
);
impl commit_encode::Strategy for ParentOwnedRightsLeaf {
    type Strategy = commit_encode::strategies::UsingStrict;
}
impl ConsensusCommit for ParentOwnedRightsLeaf {
    type Commitment = MerkleNode;
}
impl ConsensusMerkleCommit for ParentOwnedRightsLeaf {
    const MERKLE_NODE_PREFIX: &'static str = "parent_owned_right";
}
impl ToMerkleSource for ParentOwnedRights {
    type Leaf = ParentOwnedRightsLeaf;

    fn to_merkle_source(&self) -> MerkleSource<Self::Leaf> {
        self.as_inner()
            .iter()
            .flat_map(|(node_id, i)| {
                i.iter().flat_map(move |(type_id, prev_outs)| {
                    prev_outs.iter().map(move |prev_out| {
                        ParentOwnedRightsLeaf(
                            node_id.copy(),
                            *type_id,
                            *prev_out,
                        )
                    })
                })
            })
            .collect()
    }
}
