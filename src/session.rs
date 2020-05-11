use crate::*;

const LOGIN_PAGE_URL: &str = "https://wsdmoodle.waseda.jp/auth/saml2/login.php?wants=https%3A%2F%2Fwsdmoodle.waseda.jp%2F&idp=fcc52c5d2e034b1803ea1932ae2678b0&passive=off";
pub struct Session {
    pub client: reqwest::Client,
    pub session_key: String,
}
impl Session {
    pub async fn login(login_id: &str, password: &str) -> Result<Session> {
        let client = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .use_rustls_tls()
            .build()
            .context(ErrorKind::InvalidResponse(Some(
                "cannot create http client".to_owned(),
            )))?;
        let login_page =
            LoginPage::extract_from_str(&client.get(LOGIN_PAGE_URL).send().await?.text().await?)?;
        let login_response = LoginResponse::extract_from_str(
            &client
                .post(
                    reqwest::Url::parse("https://iaidp.ia.waseda.jp/")
                        .unwrap()
                        .join(&login_page.login_info_post_url)
                        .unwrap(),
                )
                .form(&[
                    ("j_username", login_id),
                    ("j_password", password),
                    ("_eventId_proceed", "Login"),
                ])
                .send()
                .await?
                .text()
                .await?,
        )?;
        let moodle_top = MoodleTop::extract_from_str(
            &client
                .post(&login_response.login_continue_url)
                .form(&[
                    ("RelayState", &login_response.relay_state),
                    ("SAMLResponse", &login_response.saml_response),
                ])
                .send()
                .await?
                .text()
                .await?,
        )?;

        println!("{:#?}", moodle_top);

        Ok(Session {
            client,
            session_key: moodle_top.session_key,
        })
    }
}

html_extractor! {
    #[derive(Debug)]
    LoginPage {
        login_info_post_url: String = (attr["action"] of "#login"),
    }
    #[derive(Debug)]
    LoginResponse {
        login_continue_url: String = (attr["action"] of "form"),
        relay_state: String = (attr["value"] of r#"input[name="RelayState"]"#),
        saml_response: String = (attr["value"] of r#"input[name="SAMLResponse"]"#),
    }
    #[derive(Debug)]
    MoodleTop {
        (session_key: String,) = (attr["href"] of r#"[data-title="logout,moodle"]"#, capture with r#"sesskey=(.*)$"#)
    }
}
