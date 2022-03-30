use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use mysql_async::{Conn, Opts, Pool,prelude::{Queryable,FromRow,FromValue}};
extern crate ini;
use ini::Ini;

enum Direct {
    FROM,
    TO,
}

enum SQLClient {
    Mysql((Conn,Pool)),
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
#[derive(Debug)]
struct DatabaseConfig {
    class: String,
    host: String,
    port: String,
    user: String,
    password: String,
    default_db: String,
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
#[derive(Debug)]
struct DatabaseMeta {
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
                Ok(SQLClient::Mysql((client,pool)))
            }
            
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_meta = DatabaseMeta::initial();
    println!("{:?}", db_meta);
    let client = db_meta.client(Direct::TO).await?;

        match client{
            SQLClient::Mysql((mut c,p)) => {
                let mut result = c.query_iter("SELECT id,action,uid from error_log limit 10").await?;
                let res = result.collect::<(String,String,String)>().await?;
                drop(c);
                p.disconnect().await?;
                println!("{:?}", res);

        },
            SQLClient::Mssql(mut c) => {
                let row = c
        .query(
            "SELECT top 1 * from mds.mdm.Master_Employee_View",
            &[&-4i32],
        )
        .await?
        .into_row()
        .await?
        .unwrap();

    println!("{:?}", row);
            },
        }
    

    Ok(())
}
