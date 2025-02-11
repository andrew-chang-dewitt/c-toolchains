use std::{fs::create_dir_all, path::PathBuf};

pub use anyhow::anyhow;
use clap::ArgAction;
pub use clap::Parser;
use log::info;
use sysinfo::System;

use crate::logging;

// use crate::logging::initialize;

#[derive(Clone, Debug)]
pub enum Verbosity {
    Debug,
    Info,
    Warn,
    Error,
}

impl From<u8> for Verbosity {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Error,
            1 => Self::Warn,
            2 => Self::Info,
            _ => Self::Debug,
        }
    }
}

#[derive(Debug, Parser)]
#[command(version, about)]
/// A tool for creating c compiler toolchains for cross compilation using gcc, glibc, binutils, &
/// the linux kernal
struct Args {
    /// target architecture for the toolchain to be created
    targetarch: String,
    /// host architecture for the toolchain to be created
    #[arg(long)]
    hostarch: Option<String>,
    /// build architecture for the toolchain to be created
    #[arg(long)]
    buildarch: Option<String>,
    /// location to save built toolchain
    #[arg(short, long)]
    outdir: Option<String>,
    /// specify output verbosity
    #[arg(short, long, action = ArgAction::Count)]
    verbose: Option<u8>,
}

#[derive(Debug)]
pub struct Config {
    verbose: Verbosity,
    hostarch: String,
    buildarch: String,
    targetarch: String,
    outdir: PathBuf,
}

impl TryFrom<Args> for Config {
    type Error = anyhow::Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let machine_arch = System::cpu_arch();
        let verbose = args
            .verbose
            .map(|v| Verbosity::from(v))
            .unwrap_or(Verbosity::Warn);
        let hostarch = args.hostarch.unwrap_or(machine_arch.clone());
        let buildarch = args.buildarch.unwrap_or(machine_arch.clone());
        let targetarch = args.targetarch;

        let default_outdir = get_default_outdir()?;
        let mut outdir = args.outdir.map(|s| s.into()).unwrap_or(default_outdir);
        /*  T==B | T==H | 1 & 2 -> B==H | name
         *  ---- | ---- | ------------- | ----
         *   T   |  T   |  T            | T
         *   T   |  F   |  F            | T_on_H
         *   F   |  T   |  F            | T_from_B
         *   F   |  F   |  T            | T_from_B
         *   F   |  F   |  F            | T_on_H_from_B
         */
        let subdir = if targetarch == buildarch {
            if targetarch == hostarch {
                targetarch.clone()
            } else {
                format!("{}_on_{}", &targetarch, &hostarch)
            }
        } else {
            if targetarch == hostarch {
                format!("{}_on_{}", &targetarch, &buildarch)
            } else {
                if buildarch == hostarch {
                    format!("{}_on_{}", &targetarch, &buildarch)
                } else {
                    format!("{}_on_{}_from_{}", &targetarch, &hostarch, &buildarch)
                }
            }
        };
        outdir.push(subdir);

        return Ok(Config {
            verbose,
            hostarch,
            buildarch,
            targetarch,
            outdir,
        });
    }
}

/// Create a `/.c-toolchains` directory in the users home directory, if it can be found
fn get_default_outdir() -> anyhow::Result<PathBuf> {
    match home::home_dir() {
        Some(mut dir) => {
            dir.push(".c-toolchains");
            Ok(dir)
        },
        None => Err( anyhow!("Unable to locate user's home directory to use as default output path. Please provide an output path via the `--outdir` option.") )
    }
}

#[derive(Debug)]
pub struct App {
    config: Config,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let args = Args::parse();
        let config = Config::try_from(args)?;

        logging::initialize(&config.verbose)?;

        info!("app config loaded: {config:#?}");

        Ok(App { config })
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let out_location = &self.config.outdir;
        self.setup()
            .and_then(|_| self.binutils())
            .and_then(|_| self.gcc1())
            .and_then(|_| self.linux_headers())
            .and_then(|_| self.glibc())
            .and_then(|_| self.gcc2())?;
        println!(
            "{} toolchain built & saved at {out_location:#?}",
            self.config.targetarch
        );
        Ok(())
    }

    fn setup(&self) -> anyhow::Result<()> {
        create_dir_all(&self.config.outdir)?;

        Ok(())
    }

    fn binutils(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn gcc1(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn linux_headers(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn glibc(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn gcc2(&self) -> anyhow::Result<()> {
        todo!()
    }
}
