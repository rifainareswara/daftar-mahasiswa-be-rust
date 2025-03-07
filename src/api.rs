use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use bson::{doc, oid::ObjectId};
use futures::TryStreamExt;
use mongodb::Database;
use crate::{error::ApiError, models::{CreateStudentDto, Student, UpdateStudentDto}};
use serde_json::json;

const COLLECTION: &str = "students";

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_student)
            .service(get_all_students)
            .service(get_student)
            .service(update_student)
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

    let result = collection.insert_one(student).await?;
    let id = result.inserted_id.as_object_id().unwrap();

    let created_student = collection
        .find_one(doc! { "_id": id })
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(HttpResponse::Created().json(created_student))
}

#[get("/students")]
async fn get_all_students(db: web::Data<Database>) -> Result<impl Responder, ApiError> {
    let collection = db.collection::<Student>(COLLECTION);
    let mut cursor = collection.find(doc! {}).await?;

    let mut students = Vec::new();
    while let Some(student) = cursor.try_next().await? {
        students.push(student);
    }

    Ok(HttpResponse::Ok().json(students))
}

#[get("/students/{id}")]
async fn get_student(
    db: web::Data<Database>,
    id: web::Path<String>, // Change the type from String to ObjectId
) -> Result<impl Responder, ApiError> {
    let object_id = ObjectId::parse_str(id.as_str()).expect("Not obj id");
    let collection = db.collection::<Student>(COLLECTION);

    let student = collection
        .find_one(doc! { "_id": object_id })
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(HttpResponse::Ok().json(student))
}

#[put("/students/{id}")]
async fn update_student(
    db: web::Data<Database>,
    id: web::Path<String>,
    student: web::Json<UpdateStudentDto>,
) -> Result<impl Responder, ApiError> {
    let object_id = ObjectId::parse_str(id.as_str()).expect("Not obj id");
    let collection = db.collection::<Student>(COLLECTION);

    // Create update document with only the fields that are provided
    let mut update_doc = doc! {};

    if let Some(nama) = &student.nama {
        update_doc.insert("nama", nama);
    }

    if let Some(nim) = &student.nim {
        update_doc.insert("nim", nim);
    }

    if let Some(jurusan) = &student.jurusan {
        update_doc.insert("jurusan", jurusan);
    }

    // If no fields to update were provided
    if update_doc.is_empty() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "message": "No fields to update were provided"
        })));
    }

    let result = collection
        .update_one(
            doc! { "_id": object_id },
            doc! { "$set": update_doc },
        )
        .await?;

    if result.matched_count == 0 {
        return Err(ApiError::NotFound);
    }

    // Get the updated student to return
    let updated_student = collection
        .find_one(doc! { "_id": object_id })
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(HttpResponse::Ok().json(updated_student))
}

#[delete("/students/{id}")]
async fn delete_student(
    db: web::Data<Database>,
    id: web::Path<String>,
) -> Result<impl Responder, ApiError> {
    let object_id = ObjectId::parse_str(id.as_str()).expect("Not obj id");
    let collection = db.collection::<Student>(COLLECTION);

    let result = collection
        .delete_one(doc! { "_id": object_id })
        .await?;

    if result.deleted_count == 0 {
        return Err(ApiError::NotFound);
    }

    Ok(HttpResponse::Ok().json(json!({ "message": "Student deleted successfully" })))
}