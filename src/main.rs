use clap::{Parser, Subcommand};
use std::path::PathBuf;

use pedalboard_cli::commands::{compile, flash, mode, monitor, read, status, upload};

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
    Reset,
    /// Reboot the device (no data loss)
    Reboot,
    /// Enter UF2 bootloader (for firmware flashing)
    Bootloader,
    /// Upload config via MIDI-CI Property Exchange
    Upload { file: PathBuf },
    /// Compile YAML config to binary (for use with pedalboard-sim)
    Compile {
        file: PathBuf,
        /// Output binary file path
        #[arg(short, long, default_value = "config.bin")]
        output: PathBuf,
    },
    /// Read back a preset from the device
    Read { index: u8 },
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
        Commands::Reset => status::reset(&cli.address).await?,
        Commands::Reboot => status::reboot(&cli.address).await?,
        Commands::Bootloader => status::bootloader(&cli.address).await?,
        Commands::Upload { file } => upload::pe_upload(&cli.address, &file).await?,
        Commands::Compile { file, output } => compile::compile(&file, &output)?,
        Commands::Read { index } => read::pe_read(&cli.address, index).await?,
        Commands::Monitor => monitor::monitor(&cli.address).await?,
        Commands::Flash { file } => flash::flash(&cli.address, &file).await?,
        Commands::Mode { mode: m } => mode::set_mode(&cli.address, &m).await?,
    }

    Ok(())
}
