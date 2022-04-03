//#![windows_subsystem = "windows"]

mod battery_monitor;
mod chaeyoung;
mod disks_meter;
mod filepaths;

extern crate clap;
use clap::{load_yaml, App};
use std::path::Path;
use std::thread;

#[cfg(feature = "visible")]
extern crate log;

#[cfg(feature = "visible")]
extern crate simplelog;

#[cfg(feature = "visible")]
use filepaths::{LOG_FILE};

use std::convert::TryInto;

fn demo<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub fn clap_make
(
	disks: &mut Vec<disks_meter::AnnaDisk> ,
	battery_config : &mut battery_monitor::BatteryConfig ,
	battery_sleep_time : &mut u64 ,
	disk_config : &mut disks_meter::DiskMonitoringConfig ,
	WHAT_ABOUT_BATTERY : &mut bool ,
	WHAT_ABOUT_DISKS : &mut bool ,
) -> () {
    let yaml = load_yaml!(".././clap.yaml");
    let matches = App::from(yaml)
        .arg(
            clap::Arg::new("disks")
                .long("disks")
                .short('d')
                .takes_value(true)
                .multiple_values(true)
                .default_values(&["Tillie", "Ferb"]),
        )
        .get_matches();

    #[cfg(feature = "visible")]
    log::info!("Collected All Command Line Arguments");

    *battery_sleep_time = matches
        .value_of("sleep_time")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(*battery_sleep_time);

    battery_config.upper_limit = matches
        .value_of("high_battery")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(battery_config.upper_limit*100.0)
        / 100.0;

    battery_config.lower_limit = matches
        .value_of("low_battery")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(battery_config.lower_limit*100.0)
        / 100.0;

    battery_config.lower_critical = matches
        .value_of("low_critical")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(battery_config.lower_critical*100.0)
        / 100.0;

    battery_config.upper_critical = matches
        .value_of("high_critical")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(battery_config.upper_critical*100.0)
        / 100.0;

	battery_config.super_critical_low = matches
		.value_of("super_low_critical")
		.and_then(|s| s.parse::<f64>().ok())
		.unwrap_or(battery_config.super_critical_low *100.0)
		/100.0;

    disk_config.sleep_time = matches
        .value_of("disk_monitor_sleep_time")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(disk_config.sleep_time);

    disk_config.max_directives = matches
        .value_of("disks_max_warnings")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(disk_config.max_directives);

    disk_config.upper_max_directives = matches
        .value_of("disks_upper_limit_warnings")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(disk_config.upper_max_directives);

    *WHAT_ABOUT_BATTERY = !matches.is_present("no_battery_monitor");
    *WHAT_ABOUT_DISKS = matches.is_present("disk_monitor");


    let temp_disks: Vec<_> = matches.values_of("disks").unwrap().collect();

    disks.clear();

    for alias in temp_disks {

        disks.push(disks_meter::AnnaDisk::new(alias.to_string()));
    }
}

fn main() {
    #[cfg(feature = "visible")]
    let builder = simplelog::ConfigBuilder::new()
        .set_max_level(simplelog::LevelFilter::Error)
        .set_time_level(simplelog::LevelFilter::Error)
        .set_time_format_str("%d-%m-%Y %H:%M:%S%.3f")
        .set_time_to_local(true)
        .build();

    if cfg!(feature = "log_stderr") {
        simplelog::CombinedLogger::init(vec![
            simplelog::WriteLogger::new(
                simplelog::LevelFilter::Info,
                builder.clone(),
                std::fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&*LOG_FILE)
                    .unwrap(),
            ),
            simplelog::TermLogger::new(
                simplelog::LevelFilter::Trace,
                builder,
                simplelog::TerminalMode::Stderr,
                simplelog::ColorChoice::Always,
            ),
        ]);
    } else {
        simplelog::CombinedLogger::init(vec![simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            builder,
            std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(&*LOG_FILE)
                .unwrap(),
        )]);
    }

    let mut disks = Vec::<disks_meter::AnnaDisk>::new();
	let mut battery_config = battery_monitor::BatteryConfig::new();
	let mut disk_config = disks_meter::DiskMonitoringConfig::new();
	let mut battery_sleep_time : u64 = 75;
	let mut WHAT_ABOUT_DISKS = false;
	let mut WHAT_ABOUT_BATTERY = true ;

	clap_make(		&mut disks,
					&mut battery_config , &mut battery_sleep_time ,
					&mut disk_config ,
				 	&mut WHAT_ABOUT_BATTERY , &mut WHAT_ABOUT_DISKS);

	#[cfg(feature = "visible")]
    log::info!(
        "Initiating Instance of Blaine at {0} PID {1}",
        std::env::consts::OS,
        std::process::id()
    );

     if WHAT_ABOUT_BATTERY {
        #[cfg(feature = "visible")]
        log::info!("Starting the Battery Thread");
        let battery_thread = thread::spawn(move || battery_monitor::monitor(battery_config ,battery_sleep_time));

    	if !WHAT_ABOUT_DISKS {
            battery_thread.join();
        }
    }

    if WHAT_ABOUT_DISKS {
        #[cfg(feature = "visible")]
        log::info!("Starting the Disk Thread");
        let disks_thread = thread::spawn(move || disks_meter::monitor(&mut disks , &disk_config));
        disks_thread.join();
    }

}
