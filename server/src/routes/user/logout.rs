use actix_web::HttpResponse;

use crate::session_state::TypedSession;
use crate::utils::e500;

pub async fn log_out(session: TypedSession) -> Result<HttpResponse, actix_web::Error> {
    if session.get_user_id().map_err(e500)?.is_none() {
        Ok(HttpResponse::Ok().finish())
    } else {
        session.logout();
        Ok(HttpResponse::Ok().finish())
    }
}