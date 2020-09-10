use clap::{App, Arg};

enum Action
{
  Dump(super::Dump),
}

pub(crate) struct Console
{
  action: Action,
}

impl Console
{
  pub(crate) fn new() -> Option<Console>
  {
    let matches = App::new("recover")
      .version("0.1.0")
      .author("B. Howe <37745048+byhowe@users.noreply.github.com>")
      .about("command line program to recover files from ext4 partitions")
      .subcommand(
        App::new("dump")
          .about("dumps information about an ext4 partition")
          .author("B. Howe <37745048+byhowe@users.noreply.github.com>")
          .arg(
            Arg::with_name("path")
              .help("path to the partition")
              .required(true),
          ),
      )
      .get_matches();

    let action = match matches.subcommand() {
      ("dump", Some(subm)) => Some(Action::Dump(super::Dump {
        path: subm.value_of("path").unwrap(),
      })),
      _ => None,
    };

    if let Some(action) = action {
      Some(Console { action })
    } else {
      None
    }
  }

  pub(crate) fn run(&self)
  {
    match &self.action {
      Action::Dump(dump) => dump.run(),
    }
  }
}
