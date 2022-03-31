use gui::ui::render_window;
use mysql_async::{
    prelude::{FromRow, FromValue, Queryable},
    Conn, Opts, Pool,
};
use tiberius::{AuthMethod, Client, Config, Row};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

mod db;
mod gui;

#[tokio::main]
async fn main()  {
    render_window();

    
}
