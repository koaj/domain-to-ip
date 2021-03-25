use std::io::BufRead;
use std::thread;
use std::time::Duration;
use std::{env::args, io};
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
    // Capturing all arguments.
    let all_args: Vec<String> = args().collect();
    // Check for configuration in args.
    // On Unix systems by this app use /etc/resolv.conf as resolver.
    let mut resolver = Resolver::from_system_conf().unwrap();
    if all_args.contains(&"secure".to_string()) {
        // Using cloudflare 1.1.1.1 DNS service if there were `secure` keyword in input args.
        resolver =
            Resolver::new(ResolverConfig::cloudflare_tls(), ResolverOpts::default()).unwrap();
    }
    let handle = thread::spawn(move || {
        for dom in &domains {
            if !dom.is_empty() {
                let mut domain = String::from(dom);
                domain.push('.');
                if resolver
                    .lookup_ip(domain)
                    .map(|lookup_ip| lookup_ip.iter().next().map(|ip| println!("{}|{}", dom, ip)))
                    .is_err()
                {
                    // Checking input args for the `debug` keyword to print error.
                    if all_args.contains(&"debug".to_string()) {
                        eprintln!("{}|Nothing found", dom);
                    }
                }

                thread::sleep(Duration::from_millis(1));
            }
        }
    });

    handle.join().unwrap();
}
