use std::io;
use std::io::BufRead;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

fn main() {
    host_to_ip();
}

fn host_to_ip() {
    let input = io::stdin();
    let mut domains: Vec<String> = Vec::new();
    for line in input.lock().lines() {
        domains.push(line.unwrap());
    }

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    for dom in &domains {
        let domain = String::from(dom);
        if resolver
            .lookup_ip(domain)
            .map(|look| look.iter().next().map(|addr| println!("{}|{}", dom, addr)))
            .is_err()
        {
            eprintln!("{}|Nothing found", dom);
        }
    }
}
