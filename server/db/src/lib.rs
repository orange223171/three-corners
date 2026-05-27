use tokio_postgres::{Client, Error, NoTls};

pub struct Db {
    postresql_client: Client,
}

impl Db {
    /// Inits database connection
    pub async fn init() -> Result<Self, Error> {
        let (client, connection) =
            tokio_postgres::connect("postgresql://:@localhost:5432/three_corners", NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Result::Ok(Self {
            postresql_client: client,
        })
    }

    /// Returns user's password hash
    pub async fn get_hash(&mut self, user: String) -> Result<String, Error> {
        for row in self
            .postresql_client
            .query(
                "SELECT password_hash FROM users WHERE user_name = $1",
                &[&user],
            )
            .await?
        {
            return Result::Ok(row.get(0));
        }

        Result::Ok(String::new())
    }

    /// Adds user into database
    pub async fn add_user(&mut self, user: String, hash: String) -> Result<(), Error> {
        self.postresql_client
            .execute(
                "INSERT INTO users (user_name, password_hash) VALUES ($1, $2)",
                &[&user, &hash],
            )
            .await?;

        Result::Ok(())
    }
}
