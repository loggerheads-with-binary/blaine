#![windows_subsystem = "windows"]

extern crate battery;
extern crate fstrings;
extern crate lazy_static;
extern crate interfaces;
extern crate msgbox;


use lazy_static::lazy_static;
use msgbox::IconType;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::mem::drop;
use std::process::Command;
use std::thread;
use std::thread::sleep;
use std::time;
use std::time::Duration;
use std::vec;

//Static Declarations
static mut SLEEP_TIME: Duration = time::Duration::from_millis(30 * 1000); //1 --> ms
static mut HIGH_BATTERY: f64 = 0.85; //2 --> %
static mut LOW_BATTERY: f64 = 0.22; //3 --> %
static mut VERY_LOW_BATTERY: f64 = 0.16; //4 --> %
static mut VERY_HIGH_BATTERY: f64 = 0.91; //5 --> %
static mut TOO_MANY_NOTIFS: u64 = 0;
static NOTIF_UPPER_LIMIT: u64 = 10;

lazy_static! {
    static ref lowargs: Vec<&'static str> = {
        let bxx = vec![

                        "-t" , "Blaine Battery Monitor" ,
                        "-m" , "The Battery is too low and is below critical level of 22%. Please CHARGE it right now ",
                        "-p" , "Blaine" ,
                        "-d" , "10"];

        bxx
    };
    static ref highargs: Vec<&'static str> = {
        let axx = vec![

                        "-t" , "Blaine Battery Monitor" ,
                        "-m" , "The Battery is too hight and is above a decent healthy level of 85%. Please UNPLUG it right now " ,
                        "-p" , "Blaine" ,
                        "-d" , "10"];

        axx
    };
    static ref melissa_args: Vec<&'static str> = {
        let mel = vec!["{\'text\': \"[CriticalAlert] GERTRUDE CHARGE LOW \\n\\nCritical Error 0x0911!!\\nHP Laptop is running out of charge. It\'s at a critical percentage and could die quickly. If you can, just charge it soon????\"}"];

        mel
    };
    static ref msgbox_low_args: Vec<&'static str> = {
        let jisoo = vec![
            "-m",
            "Critically Low Battery Level. Recharge it NOWWWWW!!!!!",
            "-t",
            "Blaine Battery Monitor",
        ];
        jisoo
    };
    static ref msgbox_high_args: Vec<&'static str> = {
        let jisoo = vec![
            "-m",
            "Unhealthily high Battery level. Unplug it NOW!!!!!!!!",
            "-t",
            "Blaine Battery Monitor",
        ];
        jisoo
    };
    static ref soundfile: String = {
        let output = Command::new("c-trans")
            .args(&["{Lois}/Toolkit/Extra/Blaine/Sounds/low.mp3"])
            .output()
            .expect("Chaeyoung Translate Failed");
        let answer = (String::from_utf8_lossy(&output.stdout)).to_string();

        answer
    };
}

pub unsafe fn EvalBattery(charge: f64, status: battery::State) -> () {
    if charge < VERY_LOW_BATTERY {
        if status == battery::State::Discharging {
            if TOO_MANY_NOTIFS > NOTIF_UPPER_LIMIT {
                return;
            }

            Command::new("melissa-alerts")
                .args([
                    "--code",
                    "0x131a0",
                    "--args",
                    &(100.0 * charge).to_string(),
                    &(100.0 * VERY_LOW_BATTERY).to_string(),
                ])
                .spawn()
                .expect("Melissa Client Failed");

            Command::new("notif")
                .args(&*msgbox_low_args)
                .spawn()
                .expect("Notif Call Failed");
            Command::new("mmc31")
                .args([&*soundfile])
                .spawn()
                .expect("MMC Media Controller Call Failed");

            thread::spawn(move || {
                msgbox::create(     "Blaine Battery Monitor" ,
								&*format!("The Battery level {a} < {b} [CRITICAL LOWER LIMIT]. PLUG it IN right now!!!!!!!!" , a = ((charge*100.0) as isize) , b = ((VERY_LOW_BATTERY*100.0) as isize)) ,
								IconType::Error
							)
            });
            TOO_MANY_NOTIFS = TOO_MANY_NOTIFS + 1;
        }
    } else if charge < LOW_BATTERY {
        if status == battery::State::Discharging {
            Command::new("notif")
                .args(&*lowargs)
                .spawn()
                .expect("Program Call Failed");
        }
    } else if charge > VERY_HIGH_BATTERY {
        if TOO_MANY_NOTIFS > NOTIF_UPPER_LIMIT {
            return;
        }

        Command::new("notif")
            .args(&*msgbox_high_args)
            .spawn()
            .expect("Notif Call Failed");

        if status != battery::State::Discharging {
            //Command::new("MsgBox.exe").args(&*msgbox_high_args).output().expect("MsgBox Failed");

            thread::spawn(move || {
                msgbox::create(     "Blaine Battery Monitor" ,
                                &*format!("The Battery level {a} > {b} [Healthy Upper Limit]. PLUG it OUT right now!!!!!!!!" , a = ((charge*100.0) as isize) , b = ((VERY_HIGH_BATTERY*100.0) as isize)) ,
                                IconType::Error
							)
            });

            TOO_MANY_NOTIFS = TOO_MANY_NOTIFS + 1;
        }
    } else if charge > HIGH_BATTERY {
        if status != battery::State::Discharging {
            Command::new("notif")
                .args(&*highargs)
                .spawn()
                .expect("Program Call Failed");
        }
    } else {
        TOO_MANY_NOTIFS = 0;
    }
}

