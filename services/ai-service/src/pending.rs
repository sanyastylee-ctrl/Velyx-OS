use crate::model::ConfirmationRequest;
use crate::model::ToolExecutionRequest;
use std::collections::HashMap;

#[derive(Clone)]
pub struct PendingAction {
    pub request: ToolExecutionRequest,
    pub confirmation: ConfirmationRequest,
}

#[derive(Default)]
pub struct PendingActionStore {
    actions: HashMap<String, PendingAction>,
}

impl PendingActionStore {
    pub fn insert(&mut self, action: PendingAction) {
        self.actions
            .insert(action.confirmation.action_id.clone(), action);
    }

    pub fn take(&mut self, action_id: &str) -> Option<PendingAction> {
        self.actions.remove(action_id)
    }
}
