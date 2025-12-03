use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct WsRoom {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub max_users: Option<usize>,
    pub is_private: bool,
    pub created_by: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl WsRoom {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description: None,
            max_users: None,
            is_private: false,
            created_by: None,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn private(mut self) -> Self {
        self.is_private = true;
        self
    }

    pub fn with_max_users(mut self, max_users: usize) -> Self {
        self.max_users = Some(max_users);
        self
    }

    pub fn created_by(mut self, user_id: i64) -> Self {
        self.created_by = Some(user_id);
        self
    }
}

