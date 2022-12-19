use anyhow::Result;
use clap::Parser;
use std::io::BufRead;
use std::net::IpAddr;
use std::{env::args, io};
use trust_dns_resolver::config::*;
use trust_dns_resolver::TokioAsyncResolver;

#[derive(Debug, Default, Clone)]
struct Host {
    domain: String,
    ip: Option<IpAddr>,
}

/// DNS Resolver to get domain IPs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

#[tokio::main]
async fn main() -> Result<()> {
    let _args = Args::parse();
    let resolve_result = host_to_ip().await?;

    for host in resolve_result {
        if let Some(ip) = host.ip {
            println!("{},{:?}", host.domain, ip)
        }
    }

    Ok(())
}

async fn host_to_ip() -> Result<Vec<Host>> {
    let input = io::stdin();
    let mut domains: Vec<String> = Vec::new();
    for line in input.lock().lines() {
        domains.push(line.unwrap());
    }
    // Capturing all arguments.
    let _all_args: Vec<String> = args().collect();
    // Check for configuration in args.
    // On Unix systems by this app use /etc/resolv.conf as resolver.
    // let mut resolver = Resolver::from_system_conf().expect("Unable to parse /etc/resolve.conf");
    // if all_args.contains(&"secure".to_string()) {
    //     // Using cloudflare 1.1.1.1 DNS service if there were `secure` keyword in input args.
    //     resolver =
    //         Resolver::new(ResolverConfig::cloudflare_tls(), ResolverOpts::default()).unwrap();
    // }

    // Construct a new Resolver with default configuration options
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())
        .expect("Unable to connect resolver");

    // Lookup the IP addresses associated with a name.
    // This returns a future that will lookup the IP addresses, it must be run in the Core to
    //  to get the actual result.

    let mut hosts: Vec<Host> = vec![];
    for dom in domains {
        let mut host = Host {
            domain: dom.to_string(),
            ip: None,
        };
        if !dom.is_empty() {
            let mut domain = dom;
            domain.push('.');

            if let Ok(lookup_ip) = resolver.lookup_ip(domain.clone()).await {
                host.ip = Some(lookup_ip.into_iter().next().unwrap());
            }
        }
        hosts.push(host);
    }

    Ok(hosts)
}
