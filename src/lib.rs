//! A native RethinkDB driver written in Rust

extern crate ql2;
extern crate protobuf;
extern crate serde_json;
#[macro_use]
extern crate derive_error;
#[cfg(feature = "with_io")]
extern crate reql_io;
#[macro_use]
extern crate slog;

#[macro_use]
mod macros;
mod types;
mod commands;
pub mod errors;

#[doc(hidden)]
pub use ql2::proto::Term;

#[cfg(feature = "with_io")]
use std::net::SocketAddr;
#[cfg(feature = "with_io")]
use std::sync::Arc;

use errors::Error;

#[cfg(feature = "with_io")]
use reql_io::r2d2;
#[cfg(feature = "with_io")]
use reql_io::tokio_core::reactor::Remote;
#[cfg(feature = "with_io")]
use reql_io::tokio_core::net::TcpStream;

use slog::Logger;

/// The result of any command that can potentially return an error
pub type Result<T> = ::std::result::Result<T, Error>;

/// The return type of `ToArg::to_arg`
#[derive(Debug, Clone)]
pub struct Arg {
    string: String,
    term: Term,
    pool: Option<Pool>,
}

/// The response returned by the `run` command
#[cfg(feature = "with_io")]
#[derive(Debug, Clone)]
pub struct Response;

/// The connection pool returned by the `connect` command
#[cfg(feature = "with_io")]
#[derive(Debug, Clone)]
pub struct Pool(Vec<DataCentre>);

// The underlying connection to each server
#[cfg(feature = "with_io")]
#[doc(hidden)]
pub struct Connection {
    id: u64,
    broken: bool,
    server: Server,
    stream: TcpStream,
    logger: Logger,
}

#[cfg(feature = "with_io")]
#[derive(Clone)]
struct ConnectionManager {
    server: Server,
    remote: Remote,
    logger: Logger,
}

#[cfg(feature = "with_io")]
#[derive(Debug)]
struct ClusterConfig(Vec<Cluster>);

#[cfg(feature = "with_io")]
#[derive(Debug, Clone)]
struct DataCentre();

#[cfg(feature = "with_io")]
#[derive(Debug, Clone)]
struct Server(Vec<SocketAddr>);

#[cfg(feature = "with_io")]
#[derive(Debug)]
struct Cluster(Vec<Server>);

#[cfg(feature = "with_io")]
#[derive(Debug, Clone)]
struct TlsCfg {
    ca_certs: String,
}

#[cfg(feature = "with_io")]
#[derive(Debug)]
struct Opts {
    db: String,
    user: String,
    password: String,
    retries: u8,
    tls: Option<TlsCfg>,
}

/// The configuration data for the `connect` command
#[cfg(feature = "with_io")]
#[derive(Debug, Clone)]
struct Config {
    pool: Arc<r2d2::Config<Connection, Error>>,
    servers: Vec<Server>,
}

/// The database cluster client
#[must_use]
#[derive(Debug, Clone)]
pub struct Client {
    term: Term,
    query: String,
    logger: Logger,
}

/// The return type of the `args!()` macro
#[derive(Debug, Clone)]
#[doc(hidden)]
pub struct Args {
    term: Term,
    string: String,
    pool: Option<Pool>,
}

/// The argument that is passed to any command
pub trait ToArg {
    fn to_arg(&self) -> Arg;
}
