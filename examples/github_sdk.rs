use async_std::task;
use dialoguer::{Input, PasswordInput};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use surf::Client;
use surf::Exception;
pub struct Sdk {
    _credentials: VerifiedCredentials,
}

#[derive(Deserialize, Serialize)]
struct StoredCredentials {
    api_token: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct VerifiedCredentials {
    api_token: String,
}

impl Sdk {
    ///
    /// # Example
    /// ```
    /// let sdk = Sdk::login(async || {
    ///     let username = Input::<String>::new().with_prompt("Your name").interact()?;
    ///     let password = PasswordInput::new().with_prompt("New Password")
    ///                     .interact()?;
    ///     Ok((username, password))
    /// }).await?;
    ///
    ///
    /// ```
    pub async fn login(login_fn: impl Fn() -> (String, String)) -> Result<Self, Exception> {
        // Try and get the credentials locally
        let base_dir =
            BaseDirs::new().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))?;
        let file = base_dir.data_dir().join("my_sdk/store.json");
        let string = async_std::fs::read_to_string(file).await?;
        let creds: StoredCredentials = serde_json::from_str(&string)?;

        // Either we had the credentials locally, or we should
        // get them from the web
        let valid_creds = Sdk::test_existing_creds(creds, login_fn).await?;
        Sdk::store_creds(valid_creds.clone()).await?;
        Ok(Self {
            _credentials: valid_creds,
        })
    }

    /// method to test the credentials we saved on previous runs.
    async fn test_existing_creds(
        creds: StoredCredentials,
        login_fn: impl Fn() -> (String, String),
    ) -> Result<VerifiedCredentials, Exception> {
        match Sdk::test_creds(creds).await {
            Ok(valid_creds) => Ok(valid_creds),
            Err(_) => {
                println!("Invalid credentials, please try again");
                Ok(Sdk::get_creds(login_fn).await?)
            }
        }
    }

    async fn test_creds(creds: StoredCredentials) -> Result<VerifiedCredentials, Exception> {
        let client = Client::new();
        client
            .get("https://api.github.com/user")
            .set_header("Authorization", format!("Token {}", creds.api_token))
            .recv_json::<VerifiedCredentials>()
            .await
    }

    async fn get_creds(
        login_fn: impl Fn() -> (String, String),
    ) -> Result<VerifiedCredentials, Exception> {
        let (username, password) = login_fn();
        let client = Client::new();
        let based64_encoded = base64::encode(&(format!("{}:{}", username, password)));
        let possible_api_token = client
            .get("https://api.github.com/user")
            .set_header("Authorization", format!("Basic {}", based64_encoded))
            .recv_json::<VerifiedCredentials>()
            .await;
        match possible_api_token {
            Ok(api_token) => Ok(api_token),
            Err(e) => {
                println!("Invalid credentials");
                Err(e)
            }
        }
    }

    async fn store_creds(creds: VerifiedCredentials) -> Result<(), Exception> {
        let base_dir =
            BaseDirs::new().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))?;
        let dir = base_dir.data_dir().join("my_sdk");
        let file = base_dir.data_dir().join("my_sdk/store.json");
        fs::create_dir_all(dir)?;
        let buf = serde_json::to_string(&creds)?;
        async_std::fs::write(file, buf.as_bytes()).await?;
        Ok(())
    }
}

fn main() -> Result<(), Exception> {
    task::block_on(async {
        Sdk::login(|| {
            let username = Input::<String>::new()
                .with_prompt("Your name")
                .interact()
                .unwrap();
            let password = PasswordInput::new()
                .with_prompt("New Password")
                .interact()
                .unwrap();
            (username, password)
        })
        .await?;

        Ok::<(), surf::Exception>(())
    })
}
