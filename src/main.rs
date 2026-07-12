use clap::{Parser, Subcommand};
use std::path::PathBuf;

use pedalboard_cli::commands::{flash, mode, monitor, read, status, upload};

#[derive(Parser)]
#[command(name = "pedalboard-cli", about = "Pedalboard configuration tool", version = concat!(env!("CARGO_PKG_VERSION"), "-", env!("GIT_HASH")))]
struct Cli {
    /// WebSocket address of the bridge
    #[arg(
        short,
        long,
        default_value = "ws://cm5-dev.home:8080",
        env = "PEDALBOARD_ADDR"
    )]
    address: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show device status (firmware version, preset count, health)
    Status,
    /// Factory reset the device
    Reset {
        /// Wait for device to be ready after reset
        #[arg(long)]
        wait: bool,
    },
    /// Reboot the device (no data loss)
    Reboot {
        /// Wait for device to be ready after reboot
        #[arg(long)]
        wait: bool,
    },
    /// Enter UF2 bootloader (for firmware flashing)
    Bootloader,
    /// Upload config via MIDI-CI Property Exchange
    Upload {
        file: PathBuf,
        /// Show what would be uploaded without connecting to the device
        #[arg(long)]
        dry_run: bool,
    },
    /// Read back a preset from the device
    Read {
        /// Preset index (0-31). Omit with --all to read all presets.
        #[arg(required_unless_present = "all")]
        index: Option<u8>,
        /// Output as valid setlist YAML (re-uploadable)
        #[arg(long)]
        yaml: bool,
        /// Read all presets
        #[arg(long)]
        all: bool,
    },
    /// Monitor MIDI output from the device in real-time
    Monitor,
    /// Flash a UF2 firmware file to the device (enters bootloader, uploads via bridge)
    Flash { file: PathBuf },
    /// Switch between live mode (bridge controls audio) and design mode (MOD UI controls audio)
    Mode {
        /// "live" or "design"
        mode: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => status::device_status(&cli.address).await?,
        Commands::Reset { wait } => status::reset(&cli.address, wait).await?,
        Commands::Reboot { wait } => status::reboot(&cli.address, wait).await?,
        Commands::Bootloader => status::bootloader(&cli.address).await?,
        Commands::Upload { file, dry_run } => {
            upload::pe_upload(&cli.address, &file, dry_run).await?
        }
        Commands::Read { index, yaml, all } => {
            read::pe_read(&cli.address, index, yaml, all).await?
        }
        Commands::Monitor => monitor::monitor(&cli.address).await?,
        Commands::Flash { file } => flash::flash(&cli.address, &file).await?,
        Commands::Mode { mode: m } => mode::set_mode(&cli.address, &m).await?,
    }

    Ok(())
}