fn main() -> () {
    unsafe {
        let args: Vec<String> = env::args().collect();
        //assert_eq!(args.len() , 6 );

        let n = args.len();

        if n == 1 {
            println!("Args Structure: <prog> <sleep-time(s)> <high-battery(%)> <low-battery(%)> <very-low-battery(%)> <very-high-battery(%)>");
            //return ;
        }

        if n > 1 {
            SLEEP_TIME = time::Duration::from_millis(
                (String::from(args[1].clone())
                    .parse::<u64>()
                    .expect("Not Standard Integer Value"))
                    * 1000,
            );
        }
        if n > 2 {
            HIGH_BATTERY = (String::from(args[2].clone())
                .parse::<f64>()
                .expect("Not Standard Floating Point Value"))
                / 100.0;
        }
        if n > 3 {
            LOW_BATTERY = (String::from(args[3].clone())
                .parse::<f64>()
                .expect("Not Standard Floating Point Value"))
                / 100.0;
        }
        if n > 4 {
            VERY_LOW_BATTERY = (String::from(args[4].clone())
                .parse::<f64>()
                .expect("Not Standard Floating Point Value"))
                / 100.0;
        }
        if n > 5 {
            VERY_HIGH_BATTERY = (String::from(args[5].clone())
                .parse::<f64>()
                .expect("Not Standard Floating Point Value"))
                / 100.0;
        }

        drop(args);
        drop(n);

        let SLEEP_TIME_2 = SLEEP_TIME.clone();
        let SLEEP_TIME_3 = SLEEP_TIME.clone();
        let list_interfaces: Vec<String> = ["Ethernet 5"];
        let list_disks: Vec<String> = ["F:\\", "T:\\"];
        // let manager = battery::Manager::new();
        // let maybe_battery  = manager.expect("Manager Error").batteries().expect("Batteries not enumerating").enumerate()[0][0].expect("Battery Decode Error");

        println!("All Configuration Setup. Starting the actual program now");

        let battery_thread = thread::spawn(move || {
            loop {
                let manager = battery::Manager::new();

                //EvalBattery((format!("{:?}", maybe_battery.state_of_charge())).parse::<f64>().unwrap() , maybe_battery.state());

                //Get percentage, and charge status
                for (idx, maybe_battery) in manager.unwrap().batteries().unwrap().enumerate() {
                    let battery = maybe_battery.unwrap();
                    let ChargeState = (format!("{:?}", battery.state_of_charge()))
                        .parse::<f64>()
                        .unwrap();

                    EvalBattery(ChargeState, battery.state());
                }

                sleep(SLEEP_TIME);
            }
        });

        let network_thread = thread::spawn(move || {
            for interface in list_interfaces {
                let iface = Interface::get_by_name(&interface).expect("Reading Interface Failed");

                println!("Interface Status: {}", iface.is_up());
            }

            sleep(SLEEP_TIME_2);
        });

        let disk_thread = thread::spawn(move || {
            for disk in list_disks {
                println!("Disk Name: {}", disk);
            }

            sleep(SLEEP_TIME_3);
        });

        battery_thread.join();
        network_thread.join();
        disk_thread.join();

        return;
    }
}
}
