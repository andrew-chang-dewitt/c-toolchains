use clap::Parser;
use sysinfo::System;

#[derive(Debug, Parser)]
#[command(version, about)]
/// A tool for creating c compiler toolchains for cross compilation using gcc, glibc, binutils, &
/// the linux kernal
struct Cli {
    /// target architecture for the toolchain to be created
    targetarch: String,
    /// host architecture for the toolchain to be created
    #[arg(long)]
    hostarch: Option<String>,
    /// build architecture for the toolchain to be created
    #[arg(long)]
    buildarch: Option<String>,
}

fn main() {
    let mut cli = Cli::parse();
    if let None = cli.hostarch {
        cli.hostarch = Some(System::cpu_arch());
    }
    if let None = cli.buildarch {
        cli.buildarch = Some(System::cpu_arch());
    }

    println!("cli: {cli:#?}");
}
