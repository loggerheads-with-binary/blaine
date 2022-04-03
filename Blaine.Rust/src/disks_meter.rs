use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use crate::filepaths::{ICON_PATH,DISK_SOUND_FILE,DISK_SOUND_FILE_SLEEP};
use crate::chaeyoung;

use std::format;

#[cfg(feature = "visible")]
extern crate log;

#[derive(Clone)]
pub struct AnnaDisk {
    pub alias: String,
    pub disk_path: PathBuf,
    pub DISK_DIRECTIVES: u64,
}

impl AnnaDisk {
    pub fn new(alias: String) -> Self {
        let disk_path = Path::new(
            &chaeyoung::chaeyoung_translate(format!("{{{}}}", alias))
                .expect("Chaeyoung Translate Failed"),
        )
        .to_path_buf();

        return AnnaDisk {
            alias: alias,
            disk_path: disk_path,
            DISK_DIRECTIVES: 0,
        };
    }
}

impl std::fmt::Display for AnnaDisk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{0}::<{1:?}>", self.alias, self.disk_path)
    }
}

impl std::fmt::Debug for AnnaDisk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{0}::<{1:?}>", self.alias, self.disk_path)
    }
}

pub struct DiskMonitoringConfig{
	pub max_directives: u64 ,
	pub upper_max_directives: u64,
	pub sleep_time : u64 ,
}

impl DiskMonitoringConfig{

	pub fn new() -> Self {

		return DiskMonitoringConfig{
			max_directives : 0,
			upper_max_directives:30,
			sleep_time : 3
		}

	}
}

pub fn check_disks(disks: &mut Vec<AnnaDisk> , config : &DiskMonitoringConfig) -> () {
    let mut all_clear_bool: bool = true;

    for disk in disks {
        //let disk = disks[i];

    if !disk.disk_path.exists() {
        let notif_msg = std::format!("The Disk [{}] is missing from the system", &disk.alias);

        all_clear_bool = false;
        disk.DISK_DIRECTIVES += 1;

        if disk.DISK_DIRECTIVES > config.upper_max_directives {
            return;
        }

        if disk.DISK_DIRECTIVES > config.max_directives {
            #[cfg(feature = "visible")]
            log::warn!(
                "Creating Melissa Alert for Disk {:?} since its been too long missing",
                disk
            );

                Command::new("melissa-alerts")
                    .args(["--code", "0x132a0", "--args", &disk.alias])
                    .spawn()
                    .expect("Melissa Call Failed");

			#[cfg(feature = "visible")]
			log::warn!("Creating mmc31 alert for disk {:?}" , disk);

			Command::new("mmc31").arg(DISK_SOUND_FILE.to_string()).spawn().expect("MMC Call Failed");
			std::thread::sleep(*DISK_SOUND_FILE_SLEEP);

            } else {
                #[cfg(feature = "visible")]
                log::warn!("Creating Notif Call for Missing Disk {:?}", disk);

                Command::new("notif")
                    .args(["-t", "Blaine Disk Monitor", "-m", &notif_msg, "-d", "9" , "-i" , &ICON_PATH])
                    .spawn()
                    .expect("Notif Call Failed");
            }

	}

	else {
            disk.DISK_DIRECTIVES = 0;
        }

	}

}

pub fn monitor(disks: &mut Vec<AnnaDisk> , config : &DiskMonitoringConfig) -> () {

	let mut sleep_time : Duration = Duration::from_secs(3);

    sleep_time = Duration::from_secs(config.sleep_time);

    if cfg!(feature = "visible") {
        log::debug!("Sleep/Cycle Time: {:?}", sleep_time);
        log::info!("Disks entered: {:?}", disks);
    }

    loop {

        #[cfg(feature = "visible")]
        log::trace!("Cycle Complete. Sleeping");

        check_disks(disks , config);
        //println!("Cycle compltete; sleeping for {:?}", sleep_time);
        sleep(sleep_time);

	}
}
