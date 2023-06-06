use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum UserError {
    #[display(fmt = "internal error")]
    InternalError,
    #[display(fmt = "bad request")]
    NotFoundById,
}

impl ResponseError for UserError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::NotFoundById => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
