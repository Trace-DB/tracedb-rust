pub use crate::prelude::*;

/// Explain response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HybridExplain {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_path_timings: Option<Vec<AccessPathTiming>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_paths: Option<Vec<AccessPathExplain>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_budget: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduped_candidate_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dirty_feature_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_stop_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_fallback_triggered: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_feature_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_visibility_guard_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_visibility_guard_removed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freshness_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fusion_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hot_overlay_searched: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexical_cache_hits: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexical_cache_misses: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexical_indexed_documents: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexical_scored_documents: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materialized_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_feature_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_versions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened_candidate_streams: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_feature_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_timings: Option<Vec<QueryPhaseTiming>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planner_candidates: Option<Vec<Candidate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_epoch: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_epoch: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalar_filter_applied: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalar_filter_predicates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalar_filter_removed_records: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalar_filter_visible_records: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_epoch: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_scanned: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped_access_paths: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_mask_visible_records: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_candidates: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_candidates: Option<i64>,
}

impl HybridExplain {
    pub fn builder() -> HybridExplainBuilder {
        <HybridExplainBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HybridExplainBuilder {
    access_path_timings: Option<Vec<AccessPathTiming>>,
    access_paths: Option<Vec<AccessPathExplain>>,
    candidate_budget: Option<i64>,
    deduped_candidate_count: Option<i64>,
    dirty_feature_count: Option<i64>,
    early_stop_reason: Option<String>,
    exact_fallback_triggered: Option<bool>,
    failed_feature_count: Option<i64>,
    final_visibility_guard_count: Option<i64>,
    final_visibility_guard_removed: Option<i64>,
    freshness_mode: Option<String>,
    fusion_method: Option<String>,
    hot_overlay_searched: Option<bool>,
    lexical_cache_hits: Option<i64>,
    lexical_cache_misses: Option<i64>,
    lexical_indexed_documents: Option<i64>,
    lexical_scored_documents: Option<i64>,
    materialized_count: Option<i64>,
    missing_feature_count: Option<i64>,
    module_versions: Option<Vec<String>>,
    opened_candidate_streams: Option<Vec<String>>,
    pending_feature_count: Option<i64>,
    phase_timings: Option<Vec<QueryPhaseTiming>>,
    planner_candidates: Option<Vec<Candidate>>,
    policy_epoch: Option<i64>,
    read_epoch: Option<i64>,
    returned_count: Option<i64>,
    scalar_filter_applied: Option<bool>,
    scalar_filter_predicates: Option<Vec<String>>,
    scalar_filter_removed_records: Option<i64>,
    scalar_filter_visible_records: Option<i64>,
    schema_epoch: Option<i64>,
    segments_scanned: Option<i64>,
    selected_strategy: Option<String>,
    skipped_access_paths: Option<Vec<String>>,
    tenant_mask_visible_records: Option<i64>,
    text_candidates: Option<i64>,
    vector_candidates: Option<i64>,
}

impl HybridExplainBuilder {
    pub fn access_path_timings(mut self, value: Vec<AccessPathTiming>) -> Self {
        self.access_path_timings = Some(value);
        self
    }

    pub fn access_paths(mut self, value: Vec<AccessPathExplain>) -> Self {
        self.access_paths = Some(value);
        self
    }

    pub fn candidate_budget(mut self, value: i64) -> Self {
        self.candidate_budget = Some(value);
        self
    }

    pub fn deduped_candidate_count(mut self, value: i64) -> Self {
        self.deduped_candidate_count = Some(value);
        self
    }

    pub fn dirty_feature_count(mut self, value: i64) -> Self {
        self.dirty_feature_count = Some(value);
        self
    }

    pub fn early_stop_reason(mut self, value: impl Into<String>) -> Self {
        self.early_stop_reason = Some(value.into());
        self
    }

    pub fn exact_fallback_triggered(mut self, value: bool) -> Self {
        self.exact_fallback_triggered = Some(value);
        self
    }

    pub fn failed_feature_count(mut self, value: i64) -> Self {
        self.failed_feature_count = Some(value);
        self
    }

    pub fn final_visibility_guard_count(mut self, value: i64) -> Self {
        self.final_visibility_guard_count = Some(value);
        self
    }

    pub fn final_visibility_guard_removed(mut self, value: i64) -> Self {
        self.final_visibility_guard_removed = Some(value);
        self
    }

    pub fn freshness_mode(mut self, value: impl Into<String>) -> Self {
        self.freshness_mode = Some(value.into());
        self
    }

    pub fn fusion_method(mut self, value: impl Into<String>) -> Self {
        self.fusion_method = Some(value.into());
        self
    }

    pub fn hot_overlay_searched(mut self, value: bool) -> Self {
        self.hot_overlay_searched = Some(value);
        self
    }

    pub fn lexical_cache_hits(mut self, value: i64) -> Self {
        self.lexical_cache_hits = Some(value);
        self
    }

    pub fn lexical_cache_misses(mut self, value: i64) -> Self {
        self.lexical_cache_misses = Some(value);
        self
    }

    pub fn lexical_indexed_documents(mut self, value: i64) -> Self {
        self.lexical_indexed_documents = Some(value);
        self
    }

    pub fn lexical_scored_documents(mut self, value: i64) -> Self {
        self.lexical_scored_documents = Some(value);
        self
    }

    pub fn materialized_count(mut self, value: i64) -> Self {
        self.materialized_count = Some(value);
        self
    }

    pub fn missing_feature_count(mut self, value: i64) -> Self {
        self.missing_feature_count = Some(value);
        self
    }

    pub fn module_versions(mut self, value: Vec<String>) -> Self {
        self.module_versions = Some(value);
        self
    }

    pub fn opened_candidate_streams(mut self, value: Vec<String>) -> Self {
        self.opened_candidate_streams = Some(value);
        self
    }

    pub fn pending_feature_count(mut self, value: i64) -> Self {
        self.pending_feature_count = Some(value);
        self
    }

    pub fn phase_timings(mut self, value: Vec<QueryPhaseTiming>) -> Self {
        self.phase_timings = Some(value);
        self
    }

    pub fn planner_candidates(mut self, value: Vec<Candidate>) -> Self {
        self.planner_candidates = Some(value);
        self
    }

    pub fn policy_epoch(mut self, value: i64) -> Self {
        self.policy_epoch = Some(value);
        self
    }

    pub fn read_epoch(mut self, value: i64) -> Self {
        self.read_epoch = Some(value);
        self
    }

    pub fn returned_count(mut self, value: i64) -> Self {
        self.returned_count = Some(value);
        self
    }

    pub fn scalar_filter_applied(mut self, value: bool) -> Self {
        self.scalar_filter_applied = Some(value);
        self
    }

    pub fn scalar_filter_predicates(mut self, value: Vec<String>) -> Self {
        self.scalar_filter_predicates = Some(value);
        self
    }

    pub fn scalar_filter_removed_records(mut self, value: i64) -> Self {
        self.scalar_filter_removed_records = Some(value);
        self
    }

    pub fn scalar_filter_visible_records(mut self, value: i64) -> Self {
        self.scalar_filter_visible_records = Some(value);
        self
    }

    pub fn schema_epoch(mut self, value: i64) -> Self {
        self.schema_epoch = Some(value);
        self
    }

    pub fn segments_scanned(mut self, value: i64) -> Self {
        self.segments_scanned = Some(value);
        self
    }

    pub fn selected_strategy(mut self, value: impl Into<String>) -> Self {
        self.selected_strategy = Some(value.into());
        self
    }

    pub fn skipped_access_paths(mut self, value: Vec<String>) -> Self {
        self.skipped_access_paths = Some(value);
        self
    }

    pub fn tenant_mask_visible_records(mut self, value: i64) -> Self {
        self.tenant_mask_visible_records = Some(value);
        self
    }

    pub fn text_candidates(mut self, value: i64) -> Self {
        self.text_candidates = Some(value);
        self
    }

    pub fn vector_candidates(mut self, value: i64) -> Self {
        self.vector_candidates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HybridExplain`].
    pub fn build(self) -> Result<HybridExplain, BuildError> {
        Ok(HybridExplain {
            access_path_timings: self.access_path_timings,
            access_paths: self.access_paths,
            candidate_budget: self.candidate_budget,
            deduped_candidate_count: self.deduped_candidate_count,
            dirty_feature_count: self.dirty_feature_count,
            early_stop_reason: self.early_stop_reason,
            exact_fallback_triggered: self.exact_fallback_triggered,
            failed_feature_count: self.failed_feature_count,
            final_visibility_guard_count: self.final_visibility_guard_count,
            final_visibility_guard_removed: self.final_visibility_guard_removed,
            freshness_mode: self.freshness_mode,
            fusion_method: self.fusion_method,
            hot_overlay_searched: self.hot_overlay_searched,
            lexical_cache_hits: self.lexical_cache_hits,
            lexical_cache_misses: self.lexical_cache_misses,
            lexical_indexed_documents: self.lexical_indexed_documents,
            lexical_scored_documents: self.lexical_scored_documents,
            materialized_count: self.materialized_count,
            missing_feature_count: self.missing_feature_count,
            module_versions: self.module_versions,
            opened_candidate_streams: self.opened_candidate_streams,
            pending_feature_count: self.pending_feature_count,
            phase_timings: self.phase_timings,
            planner_candidates: self.planner_candidates,
            policy_epoch: self.policy_epoch,
            read_epoch: self.read_epoch,
            returned_count: self.returned_count,
            scalar_filter_applied: self.scalar_filter_applied,
            scalar_filter_predicates: self.scalar_filter_predicates,
            scalar_filter_removed_records: self.scalar_filter_removed_records,
            scalar_filter_visible_records: self.scalar_filter_visible_records,
            schema_epoch: self.schema_epoch,
            segments_scanned: self.segments_scanned,
            selected_strategy: self.selected_strategy,
            skipped_access_paths: self.skipped_access_paths,
            tenant_mask_visible_records: self.tenant_mask_visible_records,
            text_candidates: self.text_candidates,
            vector_candidates: self.vector_candidates,
        })
    }
}
