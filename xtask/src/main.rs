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
    /// Generate Swift bindings
    Swift,
    /// Run Swift tests
    SwiftTest,
    /// Generate Swift documentation
    SwiftDocs,
    /// Build Swift module with swiftc
    SwiftBuild,
    /// Build WASM package
    Wasm,
    /// Build and serve mdbook documentation
    Mdbook {
        /// Serve the book instead of just building
        #[arg(long)]
        serve: bool,
    },
    /// Run all tests
    Test,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let sh = Shell::new()?;

    match cli.command {
        Commands::Build { release } => build_rust_lib(&sh, release)?,
        Commands::Swift => generate_swift_bindings(&sh)?,
        Commands::SwiftTest => run_swift_tests(&sh)?,
        Commands::SwiftDocs => generate_swift_docs(&sh)?,
        Commands::SwiftBuild => build_swift_module(&sh)?,
        Commands::Wasm => build_wasm(&sh)?,
        Commands::Mdbook { serve } => build_mdbook(&sh, serve)?,
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

fn generate_swift_bindings(sh: &Shell) -> Result<()> {
    println!("Generating Swift bindings...");

    // First build the library in release mode
    build_rust_lib(sh, true)?;

    // Create temporary script file with embedded content
    const SWIFT_BINDINGS_SCRIPT: &str = include_str!("../../generate_swift_bindings.sh");
    let script_path = sh.current_dir().join("_temp_swift_bindings.sh");
    sh.write_file(&script_path, SWIFT_BINDINGS_SCRIPT)?;
    
    // Make it executable and run
    cmd!(sh, "chmod +x {script_path}").run()?;
    cmd!(sh, "sh {script_path}").run()?;
    
    // Clean up temp file
    sh.remove_path(&script_path)?;
    
    println!("Swift bindings generated successfully!");
    Ok(())
}

fn run_swift_tests(sh: &Shell) -> Result<()> {
    println!("Running Swift tests...");

    // Generate bindings first
    generate_swift_bindings(sh)?;

    // Run Swift tests
    cmd!(sh, "swift test").run()?;

    println!("Swift tests completed successfully!");
    Ok(())
}

fn build_swift_module(sh: &Shell) -> Result<()> {
    println!("Building Swift module...");

    // Generate bindings first
    generate_swift_bindings(sh)?;

    // Set up environment variables (matching Docker setup)
    let cwd = sh.current_dir();
    let lib_path = cwd.join("target/release");
    let include_path = cwd.join("Sources/OkId/include");
    
    sh.set_var("LD_LIBRARY_PATH", format!("{}:{}", cwd.display(), lib_path.display()));
    sh.set_var("LIBRARY_PATH", lib_path.to_str().unwrap());
    sh.set_var("C_INCLUDE_PATH", include_path.to_str().unwrap());
    sh.set_var("CPLUS_INCLUDE_PATH", include_path.to_str().unwrap());

    // Create directories for outputs
    cmd!(sh, "mkdir -p .build/symbol-graphs").run()?;

    // Determine library extension based on platform
    let lib_extension = if cfg!(target_os = "macos") { "dylib" } else { "so" };
    let output_lib = format!("libOkId.{}", lib_extension);

    // Try direct swiftc compilation (matching Docker approach)
    println!("Compiling Swift module with swiftc...");
    let swiftc_result = cmd!(sh, "swiftc 
        -module-name OkId 
        -emit-library -o {output_lib} 
        -emit-module -emit-module-path ./ 
        -parse-as-library 
        -enable-testing 
        -L ./target/release/ 
        -lokid 
        -Xcc -fmodule-map-file=Sources/OkId/include/okidFFI.modulemap 
        -I Sources/OkId/include 
        Sources/OkId/OkId.swift Sources/OkId/OkIdTypes.swift"
    ).run();

    if swiftc_result.is_err() {
        println!("Direct swiftc compilation failed. Your Swift environment may have compatibility issues.");
        println!("Possible solutions:");
        println!("1. Use Docker: docker build -f Dockerfile.swift-docs -t okid-swift-docs .");
        println!("2. Use macOS where Swift has better support");
        println!("3. Check your Swift installation and ensure it matches your system architecture");
        return Err(anyhow::anyhow!("Swift module compilation failed due to environment issues"));
    }

    println!("Swift module built successfully!");
    Ok(())
}

fn generate_swift_docs(sh: &Shell) -> Result<()> {
    println!("Generating Swift documentation...");

    // Build the Swift module first
    build_swift_module(sh)?;

    // Create output directory
    cmd!(sh, "mkdir -p docs/src/swift").run()?;
    cmd!(sh, "mkdir -p .build/symbol-graphs").run()?;

    // Try to generate symbol graphs manually with swiftc
    println!("Generating symbol graphs manually...");
    let symbol_result = cmd!(sh, "swiftc 
        -emit-symbol-graph -emit-symbol-graph-dir .build/symbol-graphs 
        -module-name OkId 
        -parse-as-library 
        -L ./target/release/ 
        -lokid 
        -Xcc -fmodule-map-file=Sources/OkId/include/okidFFI.modulemap 
        -I Sources/OkId/include 
        Sources/OkId/OkId.swift Sources/OkId/OkIdTypes.swift"
    ).run();

    if symbol_result.is_ok() {
        println!("Symbol graphs generated successfully!");
        
        // Try documentation with the manually generated symbol graphs
        let doc_result = cmd!(sh, "swift package --allow-writing-to-directory docs/src/swift generate-documentation --target OkId --disable-indexing --transform-for-static-hosting --hosting-base-path okid/swift --output-path docs/src/swift --additional-symbol-graph-dir .build/symbol-graphs").run();
        
        if doc_result.is_ok() {
            println!("Swift documentation generated successfully!");
            return Ok(());
        }
    }

    // If we reach here, symbol graph approach failed
    println!("Symbol graph approach failed. Trying alternative method...");
    
    // Try using docc directly if available
    if cmd!(sh, "which docc").run().is_ok() {
        println!("Using docc directly...");
        let docc_result = cmd!(sh, "docc convert Sources/OkId/Documentation.docc 
            --fallback-display-name OkId 
            --fallback-bundle-identifier com.okid 
            --output-path docs/src/swift 
            --hosting-base-path okid/swift 
            --transform-for-static-hosting"
        ).run();
        
        if docc_result.is_ok() {
            println!("Swift documentation generated successfully with docc!");
            return Ok(());
        }
    }

    // Final fallback
    println!("All documentation generation methods failed.");
    println!("This is due to swift-symbolgraph-extract not supporting C module dependencies.");
    println!("Recommended solutions:");
    println!("1. Use Docker: docker build -f Dockerfile.swift-docs -t okid-swift-docs .");
    println!("2. Use macOS where Swift has better C module support");
    println!("3. Manually document the Swift API");
    
    Err(anyhow::anyhow!("Swift documentation generation failed due to C module dependency issues"))
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
    cmd!(sh, "wasm-pack build --target web").env("RUSTFLAGS", rustflags).run()?;
    
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

    // Run Swift tests
    run_swift_tests(sh)?;

    println!("All tests passed!");
    Ok(())
}