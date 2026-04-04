#[derive(Debug)]
pub enum AiServiceError {
    BackendUnavailable,
    PolicyBlocked(String),
    ToolNotFound(String),
    InvalidRequest(String),
    AuditFailure(String),
}

impl AiServiceError {
    pub fn message(&self) -> String {
        match self {
            Self::BackendUnavailable => "AI backend недоступен".to_string(),
            Self::PolicyBlocked(reason) => format!("Действие заблокировано policy guard: {reason}"),
            Self::ToolNotFound(tool) => format!("Инструмент не найден: {tool}"),
            Self::InvalidRequest(reason) => format!("Некорректный AI запрос: {reason}"),
            Self::AuditFailure(reason) => format!("Ошибка AI audit: {reason}"),
        }
    }
}
