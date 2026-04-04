use crate::model::{CheckResult, PolicyDecisionSource, TrustLevel};

pub struct PolicyContext {
    pub app_id: String,
    pub sender: String,
    pub trust_level: TrustLevel,
    pub sender_mismatch: bool,
}

pub struct PolicyDecision {
    pub result: CheckResult,
    pub source: PolicyDecisionSource,
}

pub fn trust_level_for_app(app_id: &str) -> TrustLevel {
    if app_id.starts_with("com.velyx.") {
        TrustLevel::System
    } else {
        TrustLevel::Unknown
    }
}

pub fn evaluate(store_result: CheckResult, context: &PolicyContext) -> PolicyDecision {
    if context.sender_mismatch {
        return PolicyDecision {
            result: CheckResult::Prompt,
            source: PolicyDecisionSource::Policy,
        };
    }

    match store_result {
        CheckResult::Deny => PolicyDecision {
            result: CheckResult::Deny,
            source: PolicyDecisionSource::Store,
        },
        CheckResult::Allow => PolicyDecision {
            result: CheckResult::Allow,
            source: PolicyDecisionSource::Store,
        },
        CheckResult::Prompt => match context.trust_level {
            TrustLevel::System => PolicyDecision {
                result: CheckResult::Prompt,
                source: PolicyDecisionSource::Default,
            },
            TrustLevel::Trusted => PolicyDecision {
                result: CheckResult::Prompt,
                source: PolicyDecisionSource::Default,
            },
            TrustLevel::Unknown => PolicyDecision {
                result: CheckResult::Prompt,
                source: PolicyDecisionSource::Default,
            },
        },
    }
}
