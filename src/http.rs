pub struct HttpResponse {
    pub status_code: HttpStatus,
    pub response: String,
}

pub enum HttpStatus {
    Ok = 200,
    BadRequest = 400,
    Forbidden = 403,
    InternalServerError = 500,
}
