//#![windows_subsystem = "windows"]

extern crate battery;
extern crate msgbox;

#[cfg(feature = "visible")]
extern crate log;

use std::format;
use std::process::Command;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use crate::filepaths::{ICON_PATH,SOUNDFILE};

#[derive(Clone)]
pub struct BatteryConfig{

	pub battery_level : f64,
	pub battery_state : battery::State,
	pub lower_limit : f64,
	pub upper_limit : f64 ,
	pub upper_critical : f64 ,
	pub lower_critical : f64 ,
	pub super_critical_low : f64
}

impl BatteryConfig{

	pub fn new() -> Self {

		return BatteryConfig{

			battery_level : 0.15,
			battery_state : battery::State::Discharging,
			lower_limit : 0.22 ,
			upper_limit : 20.85,
			upper_critical : 0.91 ,
			lower_critical : 0.17 ,
			super_critical_low : 0.11

			// static mut BATTERY_LEVEL: f64 = 0.45;
			// static mut BATTERY_STATE: battery::State = battery::State::Discharging;
			// pub static mut LOWER_LIMIT: f64 = 0.22;
			// pub static mut UPPER_LIMIT: f64 = 0.85;
			// pub static mut UPPER_CRITICAL: f64 = 0.91;
			// pub static mut LOWER_CRITICAL: f64 = 0.16;
			// pub static mut SLEEP_TIME: u64 = 30;
			// pub static mut SUPER_CRITICAL_LOW : f64 = 0.11;

		}


	}

}

pub fn critical_low_alert(battery_level : f64 , lower_critical : f64) -> () {


	let	msg = format!(
	        "The battery is Critically low [{bat:.2}] < [{spec:.2}]. PLUG IT IN RIGHT NOW!!!",
	        bat = battery_level * 100.0,
	        spec = lower_critical * 100.0
	    );


    #[cfg(feature = "visible")]

    log::warn!(
        "Creating Critical Low [Notif] Alert at {:.1}% battery",
        battery_level * 100.0
    );

    #[cfg(feature = "visible")]
    log::warn!("Creating Critical Low [Notif] Alert");

	Command::new("notif")
        .args([
            "-t",
            "Blaine Battery Monitor",
            "-i",
            &ICON_PATH,
            "-d",
            "10",
            "-m",
            &msg.clone(),
        ])
        .spawn()
        .expect("notif call failed");


    #[cfg(feature = "visible")]
    log::warn!("Creating Critical Low [Melissa] Alert");


    Command::new("melissa-alerts")
        .args([
            "--code",
            "0x131a0",
            "--args",
            &format!("{:.2}", battery_level  * 100.0),
            &format!("{:.2}", lower_critical * 100.0),
        ])
        .spawn()
        .expect("Melissa Call Failed");


    #[cfg(feature = "visible")]
    log::warn!("Creating Critical Low [Mayday Media] Alert");

    Command::new("mmc31")
        .arg(SOUNDFILE.to_string())
        .spawn()
        .expect("Mayday Media Controller Call Failed");

    #[cfg(feature = "visible")]
    log::warn!("Creating Critical Low [MessageBox] Alert");

    msgbox::create("Blaine Battery Monitor", &msg, msgbox::IconType::Error);
}

pub fn echelon4(config : &BatteryConfig) -> () {

	#[cfg(feature = "visible")]
	log::error!("Shutting Down system via echelon 4 at {:.3} battery" , config.super_critical_low);

	std::process::Command::new("echelon4").spawn().expect("Echelon call failed");
	std::process::exit(0xff);

	return;
}

pub fn critical_high_alert(config : &BatteryConfig) -> () {

	let msg2 = format!(
	        "The battery is higher than healthy :: {bat:.2} > {spec:.2}. UNPLUG RIGHT AWAY!!!",
	        bat = config.battery_level  * 100.0,
	        spec = config.upper_critical * 100.0
	);


    #[cfg(feature = "visible")]

	log::warn!(
        "Creating Critical High [MessageBox] Alert at {:.1}% battery",
        config.battery_level * 100.0
	);


    msgbox::create("Blaine Battery Monitor", &msg2, msgbox::IconType::Error);
}

