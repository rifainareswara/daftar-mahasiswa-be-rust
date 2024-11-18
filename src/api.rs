use actix_web::{delete, get, post, web, HttpResponse, Responder};
use bson::{doc, oid::ObjectId};
use futures::TryStreamExt;
use mongodb::Database;
use crate::{error::ApiError, models::{CreateStudentDto, Student}};
use serde_json::json;

const COLLECTION: &str = "students";

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_student)
            .service(get_all_students)
            .service(get_student)
            .service(delete_student),
    );
}

#[post("/students")]
async fn create_student(
    db: web::Data<Database>,
    student: web::Json<CreateStudentDto>,
) -> Result<impl Responder, ApiError> {
    let collection = db.collection::<Student>(COLLECTION);

    let student = Student {
        id: None,
        nama: student.nama.clone(),
        nim: student.nim.clone(),
        jurusan: student.jurusan.clone(),
    };

    let result = collection.insert_one(student, None).await?;
    let id = result.inserted_id.as_object_id().unwrap();

    let created_student = collection
        .find_one(doc! { "_id": id }, None)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(HttpResponse::Created().json(created_student))
}

#[get("/students")]
async fn get_all_students(db: web::Data<Database>) -> Result<impl Responder, ApiError> {
    let collection = db.collection::<Student>(COLLECTION);
    let mut cursor = collection.find(None, None).await?;

    let mut students = Vec::new();
    while let Some(student) = cursor.try_next().await? {
        students.push(student);
    }

    Ok(HttpResponse::Ok().json(students))
}

#[get("/students/{id}")]
async fn get_student(
    db: web::Data<Database>,
    id: web::Path<ObjectId>, // Change the type from String to ObjectId
) -> Result<impl Responder, ApiError> {
    let object_id = id.into_inner(); // Extract the ObjectId from the Path
    let collection = db.collection::<Student>(COLLECTION);

    let student = collection
        .find_one(doc! { "_id": object_id }, None)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(HttpResponse::Ok().json(student))
}

#[delete("/students/{id}")]
async fn delete_student(
    db: web::Data<Database>,
    id: web::Path<ObjectId>,
) -> Result<impl Responder, ApiError> {
    let object_id = id.into_inner();
    let collection = db.collection::<Student>(COLLECTION);

    let result = collection
        .delete_one(doc! { "_id": object_id }, None)
        .await?;

    if result.deleted_count == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(HttpResponse::Ok().json(json!({ "message": "Student deleted successfully" })))
}