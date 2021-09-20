use tide::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Beast {
    pub first_name: String,
    pub last_name: String,
    pub titles: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub name: String,
    pub semester: String,
    pub year: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Opinion {
    pub beast: Beast,
    pub course: Course,
    pub opinion: String,
}
