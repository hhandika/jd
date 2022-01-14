use clap::{App, Arg};

mod link_generator;

fn main() {
    let args = App::new("GC-Counter")
        .version("0.1.1")
        .about("Quickly count gc content from a fasta file.")
        .arg(
            Arg::new("input")
                .long("input")
                .short('i')
                .help("Fasta file to analyze.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("browser")
                .long("browser")
                .short('b')
                .help("Browser choices.")
                .takes_value(true)
                .possible_values(&["default", "firefox", "Google Chrome", "safari"])
                .default_value("default")
                .required(true),
        )
        .get_matches();

    let input = args.value_of("input").unwrap();
    let browser = args.value_of("browser").unwrap();

    let url = link_generator::generate_proxy_link(input).expect("Failed to generate proxy link");

    match browser {
        "default" => open::that(url).expect("Failed to open the link!"),
        _ => open::with(url, browser).expect("Failed to open the link!"),
    }

    println!("DONE!");
}
