use ini::Ini;
use mysql_async::{
    prelude::{FromRow, FromValue, Queryable},
    Conn, Opts, Pool,
};
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub enum Direct {
    FROM,
    TO,
}

pub enum SQLClient {
    Mysql((Conn, Pool)),
    Mssql(tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>),
}

impl Direct {
    fn to_str(self) -> &'static str {
        match self {
            Direct::FROM => "FROM",
            Direct::TO => "TO",
        }
    }
}
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    class: String,
    host: String,
    port: String,
    user: String,
    password: String,
    default_db: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            class: Default::default(),
            host: Default::default(),
            port: Default::default(),
            user: Default::default(),
            password: Default::default(),
            default_db: Default::default(),
        }
    }
}

impl DatabaseConfig {
    pub fn from_config(direct: Direct) -> Self {
        let conf = Ini::load_from_file("conf.ini").unwrap();
        let inf = conf.section(Some(direct.to_str())).unwrap();
        Self {
            class: inf.get("databaseType").unwrap().to_string(),
            host: inf.get("host").unwrap().to_string(),
            port: inf.get("port").unwrap().to_string(),
            user: inf.get("user").unwrap().to_string(),
            password: inf.get("password").unwrap().to_string(),
            default_db: inf.get("db").unwrap().to_string(),
        }
    }
}
#[derive(Debug, Clone,Default)]
pub struct DatabaseMeta {
    from_db: DatabaseConfig,
    to_db: DatabaseConfig,
}

impl DatabaseMeta {

    pub fn initial() -> Self {
        Self {
            from_db: DatabaseConfig::from_config(Direct::FROM),
            to_db: DatabaseConfig::from_config(Direct::TO),
        }
    }

    pub async fn client(self, direct: Direct) -> Result<SQLClient, Box<dyn std::error::Error>> {
        let db = match direct {
            Direct::FROM => self.from_db,
            Direct::TO => self.to_db,
        };

        match db.class.as_str() {
            "MSSQL" => {
                let mut config = Config::new();
                config.host(db.host);
                config.port(db.port.parse::<u16>().unwrap());
                config.authentication(AuthMethod::sql_server(db.user, db.password));
                config.trust_cert();

                let tcp = TcpStream::connect(config.get_addr()).await?;
                tcp.set_nodelay(true)?;

                let client = match Client::connect(config, tcp.compat_write()).await {
                    // Connection successful.
                    Ok(client) => client,
                    // The server wants us to redirect to a different address
                    Err(e) => Err(e)?,
                };

                Ok(SQLClient::Mssql(client))
            }
            _ => {
                let database_url = format!(
                    "{class}://{user}:{password}@{host}:{port}/{db}",
                    class = db.class,
                    user = db.user,
                    password = db.password,
                    host = db.host,
                    port = db.port,
                    db = db.default_db
                );

                let opts = Opts::from_url(&database_url).expect("DATABASE_URL invalid");
                let pool = Pool::new(opts);
                let client = pool.get_conn().await?;
                Ok(SQLClient::Mysql((client, pool)))
            }
        }
    }

    pub fn get_default_db(self, direct: Direct) -> String {
        let db = match direct {
            Direct::FROM => self.from_db,
            Direct::TO => self.to_db,
        };
        db.default_db
    }

    pub async fn show_dbs(self, direct: Direct) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let client = self.client(direct).await?;
        let res = match client {
            SQLClient::Mysql((mut c, p)) => {
                let mut result = c.query_iter("show databases").await?;
                let res = result.collect::<String>().await?;
                drop(c);
                p.disconnect().await?;
                res
            }
            SQLClient::Mssql(mut c) => {
                let row = c
                    .simple_query(" SELECT  name FROM SysDatabases")
                    .await?
                    .into_results()
                    .await?;

                let res = row[0]
                    .iter()
                    .map(|x| {
                        let r: &str = x.get(0).unwrap();
                        r.to_string()
                    })
                    .collect::<Vec<String>>();

                res
            }
        };
        Ok(res)
    }
}
