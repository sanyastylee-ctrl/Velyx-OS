use crate::model::{AppliedMarker, SignatureCheckResult, UpdateAttempt, UpdatePackage};

pub fn verify_signature(package: &UpdatePackage) -> Result<SignatureCheckResult, String> {
    if package.signed {
        Ok(SignatureCheckResult {
            valid: true,
            reason: "signed package accepted".to_string(),
        })
    } else {
        Ok(SignatureCheckResult {
            valid: false,
            reason: format!(
                "update {} не подписан и не может быть применен",
                package.update_id
            ),
        })
    }
}

pub fn verify_post_apply(
    package: &UpdatePackage,
    attempt: &UpdateAttempt,
    marker: Option<AppliedMarker>,
) -> Result<SignatureCheckResult, String> {
    let marker = marker.ok_or_else(|| "post-apply marker is missing".to_string())?;
    if marker.update_id != package.update_id || marker.attempt_id != attempt.attempt_id {
        return Ok(SignatureCheckResult {
            valid: false,
            reason: "post-apply marker mismatch".to_string(),
        });
    }
    if package.update_id.contains("post-verify-fail") {
        return Ok(SignatureCheckResult {
            valid: false,
            reason: "simulated post-apply verification failed".to_string(),
        });
    }
    Ok(SignatureCheckResult {
        valid: true,
        reason: "post-apply verification passed".to_string(),
    })
}
