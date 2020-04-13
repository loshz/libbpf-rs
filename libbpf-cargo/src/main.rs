use std::path::PathBuf;
use std::process::exit;

use structopt::StructOpt;

mod build;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long)]
    debug: bool,
    #[structopt(subcommand)]
    wrapper: Wrapper,
}

// cargo invokes subcommands with the first argument as
// the subcommand name. ie.
//
//     cargo ${command} --help
//
// into
//
//     cargo-${command} ${command} --help
//
// so we must have a dummy subcommand here to eat the arg.
#[derive(Debug, StructOpt)]
enum Wrapper {
    Libbpf(Command),
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Build bpf programs
    Build {
        #[structopt(long, parse(from_os_str))]
        manifest_path: Option<PathBuf>,
    },
}

fn main() {
    let opts = Opt::from_args();

    let rc = match opts.wrapper {
        Wrapper::Libbpf(cmd) => match cmd {
            Command::Build { manifest_path } => build::build(opts.debug, manifest_path),
        },
    };

    exit(rc);
}
