use std::process::Command;

#[cfg(target_os = "windows")]
pub const CREATE_NO_WINDOW: u32 = 0x08000000;

#[cfg(target_os = "windows")]
pub const DETACHED_PROCESS: u32 = 0x00000008;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

//print<S: AsRef<str>>(stringlike: S)

pub fn chaeyoung_translate<S: AsRef<str> + std::convert::AsRef<std::ffi::OsStr>>(
    chaepath: S,
) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    let proc = Command::new("c-trans.exe")
        .arg(chaepath)
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    #[cfg(not(target_os = "windows"))]
    let proc = Command::new("c-trans").arg(chaepath).output();

    if proc.is_err() {
        return Err(proc.err().unwrap().to_string());
    }

    return Ok((String::from_utf8_lossy(&proc.unwrap().stdout)).to_string());
}
