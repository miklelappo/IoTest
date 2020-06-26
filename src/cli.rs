use clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg};

pub fn cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .author(crate_authors!())
        .about("An IO Load test")
        .arg(
            Arg::with_name("output_folder")
            .short("o")
            .long("output_folder")
            .takes_value(true)
            .required(true)
            .help("Select output folder"),
        )
        .arg(
            Arg::with_name("threads")
            .short("t")
            .long("threads")
            .takes_value(true)
            .default_value("10")
            .help("Select number of threads"),
        )
        .arg(
            Arg::with_name("blocksize")
            .short("s")
            .long("blocksize")
            .takes_value(true)
            .default_value("512")
            .help("Set blocksize"),
        )
        .arg(
            Arg::with_name("blocknum")
            .short("c")
            .long("blockcount")
            .takes_value(true)
            .default_value("1000")
            .help("Set number of blocks to be written by each thread"),
        )
        .arg(
            Arg::with_name("clean")
            .short("d")
            .long("clean")
            .help("Remove created files after test finished"),
        )

}