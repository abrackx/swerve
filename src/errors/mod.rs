// use actix_web::{http::StatusCode, HttpResponse};
// use actix_web::web::Json;
//
// pub struct ServiceError {
//     pub http_status: StatusCode,
//     pub message: String,
// }
//
// impl ServiceError {
//     pub fn new(http_status: StatusCode, message: String) -> ServiceError {
//         ServiceError {
//             http_status,
//             message,
//         }
//     }
//
//     pub fn response(&self) -> HttpResponse {
//         HttpResponse::build(self.http_status).json(&self.body)
//     }
// }