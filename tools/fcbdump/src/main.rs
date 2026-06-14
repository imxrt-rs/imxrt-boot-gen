//! Write every FlexSPI Configuration Block (FCB) in this workspace to disk.
//!
//! 1. `cargo rustc ... --emit=obj` compiles the crate to an object file for an
//!    ARM target, where the FCB lands in `.fcb`.
//! 2. `rust-objcopy` copies that section into a flat `<crate>.bin`.
//!
//! The tool takes no arguments: every board found under the workspace's
//! `fcbs/` directory is built, and its `.bin` is written into `target/fcbdump`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

/// Build each FCB for an embedded target.
///
/// It's not affecting the contents within the FCB; that's just data.
/// However, targeting an embedded platform means the `.fcb` section
/// is emitted.
const TARGET: &str = "thumbv7em-none-eabihf";

/// The link section holding the configuration block.
const FCB_SECTION: &str = ".fcb";

/// This tool's own manifest, baked in at build time.
///
/// Although this is usually invoked from the workspace root, we can
/// use our own manifest to find other contents within the workspace.
const MANIFEST: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");

/// A buildable FCB, discovered from the workspace.
struct Fcb {
    /// Cargo package name, e.g. `imxrt1010evk-fcb`.
    package: String,
}

impl Fcb {
    fn stem(&self) -> String {
        self.package.replace('-', "_")
    }

    fn output(&self) -> String {
        format!("{}.bin", self.stem())
    }
}

/// The objcopy program to use; override with `OBJCOPY` (e.g. `llvm-objcopy`).
fn objcopy() -> String {
    std::env::var("OBJCOPY").unwrap_or_else(|_| "rust-objcopy".to_string())
}

/// The cargo program to use; set by Cargo when run via `cargo run`.
fn cargo() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}

fn path_str(path: &Path) -> Result<&str, String> {
    path.to_str()
        .ok_or_else(|| format!("path is not valid UTF-8: {}", path.display()))
}

/// Locate the workspace root by asking cargo, so every other path can be built
/// from it instead of guessed with `..` hops out of this crate's directory.
fn workspace_root() -> Result<PathBuf, String> {
    let output = Command::new(cargo())
        .args(["locate-project", "--workspace", "--message-format", "plain"])
        .args(["--manifest-path", MANIFEST])
        .output()
        .map_err(|err| format!("could not run cargo locate-project: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "cargo locate-project failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    // The output is the path to the workspace root `Cargo.toml`; its parent is
    // the workspace root directory.
    let manifest = String::from_utf8(output.stdout)
        .map_err(|err| format!("cargo locate-project output is not UTF-8: {err}"))?;
    Path::new(manifest.trim())
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("workspace manifest has no parent: {}", manifest.trim()))
}

/// Discover the FCB crates by scanning the `fcbs/` directory.
///
/// Each subdirectory `fcbs/<board>` is assumed to be a Cargo package named
/// `<board>-fcb` (e.g. `imxrt1010evk` -> `imxrt1010evk-fcb`); we don't read the
/// manifests. The only filter is that the directory must hold a `Cargo.toml`,
/// to skip stray files. If the assumed package name is wrong, the cargo build
/// for that board fails later.
fn discover(fcbs_dir: &Path) -> Result<Vec<Fcb>, String> {
    let entries = fs::read_dir(fcbs_dir)
        .map_err(|err| format!("could not read {}: {err}", fcbs_dir.display()))?;

    let mut fcbs = Vec::new();
    for entry in entries {
        let path = entry
            .map_err(|err| format!("could not read {}: {err}", fcbs_dir.display()))?
            .path();
        if !path.join("Cargo.toml").is_file() {
            continue;
        }
        let Some(board) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        fcbs.push(Fcb {
            package: format!("{board}-fcb"),
        });
    }
    fcbs.sort_by(|a, b| a.package.cmp(&b.package));

    Ok(fcbs)
}

/// Build `fcb` for the ARM target and extract its `.fcb` section into `out_dir`,
/// using `build_dir` as cargo's target directory.
fn dump(fcb: &Fcb, out_dir: &Path, build_dir: &Path) -> Result<(), String> {
    let object_path = out_dir.join(format!("{}.o", fcb.stem()));
    let output_path = out_dir.join(fcb.output());
    let object = path_str(&object_path)?;
    let output = path_str(&output_path)?;
    let build = path_str(build_dir)?;

    // Force a fresh compile. `cargo rustc --emit=obj` only writes the object
    // when rustc actually runs; if cargo deems the crate fresh it skips
    // compilation and no object appears.
    let status = Command::new(cargo())
        .args(["clean", "--manifest-path", MANIFEST])
        .args(["-p", &fcb.package, "--release", "--target", TARGET])
        .args(["--target-dir", build])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|err| format!("could not run cargo clean: {err}"))?;
    if !status.success() {
        return Err(format!("cargo clean failed for {}", fcb.package));
    }

    // Step 1: compile the FCB crate to an object file for the ARM target.
    let status = Command::new(cargo())
        .args(["rustc", "--manifest-path", MANIFEST])
        .args(["-p", &fcb.package, "--release", "--target", TARGET])
        .args(["--target-dir", build])
        .arg("--")
        .arg(format!("--emit=obj={object}"))
        .status()
        .map_err(|err| format!("could not run cargo: {err}"))?;
    if !status.success() {
        return Err(format!("cargo failed to build {}", fcb.package));
    }

    // Step 2: copy just the `.fcb` section into a flat binary.
    let status = Command::new(objcopy())
        .args(["-O", "binary", "--only-section", FCB_SECTION])
        .arg(object)
        .arg(output)
        .status()
        .map_err(|err| {
            format!(
                "could not run `{}` (install with `cargo install cargo-binutils` \
                 and `rustup component add llvm-tools`, or set OBJCOPY): {err}",
                objcopy()
            )
        })?;
    if !status.success() {
        return Err(format!(
            "objcopy failed to extract {FCB_SECTION} from {object}"
        ));
    }

    println!("wrote {output}");
    Ok(())
}

fn run() -> Result<(), String> {
    let root = workspace_root()?;
    let fcbs_dir = root.join("fcbs");
    // Generated binaries go in `target/fcbdump`; the crates are built in a
    // nested, dedicated target directory so repeated cleans never invalidate
    // the workspace's main `target/` cache.
    let out_dir = root.join("target").join("fcbdump");
    let build_dir = out_dir.join("build");
    fs::create_dir_all(&out_dir)
        .map_err(|err| format!("could not create {}: {err}", out_dir.display()))?;

    for fcb in discover(&fcbs_dir)? {
        dump(&fcb, &out_dir, &build_dir)?;
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}
