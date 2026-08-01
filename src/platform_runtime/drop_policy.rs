use serde::{Deserialize, Serialize};

use crate::platform_host::{PlatformElementId, PlatformHostRevision};

use super::drag_drop::SelfDrawnDropOperation;
use super::drag_drop_collection::SelfDrawnCollectionDropTarget;

/// Target identity supplied to a synchronous self-drawn drop policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SelfDrawnDropPolicyTarget {
    Generic {
        id: PlatformElementId,
    },
    Collection {
        id: PlatformElementId,
        target: SelfDrawnCollectionDropTarget,
    },
}

impl SelfDrawnDropPolicyTarget {
    pub fn id(&self) -> &PlatformElementId {
        match self {
            Self::Generic { id } | Self::Collection { id, .. } => id,
        }
    }

    pub fn collection_target(&self) -> Option<&SelfDrawnCollectionDropTarget> {
        match self {
            Self::Generic { .. } => None,
            Self::Collection { target, .. } => Some(target),
        }
    }
}

/// React Aria-compatible policy call selected during drop-target resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum SelfDrawnDropPolicyRequest {
    ShouldAcceptItemDrop {
        target: SelfDrawnDropPolicyTarget,
        types: Vec<String>,
    },
    GetDropOperation {
        target: SelfDrawnDropPolicyTarget,
        types: Vec<String>,
        allowed_operations: Vec<SelfDrawnDropOperation>,
    },
}

/// Revision-scoped synchronous query issued before a target is exposed as valid.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfDrawnDropPolicyQuery {
    pub frame_revision: PlatformHostRevision,
    pub event_sequence: u64,
    pub query_sequence: u64,
    pub policy_id: String,
    pub request: SelfDrawnDropPolicyRequest,
}

/// Typed answer to one [`SelfDrawnDropPolicyQuery`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SelfDrawnDropPolicyDecision {
    AcceptItemDrop { accepted: bool },
    DropOperation { operation: SelfDrawnDropOperation },
}

/// Response metadata must echo the query exactly. Stale or mismatched answers
/// are rejected and resolve to `cancel` rather than crossing a frame boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfDrawnDropPolicyResponse {
    pub frame_revision: PlatformHostRevision,
    pub event_sequence: u64,
    pub query_sequence: u64,
    pub decision: SelfDrawnDropPolicyDecision,
}

impl SelfDrawnDropPolicyResponse {
    pub fn accept_item_drop(query: &SelfDrawnDropPolicyQuery, accepted: bool) -> Self {
        Self::for_query(
            query,
            SelfDrawnDropPolicyDecision::AcceptItemDrop { accepted },
        )
    }

    pub fn drop_operation(
        query: &SelfDrawnDropPolicyQuery,
        operation: SelfDrawnDropOperation,
    ) -> Self {
        Self::for_query(
            query,
            SelfDrawnDropPolicyDecision::DropOperation { operation },
        )
    }

    pub fn for_query(
        query: &SelfDrawnDropPolicyQuery,
        decision: SelfDrawnDropPolicyDecision,
    ) -> Self {
        Self {
            frame_revision: query.frame_revision,
            event_sequence: query.event_sequence,
            query_sequence: query.query_sequence,
            decision,
        }
    }

    fn matches(&self, query: &SelfDrawnDropPolicyQuery) -> bool {
        self.frame_revision == query.frame_revision
            && self.event_sequence == query.event_sequence
            && self.query_sequence == query.query_sequence
    }
}

/// Transport outcome returned by a synchronous policy resolver.
///
/// A TSX process bridge should apply a bounded wait and map timeout, disconnect,
/// malformed data, and handler failure to the corresponding non-resolved case.
/// Every non-resolved case fails closed in the portable runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelfDrawnDropPolicyResolution {
    Resolved(SelfDrawnDropPolicyResponse),
    TimedOut,
    Unavailable,
    Failed,
}

/// Synchronous boundary used while the current pointer or keyboard event is
/// resolving a drop target. Rust never evaluates a JavaScript closure; a
/// process bridge may serialize this query and return a typed response.
pub trait SelfDrawnDropPolicyResolver {
    fn resolve(&mut self, query: &SelfDrawnDropPolicyQuery) -> SelfDrawnDropPolicyResolution;
}

