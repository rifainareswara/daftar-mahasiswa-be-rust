use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub nama: String,
    pub nim: String,
    pub jurusan: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateStudentDto {
    pub nama: String,
    pub nim: String,
    pub jurusan: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStudentDto {
    pub nama: Option<String>,
    pub nim: Option<String>,
    pub jurusan: Option<String>,
}