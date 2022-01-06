# HTTP DNS and Loadbalancer Check

A simple utility that will query the ip addresses for a given URL, and will attempt to make that query on each of the IP addresses.  

This is very useful when debugging a bad server or load balancer.

Example:

```
# check the availability of localhost
cargo run -- http://localhost:3000/

# check the availability of https://thebomb.com/totallyawesome
cargo run -- https://thebomb.com/totallyawesome
```


