use std::num::ParseIntError;
use clap::{App, Arg, ArgAction, ArgGroup};
use windows::core::{HRESULT};
use windows::Win32::Foundation::{NTSTATUS, RtlNtStatusToDosError, WIN32_ERROR};

fn main() {
    #[cfg(not(windows))]
    { panic!("this program can only be run on Windows"); }
    
    let version = format!("v{}.{}", env!("CARGO_PKG_VERSION"), env!("commit_hash"));
    let matches = App::new("wcm")
        .version(version.as_str())
        .about("tool for query the windows error code information")
        .arg(Arg::new("version")
            .short('v')
            .long("version")
            .action(ArgAction::Version)
            .help("Prints version information"))
        .arg(Arg::with_name("ntstatus")
            .short('N')
            .long("ntstatus")
            .value_name("NTSTATUS")
            .allow_hyphen_values(true)
            .display_order(0)
            .help("NTSTATUS code")
            .takes_value(true))
        .arg(Arg::with_name("win32err")
            .short('E')
            .long("win32err")
            .value_name("Win32Error")
            .display_order(1)
            .help("Win32 error code")
            .takes_value(true))
        .arg(Arg::with_name("hresult")
            .short('H')
            .long("hresult")
            .value_name("HRESULT")
            .display_order(2)
            .help("HRESULT error code")
            .takes_value(true))
        .group(ArgGroup::with_name("flags")
            .args(&["ntstatus", "win32err", "hresult"])
            .required(true)
            .multiple(false))
        .get_matches();

    if let Some(ntstatus) = matches.value_of("ntstatus") {
        match parse_code(ntstatus) {
            Ok(ntstatus_code) => {
                let ntstatus = NTSTATUS(ntstatus_code as i32);
                let win32error_code = unsafe { RtlNtStatusToDosError(ntstatus) };
                let err = windows::core::Error::from(ntstatus);
                println!("   NTSTATUS: 0x{:08x}\nWin32 Error: {}\n    HRESULT: 0x{:08x}\n    Message: {}",
                         ntstatus.0,
                         win32error_code,
                         HRESULT::from_win32(win32error_code).0,
                         err.message());
            }
            Err(er) => eprintln!("invalid code value '{}', error: {}", ntstatus, er)
        }
    } else if let Some(win32err) = matches.value_of("win32err") {
        match parse_code(win32err) {
            Ok(win32error_code) => {
                let err = windows::core::Error::from(WIN32_ERROR(win32error_code as u32));
                println!("Win32 Error: {}\n    HRESULT: 0x{:08x}\n    Message: {}",
                         win32error_code,
                         HRESULT::from_win32(win32error_code as u32).0,
                         err.message());
            }
            Err(er) => eprintln!("invalid code value '{}', error: {}", win32err, er)
        }
    } else if let Some(hresult) = matches.value_of("hresult") {
        match parse_code(hresult) {
            Ok(hresult_code) => {
                let err = windows::core::Error::from_hresult(HRESULT(hresult_code as i32));
                println!(" HRESULT: 0x{:08x}\n Message: {}", err.code().0, err.message());
            }
            Err(er) => eprintln!("invalid code value '{}', error: {}", hresult, er)
        }
    };
}

fn parse_code(code: &str) -> Result<i64, ParseIntError> {
    let is_negative = code.starts_with('-');
    let number_str = if is_negative {
        &code[1..]
    } else {
        code
    };

    let number = if number_str.starts_with("0x") {
        i64::from_str_radix(&number_str[2..], 16)
    } else {
        number_str.parse::<i64>()
    }?;

    Ok(if is_negative { -number } else { number })
}

#[cfg(test)]
mod tests {
    use windows::core::HRESULT;
    use windows::Win32::Foundation::{E_ACCESSDENIED, ERROR_ACCESS_DENIED, NTSTATUS, RtlNtStatusToDosError, STATUS_ACCESS_DENIED, WIN32_ERROR};
    use super::parse_code;
    #[test]
    fn test_parse_code() {
        let cases = [
            "0xc0000022",
            "0x80070005",
            "5",
            "-2147024891",
            "3221225506",
            "-1073741790"
        ];
        for case in cases {
            let _ = parse_code(case).unwrap();
        }
    }

    #[test]
    fn test_ntstatus() {
        let ntstatus_code = parse_code("0xc0000022").unwrap();
        let ntstatus = NTSTATUS(ntstatus_code as i32);
        assert_eq!(ntstatus, STATUS_ACCESS_DENIED);

        let win32error_code = unsafe { WIN32_ERROR(RtlNtStatusToDosError(ntstatus)) };
        assert_eq!(win32error_code, ERROR_ACCESS_DENIED);

        let hresult = HRESULT::from_win32(win32error_code.0);
        assert_eq!(hresult, E_ACCESSDENIED);
    }
}