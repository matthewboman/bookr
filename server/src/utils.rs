use chrono::{Duration, Utc};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sqlx::types::chrono::NaiveDateTime;

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

pub fn is_token_expired(created_at: NaiveDateTime, expiration_duration: Duration) -> bool {
    let current_time = Utc::now().naive_utc();

    // Token shouldn't be from the future
    if created_at > current_time {
        return false;
    }

    created_at + expiration_duration <= current_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_is_not_expired() {
        let current_time        = Utc::now().naive_utc();
        let expiration_duration = Duration::minutes(30);
        let created_at          = current_time - Duration::minutes(25);

        assert_eq!(is_token_expired(created_at, expiration_duration), false);
    }

    #[test]
    fn test_token_is_expired() {
        let current_time        = Utc::now().naive_utc();
        let expiration_duration = Duration::minutes(30);
        let created_at          = current_time - Duration::minutes(35);

        assert_eq!(is_token_expired(created_at, expiration_duration), true);
    }
}