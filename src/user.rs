use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub birth_date: NaiveDate,
    pub custom_data: CustomData,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(name: String, birth_date_ymd: (i32, u32, u32)) -> Self {
        let (y, m, d) = birth_date_ymd;
        let id = uuid::Uuid::parse_str("b916577c-2c51-4025-891f-13b0e27b8049")
            .unwrap();
        Self {
            id: id,
            // id: uuid::Uuid::new_v4(),
            name: name,
            birth_date: NaiveDate::from_ymd_opt(y, m, d).unwrap(),
            custom_data: CustomData { random: 1 },
            created_at: Some(Utc::now()),
            updated_at: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomData {
    pub random: u32,
}