impl<F> SelfDrawnDropPolicyResolver for F
where
    F: FnMut(&SelfDrawnDropPolicyQuery) -> SelfDrawnDropPolicyResolution,
{
    fn resolve(&mut self, query: &SelfDrawnDropPolicyQuery) -> SelfDrawnDropPolicyResolution {
        self(query)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct SelfDrawnDropPolicyEvaluationStats {
    pub(super) queries: u64,
    pub(super) failures: u64,
}

pub(super) struct SelfDrawnDropPolicyEvaluation<'a> {
    frame_revision: PlatformHostRevision,
    event_sequence: u64,
    query_sequence: u64,
    resolver: Option<&'a mut dyn SelfDrawnDropPolicyResolver>,
    stats: SelfDrawnDropPolicyEvaluationStats,
}

impl<'a> SelfDrawnDropPolicyEvaluation<'a> {
    pub(super) fn new(
        frame_revision: PlatformHostRevision,
        event_sequence: u64,
        resolver: Option<&'a mut dyn SelfDrawnDropPolicyResolver>,
    ) -> Self {
        Self {
            frame_revision,
            event_sequence,
            query_sequence: 0,
            resolver,
            stats: SelfDrawnDropPolicyEvaluationStats::default(),
        }
    }

    pub(super) fn stats(&self) -> SelfDrawnDropPolicyEvaluationStats {
        self.stats
    }

    pub(super) fn should_accept_item_drop(
        &mut self,
        policy_id: &str,
        target: SelfDrawnDropPolicyTarget,
        types: &[String],
    ) -> bool {
        let Some(decision) = self.resolve(
            policy_id,
            SelfDrawnDropPolicyRequest::ShouldAcceptItemDrop {
                target,
                types: types.to_vec(),
            },
        ) else {
            return false;
        };
        match decision {
            SelfDrawnDropPolicyDecision::AcceptItemDrop { accepted } => accepted,
            SelfDrawnDropPolicyDecision::DropOperation { .. } => {
                self.stats.failures = self.stats.failures.saturating_add(1);
                false
            }
        }
    }

    pub(super) fn get_drop_operation(
        &mut self,
        policy_id: &str,
        target: SelfDrawnDropPolicyTarget,
        types: &[String],
        allowed_operations: &[SelfDrawnDropOperation],
    ) -> SelfDrawnDropOperation {
        let Some(decision) = self.resolve(
            policy_id,
            SelfDrawnDropPolicyRequest::GetDropOperation {
                target,
                types: types.to_vec(),
                allowed_operations: allowed_operations.to_vec(),
            },
        ) else {
            return SelfDrawnDropOperation::Cancel;
        };
        match decision {
            SelfDrawnDropPolicyDecision::DropOperation { operation }
                if operation == SelfDrawnDropOperation::Cancel
                    || allowed_operations.contains(&operation) =>
            {
                operation
            }
            SelfDrawnDropPolicyDecision::DropOperation { .. }
            | SelfDrawnDropPolicyDecision::AcceptItemDrop { .. } => {
                self.stats.failures = self.stats.failures.saturating_add(1);
                SelfDrawnDropOperation::Cancel
            }
        }
    }

    fn resolve(
        &mut self,
        policy_id: &str,
        request: SelfDrawnDropPolicyRequest,
    ) -> Option<SelfDrawnDropPolicyDecision> {
        self.stats.queries = self.stats.queries.saturating_add(1);
        let Some(query_sequence) = self.query_sequence.checked_add(1) else {
            self.stats.failures = self.stats.failures.saturating_add(1);
            return None;
        };
        self.query_sequence = query_sequence;
        let query = SelfDrawnDropPolicyQuery {
            frame_revision: self.frame_revision,
            event_sequence: self.event_sequence,
            query_sequence,
            policy_id: policy_id.to_string(),
            request,
        };
        let Some(resolver) = self.resolver.as_deref_mut() else {
            self.stats.failures = self.stats.failures.saturating_add(1);
            return None;
        };
        match resolver.resolve(&query) {
            SelfDrawnDropPolicyResolution::Resolved(response) if response.matches(&query) => {
                Some(response.decision)
            }
            SelfDrawnDropPolicyResolution::Resolved(_)
            | SelfDrawnDropPolicyResolution::TimedOut
            | SelfDrawnDropPolicyResolution::Unavailable
            | SelfDrawnDropPolicyResolution::Failed => {
                self.stats.failures = self.stats.failures.saturating_add(1);
                None
            }
        }
    }
}
