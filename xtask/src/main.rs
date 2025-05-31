use anyhow::Result;
use clap::{Parser, Subcommand};
use xshell::{cmd, Shell};

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Build automation for okid", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the Rust library
    Build {
        /// Build in release mode
        #[arg(long)]
        release: bool,
    },
    Wasm,
    /// Build and serve mdbook documentation
    Mdbook {
        /// Serve the book instead of just building
        #[arg(long)]
        serve: bool,
    },
    /// Build docs with all WASM modules
    BuildDocs {
        /// Build in release mode
        #[arg(long)]
        release: bool,
    },
    /// Run all tests
    Test,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let sh = Shell::new()?;

    match cli.command {
        Commands::Build { release } => build_rust_lib(&sh, release)?,
        Commands::Wasm => build_wasm(&sh)?,
        Commands::Mdbook { serve } => build_mdbook(&sh, serve)?,
        Commands::BuildDocs { release } => build_docs(&sh, release)?,
        Commands::Test => run_all_tests(&sh)?,
    }

    Ok(())
}

fn build_rust_lib(sh: &Shell, release: bool) -> Result<()> {
    println!("Building Rust library...");

    let mut args = vec!["build"];
    if release {
        args.push("--release");
    }
    args.extend(&["--features", "sha2,blake3,uuid,ulid"]);

    cmd!(sh, "cargo {args...}").run()?;

    println!("Library built successfully!");
    Ok(())
}

fn build_wasm(sh: &Shell) -> Result<()> {
    println!("Building WASM package...");

    // Check if wasm-pack is installed
    if cmd!(sh, "which wasm-pack").run().is_err() {
        println!("Installing wasm-pack...");
        let installer_script = sh.current_dir().join("_wasm_pack_installer.sh");
        cmd!(sh, "curl -sSfL https://rustwasm.github.io/wasm-pack/installer/init.sh -o {installer_script}").run()?;
        cmd!(sh, "chmod +x {installer_script}").run()?;
        cmd!(sh, "sh {installer_script}").run()?;
        sh.remove_path(&installer_script)?;
    }

    // Build with wasm-pack
    let rustflags = "--cfg getrandom_backend=\"wasm_js\"";
    cmd!(sh, "wasm-pack build --target web")
        .env("RUSTFLAGS", rustflags)
        .run()?;

    println!("WASM package built successfully!");
    Ok(())
}

fn build_mdbook(sh: &Shell, serve: bool) -> Result<()> {
    println!("Building mdbook documentation...");

    // Check if mdbook is installed
    if cmd!(sh, "which mdbook").run().is_err() {
        println!("Installing mdbook...");
        cmd!(sh, "cargo install mdbook").run()?;
    }

    // Build or serve the book
    if serve {
        println!("Serving mdbook at http://localhost:3000");
        cmd!(sh, "mdbook serve docs").run()?;
    } else {
        cmd!(sh, "mdbook build docs").run()?;
        println!("Mdbook documentation built successfully!");
    }

    Ok(())
}

fn run_all_tests(sh: &Shell) -> Result<()> {
    println!("Running all tests...");

    // Run Rust tests
    println!("Running Rust tests...");
    cmd!(sh, "cargo test --features sha2,blake3,uuid,ulid").run()?;

    println!("All tests passed!");
    Ok(())
}

fn build_docs(sh: &Shell, release: bool) -> Result<()> {
    println!("Building documentation with WASM modules...");

    // Check if wasm-pack is installed
    if cmd!(sh, "which wasm-pack").run().is_err() {
        println!("Installing wasm-pack...");
        let installer_script = sh.current_dir().join("_wasm_pack_installer.sh");
        cmd!(sh, "curl -sSfL https://rustwasm.github.io/wasm-pack/installer/init.sh -o {installer_script}").run()?;
        cmd!(sh, "chmod +x {installer_script}").run()?;
        cmd!(sh, "sh {installer_script}").run()?;
        sh.remove_path(&installer_script)?;
    }

    // Create output directories
    cmd!(sh, "mkdir -p docs/src/okid").run()?;

    // Build okid WASM module
    println!("📦 Building okid WASM module...");
    let rustflags = "--cfg getrandom_backend=\"wasm_js\"";
    let mut cmd = cmd!(sh, "wasm-pack build --target web");
    if release {
        cmd = cmd.arg("--release");
    } else {
        cmd = cmd.arg("--dev");
    }
    cmd.env("RUSTFLAGS", rustflags).run()?;

    // Copy okid to docs/src
    println!("📋 Copying okid to docs/src/okid...");
    let pkg_contents = sh.read_dir("pkg")?;
    for entry in pkg_contents {
        let src = entry;
        let dest = sh
            .current_dir()
            .join("docs/src/okid")
            .join(src.file_name().unwrap());
        if src.is_dir() {
            cmd!(sh, "cp -r {src} {dest}").run()?;
        } else {
            cmd!(sh, "cp {src} {dest}").run()?;
        }
    }
    println!("✅ okid copied to docs/src/okid");

    println!("🎉 Both WASM modules have been built and copied to docs/src");
    println!("Available modules:");
    println!("  - docs/src/okid/");

    Ok(())
}
