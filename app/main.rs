use clap::{App, Arg};

mod log;

mod dump;
pub(crate) use dump::Dump;

fn main()
{
  let matches = App::new("Recover")
    .version("0.1.0")
    .author("B. Howe <37745048+byhowe@users.noreply.github.com>")
    .about("Command line program to recover files from ext4 partitions")
    .subcommand(
      App::new("dump")
        .about("Dumps information about an ext4 partition")
        .author("B. Howe <37745048+byhowe@users.noreply.github.com>")
        .arg(
          Arg::with_name("path")
            .help("path to the partition")
            .takes_value(true)
            .value_name("PATH")
            .required(true),
        )
        .arg(
          Arg::with_name("offset")
            .help("offset from the start of the image")
            .takes_value(true)
            .default_value("0")
            .value_name("OFFSET")
            .long("offset")
            .short("s"),
        ),
    )
    .get_matches();

  match matches.subcommand() {
    ("dump", Some(subm)) => Dump {
      path: subm.value_of("path").unwrap().into(),
      offset: subm
        .value_of("offset")
        .unwrap()
        .parse::<u64>()
        .unwrap_or_else(|err| die!("Unable to parse OFFSET as a valid u64: {}", err)),
    }
    .run(),
    _ => {
      eprintln!("{}", matches.usage());
      std::process::exit(1);
    }
  }
}
