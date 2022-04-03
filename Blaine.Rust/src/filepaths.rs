extern crate lazy_static;

use crate::chaeyoung;

#[cfg(feature = "visible")]
lazy_static::lazy_static! {

    pub static ref LOG_FILE : String  = {

        let file = "{Lois}/Toolkit/Extra/Blaine/Blaine.Rust/standard.log" ; //D:\Toolkit\Extra\Blaine\Blaine.Rust\standard.log

        let path = chaeyoung::chaeyoung_translate(file).expect("Chaeyoung Translate Failed");
        path
    };
}

lazy_static::lazy_static!{


	pub static ref DISK_SOUND_FILE : String = {

		let path = chaeyoung::chaeyoung_translate("{Lois}/Toolkit/Extra/Blaine/Sounds/disk_stolen.wav")
            .expect("Chaeyoung Translate Failed");

        #[cfg(feature = "visible")]
        log::debug!("File Path to the Disk SoundFile: {}", path);

        path
	};

	pub static ref DISK_SOUND_FILE_SLEEP : std::time::Duration = {

		let time_needed = std::time::Duration::from_secs(15); 	//Length of Audio File
		time_needed
	};

	pub static ref ICON_PATH: String = {
        let path = chaeyoung::chaeyoung_translate(
            "{Lois}/Toolkit/Extra/Blaine/.Res/FA_Blaine_Female_F3_Formal_FV-12121212.jpg",
        )
        .expect("Chaeyoung Translate Failed");

        #[cfg(feature = "visible")]
        log::debug!("File Path to the icon: {}", path);

        path
    };

    pub static ref SOUNDFILE: String = {
        let path = chaeyoung::chaeyoung_translate("{Lois}/Toolkit/Extra/Blaine/Sounds/low_battery.wav")
            .expect("Chaeyoung Translate Failed");

        #[cfg(feature = "visible")]
        log::debug!("File Path to the SoundFile: {}", path);

        path
    };

}


fn main(){

	let xx = "";
	println!("{}", xx);
	return ;

}
