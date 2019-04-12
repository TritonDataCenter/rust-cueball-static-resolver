/*
 * Copyright 2019 Joyent, Inc.
 */

//! A basic example that demonstrates using the StaticIpResolver for cueball to
//! establish a basic connection pool of TcpStream connections.

use std::net::{IpAddr, Ipv4Addr};
use std::sync::Mutex;

use slog::{Drain, Logger, o};

use cueball::connection_pool::ConnectionPool;
use cueball::connection_pool::types::ConnectionPoolOptions;
use cueball_static_resolver::StaticIpResolver;
use cueball_tcp_stream_connection::TcpStreamWrapper;

fn main() {

    let be1 = (IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 55555);
    let be2 = (IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 55556);
    let be3 = (IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 55557);
    let be4 = (IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 55558);

    let resolver = StaticIpResolver::new(vec![be1, be2, be3, be4]);

    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let log = Logger::root(
        Mutex::new(
            slog_term::FullFormat::new(plain).build()
        ).fuse(),
        o!("build-id" => "0.1.0")
    );

    let pool_opts = ConnectionPoolOptions::<StaticIpResolver> {
        maximum: 5,
        claim_timeout: None,
        resolver: resolver,
        log: log,
        rebalancer_action_delay: None
    };

    let _pool = ConnectionPool::<TcpStreamWrapper, StaticIpResolver>::new(pool_opts);

    println!("Cueball!");

    loop {}
}
