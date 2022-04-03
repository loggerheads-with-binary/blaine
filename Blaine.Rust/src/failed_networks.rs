extern crate interfaces;
use interfaces::Interface;

pub fn check_interfaces(interface_list: &[String]) -> () {
    for interface in interface_list {
        let xx = Interface::get_by_name(interface);

        if xx.is_none() {
            eprintln!("Interface {} is erroneous", interface);
            return;
        }

        if !xx.unwrap().is_up() {
            let mut msg = "Network Interface [".to_string();
            msg.push_str(&interface);
            msg.push_str("] is not running");
            Command::new("notif")
                .args(["-t", "Blaine Network Monitor", "-m", &msg, "-d", "10"])
                .spawn()
                .expect("Notif call failed");
        }
    }
}

fn main() -> () {
    let interface_list: [String; 1] = ["Ethernet 5".to_string()];

    loop {
        check_interfaces(interfaces_list);
        print("Cycle Complete; Sleeping Now");
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
