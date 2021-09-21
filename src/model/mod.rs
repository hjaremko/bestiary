use tide::prelude::*;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Beast {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub titles: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Course {
    pub id: i64,
    pub name: String,
    pub semester: i64,
    pub year: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Opinion {
    pub id: i64,
    pub beast_id: i64,
    pub course_id: i64,
    pub opinion: Option<String>,
}
