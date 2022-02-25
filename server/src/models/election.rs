use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Election {
    id: String,
    created_at: DateTime<Utc>,
    voting_start_date: DateTime<Utc>,
    voting_end_date: DateTime<Utc>,
    sign_up_start_date: DateTime<Utc>,
    sign_up_end_date: DateTime<Utc>,
}

impl Election {
    pub fn new(sign_duration_minutes: i64, voting_duration_minutes: i64) -> Self {
        let now = Utc::now();

        Election {
            id: Uuid::new_v4().to_string(),
            created_at: now,
            sign_up_start_date: now,
            voting_start_date: now,
            sign_up_end_date: now + Duration::minutes(sign_duration_minutes),
            voting_end_date: now + Duration::minutes(voting_duration_minutes),
        }
    }
}
