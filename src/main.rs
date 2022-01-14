use clap::{crate_authors, crate_name, crate_version, App, Arg};

mod utils;

fn main() {
    let args = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Quickly count gc content from a fasta file.")
        .arg(
            Arg::new("input")
                .long("input")
                .short('i')
                .help("Fasta file to analyze.")
                .takes_value(true)
                .required_unless_present_any(&["set", "reset"]),
        )
        .arg(
            Arg::new("browser")
                .long("browser")
                .short('b')
                .help("Browser choices.")
                .takes_value(true)
                .possible_values(&["default", "firefox", "Google Chrome", "safari"])
                .default_value("default")
                .required_unless_present_any(&["set", "reset"]),
        )
        .arg(
            Arg::new("set")
                .long("set")
                .takes_value(true)
                .help("Costum proxy.")
                .conflicts_with_all(&["browser", "input"]),
        )
        .arg(
            Arg::new("reset")
                .long("reset")
                .help("Reset to default settings.")
                .conflicts_with_all(&["browser", "input"]),
        )
        .get_matches();

    if args.is_present("set") {
        utils::set_proxy(
            args.value_of("set")
                .expect("Faile to parse proxy settings."),
        );
    } else if args.is_present("reset") {
        utils::reset_proxy();
    } else {
        let input = args.value_of("input").unwrap();
        let browser = args.value_of("browser").unwrap();
        let url = utils::generate_proxy_link(input);

        match browser {
            "default" => open::that(url).expect("Failed to open the link!"),
            _ => open::with(url, browser).expect("Failed to open the link!"),
        }

        println!("DONE!");
    }
}
