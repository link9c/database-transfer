use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

extern crate ini;
use ini::Ini;

#[derive(Debug)]
struct DatabaseConfig {
    class: String,
    host: String,
    port: String,
    user: String,
    password: String,
}

impl DatabaseConfig {
    pub fn from_config(class: &str) -> Self {
        let conf = Ini::load_from_file("conf.ini").unwrap();
        let inf = conf.section(Some(class)).unwrap();
        Self {
            class: inf.get("databaseType").unwrap().to_string(),
            host: inf.get("host").unwrap().to_string(),
            port: inf.get("port").unwrap().to_string(),
            user: inf.get("user").unwrap().to_string(),
            password: inf.get("password").unwrap().to_string(),
        }
    }
    pub fn read_table(){

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
            from_db: DatabaseConfig::from_config("FROM"),
            to_db: DatabaseConfig::from_config("TO"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_meta = DatabaseMeta::initial();
    println!("{:?}", db_meta);
    let mut config = Config::new();

    config.host("10.6.1.170");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("datateam_developer", "MP-it226"));
    config.trust_cert();

    println!("{:?}", config.get_addr());

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = match Client::connect(config, tcp.compat_write()).await {
        // Connection successful.
        Ok(client) => client,
        // The server wants us to redirect to a different address
        Err(e) => Err(e)?,
    };

    let row = client
        .query("SELECT top 1 * from mds.mdm.Master_Employee_View", &[&-4i32])
        .await?
        .into_row()
        .await?
        .unwrap();

    println!("{:?}", row);

    Ok(())
}
