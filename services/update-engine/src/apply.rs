use crate::model::{AppliedMarker, UpdateAttempt, UpdatePackage};
use chrono::Utc;
use std::path::Path;

pub fn simulate_apply(
    base_dir: &Path,
    package: &UpdatePackage,
    attempt: &UpdateAttempt,
) -> Result<AppliedMarker, String> {
    let marker = AppliedMarker {
        update_id: package.update_id.clone(),
        attempt_id: attempt.attempt_id.clone(),
        snapshot_id: attempt.snapshot_id.clone(),
        payload_kind: package.payload_kind.clone(),
        simulated: true,
        applied_at: Utc::now().to_rfc3339(),
    };
    let marker_path = base_dir.join("update_apply_marker.json");
    let tmp = marker_path.with_extension("json.tmp");
    let raw = serde_json::to_string_pretty(&marker)
        .map_err(|err| format!("apply marker serialization failed: {err}"))?;
    std::fs::write(&tmp, raw).map_err(|err| format!("apply marker temp write failed: {err}"))?;
    std::fs::rename(&tmp, &marker_path)
        .map_err(|err| format!("apply marker rename failed: {err}"))?;
    Ok(marker)
}
