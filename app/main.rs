mod dump;
pub(crate) use dump::Dump;

use clap::{App, Arg};

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
            .required(true),
        ),
    )
    .get_matches();

  match matches.subcommand() {
    ("dump", Some(subm)) => Dump {
      path: subm.value_of("path").unwrap().into(),
    }
    .run(),
    _ => {
      eprintln!("{}", matches.usage());
      std::process::exit(1);
    }
  }
}
