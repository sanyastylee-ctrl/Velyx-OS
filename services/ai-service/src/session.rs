use crate::context::SessionContext;
use std::collections::HashMap;

#[derive(Default)]
pub struct SessionStore {
    sessions: HashMap<String, SessionContext>,
}

impl SessionStore {
    pub fn insert(&mut self, session: SessionContext) {
        self.sessions.insert(session.session_id.clone(), session);
    }

    pub fn get(&self, session_id: &str) -> Option<SessionContext> {
        self.sessions.get(session_id).cloned()
    }
}