pub fn high_alert(config : &BatteryConfig) -> () {

    let msg = format!(
        "The Battery is higher than healthy :: {bat:.2} > {spec:.2}. Unplug soon!!!",
        bat = config.battery_level * 100.0,
        spec = config.upper_limit * 100.0
	);


    #[cfg(feature = "visible")]

	log::warn!(
        "Creating High [Notif] Alert at {:.1}% battery",
        config.battery_level * 100.0
	);

    Command::new("notif")
        .args([
            "-t",
            "Blaine Battery Monitor",
            "-m",
            &msg,
            "-i",
            &ICON_PATH,
            "-d",
            "5",
        ])
        .spawn()
        .expect("Notif Call Failed");
}

pub fn low_alert(config : &BatteryConfig) -> () {

	let msg = format!(
        "The Battery is lower than healthy :: {bat:.2} < {spec:.2}. CHARGE ASAP!!",
        bat = config.battery_level * 100.0,
        spec = config.lower_limit * 100.0
    );


    #[cfg(feature = "visible")]
	log::warn!(
        "Creating Low [Notif] Alert at {:.1}% battery",
        config.battery_level * 100.0
    );


    Command::new("notif")
        .args([
            "-t",
            "Blaine Battery Monitor",
            "-d",
            "10",
            "-i",
            &ICON_PATH,
            "-m",
            &msg,
        ])
        .spawn()
        .expect("notif call failed");
}

pub fn eval_battery(config : &mut BatteryConfig) -> () {

	if config.battery_level  < config.super_critical_low {
		if config.battery_state == battery::State::Discharging {
			echelon4(config);
			return;
		}
	}

	if config.battery_level < config.lower_critical  {
        if config.battery_state == battery::State::Discharging {
			let xx = config.battery_level.clone();
			let yy = config.lower_critical.clone();
			thread::spawn(move|| critical_low_alert(xx , yy));
			return ;
        }
	}

	if config.battery_level < config.lower_limit  {
        if config.battery_state == battery::State::Discharging {
            //thread::spawn(|| );
            low_alert(config);
			return ;
        }
	}

	if config.battery_level > config.upper_critical {

    	if config.battery_state != battery::State::Discharging {
        	critical_high_alert(config);
			return ;
    	}
	}

	if config.battery_level > config.upper_limit {
    	if config.battery_state != battery::State::Discharging {
        	high_alert(config);
			return ;
    	}
	}

    return;
}

pub fn monitor(config : BatteryConfig , sleep_secs : u64) -> () {

	let mut battery_config = config.clone();
	std::mem::drop(config);

	let sleep_time = Duration::from_secs(sleep_secs);

    if cfg!(feature = "visible") {
        log::debug!("Cycle/Sleep Time: {:?}", sleep_time);
        log::debug!(
            "Critically Low Battery Level: {:.1}",
            battery_config.lower_critical * 100.0
        );


		log::debug!("Low Battery Level: {:.1}", battery_config.lower_limit * 100.0);
    	log::debug!("High Battery Level: {:.1}", battery_config.upper_limit * 100.0);

	log::debug!(
        "Unhealthily High Battery Level: {:.1}",
        battery_config.upper_critical * 100.0
    );

	}


    loop {
        let manager = battery::Manager::new();
        for (_idx, maybe_battery) in manager.unwrap().batteries().unwrap().enumerate() {
            let battery = maybe_battery.unwrap();
            battery_config.battery_level = (format!("{:?}", battery.state_of_charge()))
                .parse::<f64>()
                .unwrap();
            battery_config.battery_state = battery.state();
            eval_battery(&mut battery_config);
        }

        #[cfg(feature = "visible")]
        log::trace!("Cycle Complete. Sleeping");

        sleep(sleep_time);
    }

    return;
}

fn main() {
    let xx = "";
}
