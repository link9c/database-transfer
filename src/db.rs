use ini::Ini;
use mysql_async::{
    prelude::{FromRow, FromValue, Queryable},
    Conn, Opts, Pool,
};
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
#[derive(Debug, Clone, Copy)]
pub enum Direct {
    FROM,
    TO,
}

impl Default for Direct {
    fn default() -> Self {
        Direct::FROM
    }
}

impl Direct {
    pub fn toggle(self) -> Self {
        match self {
            Direct::FROM => Self::TO,
            Direct::TO => Self::FROM,
        }
    }
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
#[derive(Debug, Clone, Default)]
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
                //tcp.set_nodelay(true)?;

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
                drop(c);
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

    pub async fn show_tables(
        self,
        direct: Direct,
        ddb: String,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let client = self.client(direct).await?;
        let res = match client {
            SQLClient::Mysql((mut c, p)) => {
                let sql_str = format!(
                    "select table_name from information_schema.tables where table_schema='{}' and table_type='base table';"
                    ,ddb
                );
                let mut result = c.query_iter(sql_str).await?;
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

    pub async fn table_detail(
        self,
        direct: Direct,
        ddb: String,
        table: String,
    ) -> Result<Vec<(String, String, String, String)>, Box<dyn std::error::Error>> {
        let client = self.client(direct).await?;
        let res = match client {
            SQLClient::Mysql((mut c, p)) => {
                let sql_str = format!(
                    "SELECT
                    COLUMN_NAME,
                    
                    DATA_TYPE,
                    CHARACTER_MAXIMUM_LENGTH,
                    COLUMN_COMMENT
                FROM
                    information_schema.`COLUMNS` 
                WHERE
                    TABLE_SCHEMA = '{}' 
                    AND table_name = '{}' 
                ORDER BY
                    TABLE_NAME,
                    ORDINAL_POSITION;",
                    ddb, table
                );
                println!("{}", sql_str);
                let mut result = c.query_iter(sql_str).await?;
                let res = result.collect::<(String, String, String, String)>().await?;
                drop(c);

                p.disconnect().await?;
                res
            }
            SQLClient::Mssql(mut c) => {
                let sql_str = format!(
                    "SELECT
                a.name AS COLUMN_NAME,
                --isnull( e.text, '' ) AS COLUMN_DEFAULT,
                b.name AS DATA_TYPE,
                COLUMNPROPERTY( a.id, a.name, 'PRECISION' ) AS CHARACTER_MAXIMUM_LENGTH,
                isnull( g.[value], '' ) AS COLUMN_COMMENT 
            FROM
                syscolumns a
                LEFT JOIN systypes b ON a.xtype= b.xusertype
                INNER JOIN sysobjects d ON a.id= d.id 
                AND d.xtype= 'U' 
                AND d.name<> 'dtproperties'
                LEFT JOIN syscomments e ON a.cdefault= e.id
                LEFT JOIN sys.extended_properties g ON a.id= g.major_id 
                AND a.colid= g.minor_id
                LEFT JOIN sys.extended_properties f ON d.id= f.major_id 
                AND f.minor_id = 0 
            WHERE
                d.name= '{}' 
            ORDER BY
                a.id,
                a.colorder",
                    table
                );
                println!("{}", sql_str);
                let row = c.simple_query(sql_str).await?.into_results().await?;

                let res = row[0]
                    .iter()
                    .map(|x| {
                        let r1: &str = x.get(0).unwrap();
                        let r2: &str = x.get(0).unwrap();
                        let r3: &str = x.get(0).unwrap();
                        let r4: &str = x.get(0).unwrap();
                        (
                            r1.to_string(),
                            r2.to_string(),
                            r3.to_string(),
                            r4.to_string(),
                        )
                    })
                    .collect::<Vec<(String, String, String, String)>>();

                res
            }
        };
        Ok(res)
    }
}
