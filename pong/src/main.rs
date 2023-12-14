use std::net::TcpListener;

use common::handle_connection;

fn main() {
    println!("pong is running... listening for pings...");

    // Create a daemon
    let mdns = mdns_sd::ServiceDaemon::new().expect("Failed to create daemon");

    // --------------------------------------------------------------------------------
    // announce the existence of this container on the network
    //

    let my_local_ip = local_ip_address::local_ip().unwrap();

    let my_local_ip_as_str = my_local_ip.clone().to_string();
    let my_local_ip_as_str = my_local_ip_as_str.as_str();

    println!("This is my local IP address: {:?}", my_local_ip);

    // Create a service info.
    let service_type = "_example._tcp.local.";
    let instance_name = "pong";
    let ip = my_local_ip;
    let host_name = my_local_ip_as_str;
    let port = 8787;

    let my_service = mdns_sd::ServiceInfo::new(
        service_type,
        instance_name,
        host_name,
        ip,
        port,
        None
    ).unwrap();

    let cloned = my_service.clone();

    // Register with the daemon, which publishes the service.
    mdns.register(my_service).expect("Failed to register our service");

    //
    // --------------------------------------------------------------------------------

    // --------------------------------------------------------------------------------
    // listen for the other container on the network
    //

    // Browse for a service type.
    let service_type = "_example._tcp.local.";
    let receiver = mdns.browse(service_type).expect("Failed to browse");

    //
    // --------------------------------------------------------------------------------

    let localhost = format!("{}:{}", my_local_ip_as_str, port);
    let listener = TcpListener::bind(localhost).unwrap();

    // Receive the browse events in sync or async. Here is
    // an example of using a thread. Users can call `receiver.recv_async().await`
    // if running in async environment.
    // std::thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            match event {
                mdns_sd::ServiceEvent::ServiceResolved(info) => {
                    println!("Resolved a new service: {}", info.get_fullname());

                    if info.get_fullname() != cloned.get_fullname() {
                        for stream in listener.incoming() {
                            let stream = stream.unwrap();
                            let partner_ip_and_port = format!("{}:{}", info.get_hostname().trim_end_matches('.'), info.get_port());
                            handle_connection(stream, "ping", "pong", partner_ip_and_port)
                        }
                    }
                }
                other_event => {
                    // println!("Received other event: {:?}", &other_event);
                    println!("Received other event");
                }
            }
        }
    // });

    // Gracefully shutdown the daemon.
    std::thread::sleep(std::time::Duration::from_secs(1));
    mdns.shutdown().unwrap();
}