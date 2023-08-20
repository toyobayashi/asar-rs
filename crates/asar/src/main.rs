use anyhow::Result;
use asar_rs::{
  create_package_with_options, extract_all, list_package_with_options, AsarFile, CreateOptions,
  ListOptions,
};
use clap::{
  arg, command,
  error::{ContextKind, ContextValue, ErrorKind},
  ArgAction, Command,
};

pub fn main() -> Result<()> {
  let bin_name = env!("CARGO_BIN_NAME");
  let matches = command!() // requires `cargo` feature
    .propagate_version(true)
    .subcommand_required(true)
    .arg_required_else_help(true)
    .name(bin_name)
    .subcommand(
      Command::new("pack")
        .alias("p")
        .about("create asar archive")
        .arg(arg!(--ordering <file_path> "path to a text file for ordering contents").required(false))
        .arg(arg!(--unpack <expression> "do not pack files matching glob <expression>").required(false))
        .arg(
          arg!(--"unpack-dir" <expression> "do not pack dirs matching glob <expression> or starting with literal <expression>")
            .required(false),
        )
        .arg(arg!(--"exclude-hidden" "exclude hidden files").action(ArgAction::SetTrue))
        .arg(arg!(<dir>))
        .arg(arg!(<output>)),
    )
    .subcommand(
      Command::new("list")
        .alias("l")
        .about("list files of asar archive")
        .arg(
          arg!(-i --"is-pack" "each file in the asar is pack or unpack")
            .action(ArgAction::SetTrue)
            .required(false),
        )
        .arg(arg!(<archive>)),
    )
    .subcommand(
      Command::new("extract-file")
        .alias("ef")
        .about("extract one file from archive")
        .arg(arg!(<archive>))
        .arg(arg!(<filename>)),
    )
    .subcommand(
      Command::new("extract")
        .alias("e")
        .about("extract archive")
        .arg(arg!(<archive>))
        .arg(arg!(<dest>)),
    )
    .try_get_matches()
    .unwrap_or_else(|e| {
      match e.kind() {
        ErrorKind::InvalidSubcommand => {
          let invalid_sub = e.get(ContextKind::InvalidSubcommand);
          if let Some(ContextValue::String(invalid_sub)) = invalid_sub {
            println!(
              "{}: '{}' is not an {} command. See \'{} --help\'.",
              bin_name,
              invalid_sub,
              bin_name,
              bin_name
            );
            std::process::exit(0);
          } else {
            e.exit();
          }
        },
        ErrorKind::UnknownArgument => {
          let invalid_arg = e.get(ContextKind::InvalidArg);
          if let Some(ContextValue::String(invalid_arg)) = invalid_arg {
            println!("error: unknown option '{}'", invalid_arg);
            std::process::exit(1);
          } else {
            e.exit();
          }
        }
        _ => {
          e.exit();
        }
      }
    });

  match matches.subcommand() {
    Some(("pack", sub_match)) => {
      let dir = sub_match.get_one::<String>("dir").unwrap();
      let output = sub_match.get_one::<String>("output").unwrap();
      let mut options = CreateOptions::new();
      options.unpack = sub_match.get_one::<String>("unpack").map(|v| v.clone());
      options.unpack_dir = sub_match.get_one::<String>("unpack-dir").map(|v| v.clone());
      options.ordering = sub_match
        .get_one::<std::path::PathBuf>("ordering")
        .map(|v| v.clone());
      options.dot = sub_match.get_one::<bool>("exclude-hidden").map(|v| !v);
      create_package_with_options(&dir, &output, &options)?;
    }
    Some(("list", sub_match)) => {
      let archive = sub_match.get_one::<String>("archive").unwrap();
      let mut options = ListOptions::new();
      options.is_pack = *sub_match.get_one::<bool>("is-pack").unwrap_or(&false);
      let list = list_package_with_options(archive, &options)?;
      for item in list {
        println!("{}", item);
      }
    }
    Some(("extract-file", sub_match)) => {
      let archive = sub_match.get_one::<String>("archive").unwrap();
      let filename = sub_match.get_one::<String>("filename").unwrap();
      let mut asar = AsarFile::open(archive)?;
      asar.extract_file(
        &filename,
        std::path::PathBuf::from(filename)
          .file_name()
          .unwrap()
          .to_str()
          .unwrap(),
      )?;
    }
    Some(("extract", sub_match)) => {
      let archive = sub_match.get_one::<String>("archive").unwrap();
      let dest = sub_match.get_one::<String>("dest").unwrap();
      extract_all(archive, dest)?;
    }
    _ => {
      unreachable!();
    }
  };

  Ok(())
}
