use reqwest::Client;
use secrecy::{ExposeSecret, Secret};

use crate::domain::UserEmail;

pub struct EmailClient {
    http_client: Client,
    base_url:    String,
    sender:      UserEmail,
    auth_token:  Secret<String>,
}

impl EmailClient {
    pub fn new(
        base_url:   String,
        sender:     UserEmail,
        auth_token: Secret<String>,
        timeout:    std::time::Duration
    ) -> EmailClient {
        let http_client = Client::builder()
            .timeout(timeout)
            .build()
            .unwrap();
        Self {
            http_client,
            base_url,
            sender,
            auth_token,
        }
    }

    pub async fn send_email(
        &self,
        recipient: &UserEmail,
        subject:   &str,
        html_body: &str,
        text_body: &str,
    ) -> Result<(), reqwest::Error> {
        let url  = format!("{}/email", self.base_url);
        let body = SendEmailRequest {
            from:      self.sender.as_ref(),
            to:        recipient.as_ref(),
            subject:   subject,
            html_body: html_body,
            text_body: text_body
        };
        self.http_client
            .post(&url)
            .header(
                "X-Postmark-Server-Token",
                self.auth_token.expose_secret()
            )
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<'a> {
    from:      &'a str,
    to:        &'a str,
    subject:   &'a str,
    html_body: &'a str,
    text_body: &'a str,
}

#[cfg(test)]
mod tests {
    use claims::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use secrecy::Secret;
    use wiremock::matchers::{any, header, header_exists, method, path};
    use wiremock::{Mock, MockServer, Request, ResponseTemplate};

    use crate::domain::UserEmail;
    use crate::email_client::EmailClient;

    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &Request) -> bool {
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
            if let Ok(body) = result { 

                body.get("From").is_some()
                    && body.get("To").is_some()
                    && body.get("From").is_some()
                    && body.get("Subject").is_some()
                    && body.get("HtmlBody").is_some()
                    && body.get("TextBody").is_some()
            } else {
                false
            }
        }
    }

    #[tokio::test]
    async fn send_email_sends_the_expected_request() {
        let mock_server  = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1) // match only 1 condition
            .mount(&mock_server)
            .await;
        
        let _ = email_client
            .send_email(&email(), &subject(), &content(), &content())
            .await;
    }

    #[tokio::test]
    async fn send_email_succeeds_if_the_server_returns_200() {
        let mock_server  = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1) // match only 1 condition
            .mount(&mock_server)
            .await;
        
        let outcome = email_client
            .send_email(&email(), &subject(), &content(), &content())
            .await;

        assert_ok!(outcome);
    }

    #[tokio::test]
    async fn send_email_fails_if_the_server_returns_500() {
        let mock_server  = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1) // match only 1 condition
            .mount(&mock_server)
            .await;
        
        let outcome = email_client
            .send_email(&email(), &subject(), &content(), &content())
            .await;

        assert_err!(outcome);
    }

    #[tokio::test]
    async fn send_email_times_out_if_the_server_takes_too_long() {
        let mock_server  = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        let response = ResponseTemplate::new(200)
            .set_delay(std::time::Duration::from_secs(180));

        Mock::given(any())
            .respond_with(response)
            .expect(1) // match only 1 condition
            .mount(&mock_server)
            .await;
        
        let outcome = email_client
            .send_email(&email(), &subject(), &content(), &content())
            .await;

        assert_err!(outcome);
    }

    fn subject() -> String {
        Sentence(1..2).fake()
    }

    fn content() -> String {
        Paragraph(1..2).fake()
    }

    fn email() -> UserEmail {
        UserEmail::parse(SafeEmail().fake()).unwrap()
    }

    fn email_client(base_url: String) -> EmailClient {
        EmailClient::new(
            base_url, 
            email(), 
            Secret::new(Faker.fake()),
            std::time::Duration::from_millis(200)
        )
    }
}