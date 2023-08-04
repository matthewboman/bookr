use actix_web::HttpResponse;
use actix_web::http::header::LOCATION;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn e400<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static
{
    actix_web::error::ErrorBadRequest(e)
}

pub fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static
{
    actix_web::error::ErrorInternalServerError(e)
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;

    let mut current = e.source();

    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }

    Ok(())
}

pub fn generate_token() -> String {
    let mut rng = thread_rng();

    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(25)
        .collect()
}

// TODO: is this useful for REST?
// pub fn see_other(location: &str) -> HttpResponse {
//     HttpResponse::SeeOther()
//         .insert_header((LOCATION, location))
//         .finish()
// }