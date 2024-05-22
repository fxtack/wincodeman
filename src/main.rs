use clap::{App, AppSettings, Arg, ArgAction, ArgGroup};

fn main() {
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
        println!("{}", ntstatus);
    } else if let Some(win32err) = matches.value_of("win32err") {
        println!("{}", win32err);
    } else if let Some(hresult) = matches.value_of("hresult") {
        println!("{}", hresult);
    }
}
