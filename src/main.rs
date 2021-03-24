use std::io;
use std::io::BufRead;
use std::thread;
use std::time::Duration;
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

    let resolver = Resolver::from_system_conf().unwrap();
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
                    eprintln!("{}|Nothing found", dom);
                }

                thread::sleep(Duration::from_millis(8));
            }
        }
    });

    handle.join().unwrap();
}
