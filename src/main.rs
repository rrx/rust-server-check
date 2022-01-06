use dns_lookup::lookup_host;

const DAYS: i32 = 7;

fn main() -> Result<(), failure::Error> {

    // get urls from the commandline
    let args = std::env::args().collect::<Vec<String>>();
    let (_, urls) = args.split_at(1);

    // iterate over the urls, get the associated IP addresses
    for u in urls.iter() {
        let parts = url::Url::parse(u)?;
        let host = parts.host_str().expect("Invalid Url");

        // check ssl expiration
        match ssl_expiration::SslExpiration::from_domain_name(host) {
            Ok(expiration) => {
                if expiration.is_expired() {
                    return Err(failure::err_msg(format!("{} is expired", host)));
                } else if expiration.days() < DAYS {
                    return Err(failure::err_msg(format!("{} will expire in {} days", host, DAYS)));
                }
            }
            Err(e) => {
                return Err(failure::err_msg(e.to_string()));
            }
        }

        let ips: Vec<std::net::IpAddr> = lookup_host(host)?;
        for ip in ips.iter() {

            // call the URL at the given ip address and print the results
            let client = reqwest::blocking::Client::builder()
                .resolve(host, std::net::SocketAddr::new(*ip, 443))
                .build()?;

            // bail if we have an error
            let r = client.get(u).send()?;

            println!("GET: {}, addr: {:?}, status: {:?}", 
                     u, 
                     r.remote_addr().expect("Invalid Remote Address"),
                     r.status());
        }
    }
    Ok(())
}

