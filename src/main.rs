use anyhow::Result;
use clap::Parser;
use std::env::args;
use std::io::{self, BufRead};
use std::net::IpAddr;
use tokio::runtime::Runtime;
use trust_dns_resolver::config::*;
use trust_dns_resolver::TokioAsyncResolver;

#[derive(Debug, Default, Clone)]
struct Host {
    domain: String,
    ip: Option<IpAddr>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

fn main() -> Result<()> {
    let _args = Args::parse();
    let resolve_result = Runtime::new()?.block_on(host_to_ip())?;

    for host in resolve_result {
        if let Some(ip) = host.ip {
            println!("{}, {:?}", host.domain, ip);
        }
    }

    Ok(())
}

async fn host_to_ip() -> Result<Vec<Host>> {
    let input = io::stdin();
    let mut domains: Vec<String> = Vec::new();
    for line in input.lock().lines() {
        domains.push(line?);
    }

    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())
        .expect("Unable to connect resolver");

    let mut hosts: Vec<Host> = Vec::new();
    for dom in domains {
        let mut host = Host {
            domain: dom.to_string(),
            ip: None,
        };
        if !dom.is_empty() {
            let mut domain = dom;
            domain.push('.');

            if let Ok(lookup_ip) = resolver.lookup_ip(domain.clone()).await {
                host.ip = lookup_ip.into_iter().next();
            }
        }
        hosts.push(host);
    }

    Ok(hosts)
}
