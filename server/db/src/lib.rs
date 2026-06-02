use std::fs;

use tokio_postgres::{Client, Error, NoTls};

pub struct Db {
    postresql_client: Client,
}

impl Db {
    /// Inits database connection
    pub async fn init() -> Result<Self, Error> {
        let mut login = String::new();
        let mut password = String::new();

        let mut i = 0;
        for str in fs::read_to_string("/etc/three_corners/server/db.conf")
            .expect("Error to read database confing")
            .split(char::is_whitespace)
        {
            match i {
                0 => login = str.to_string(),
                1 => password = str.to_string(),
                _ => (),
            }

            i += 1;
        }
        let (client, connection) = tokio_postgres::connect(
            (String::from("postgresql://")
                + login.as_str()
                + ":"
                + password.as_str()
                + "@localhost:5432/three_corners")
                .as_str(),
            NoTls,
        )
        .await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Result::Ok(Self {
            postresql_client: client,
        })
    }

    /// Returns user's password hash, if user doesn't exist returns None
    pub async fn get_hash(&mut self, user: String) -> Result<Option<String>, Error> {
        match self
            .postresql_client
            .query(
                "SELECT password_hash FROM users WHERE user_name = $1",
                &[&user],
            )
            .await?
            .first()
        {
            Some(row) => Result::Ok(row.try_get(0)?),
            None => Result::Ok(None),
        }
    }

    /// Returns totp secret of specified user
    /// # None
    /// - if user doesn't exist returns None
    /// - if user doesn't have 2FA returns None
    pub async fn get_totp_secret(&mut self, user: String) -> Result<Option<String>, Error> {
        match self
            .postresql_client
            .query(
                "SELECT totp_secret FROM users WHERE user_name = $1",
                &[&user],
            )
            .await?
            .first()
        {
            Some(row) => match row.try_get::<_, String>(0) {
                Ok(totp_secret) => Result::Ok(Some(totp_secret.trim().to_string())),
                Err(_) => Result::Ok(None),
            },
            None => Result::Ok(None),
        }
    }

    /// Adds user into database
    pub async fn add_user(&mut self, user: String, hash: String) -> Result<(), Error> {
        self.postresql_client
            .execute(
                "INSERT INTO users (user_name, password_hash, totp_secret) VALUES ($1, $2, NULL)",
                &[&user, &hash],
            )
            .await?;

        Result::Ok(())
    }

    /// Adds 2FA as totp secret's entry
    pub async fn add_2fa(&mut self, user: String, totp_secret: String) -> Result<(), Error> {
        self.postresql_client
            .execute(
                "UPDATE users SET totp_secret = $1 WHERE user_name = $2",
                &[&totp_secret, &user],
            )
            .await?;

        Result::Ok(())
    }

    /// Removes 2FA as totp secret's entry, write NULL
    pub async fn remove_2fa(&mut self, user: String) -> Result<(), Error> {
        self.postresql_client
            .execute(
                "UPDATE users SET totp_secret = NULL WHERE user_name = $1",
                &[&user],
            )
            .await?;

        Result::Ok(())
    }
}
