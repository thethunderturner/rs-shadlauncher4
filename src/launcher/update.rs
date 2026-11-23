use ring::digest;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub const CURRENT_COMMIT: &str = env!("LAUNCHER_COMMIT");
pub const CURRENT_DATE: &str = env!("LAUNCHER_COMMIT_DATE");
const API: &str = "https://api.github.com/repos/thethunderturner/rs-shadlauncher4";
const DOWNLOAD_PREFIX: &str = "https://github.com/thethunderturner/rs-shadlauncher4/releases/download/";

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Channel {
    #[default]
    Release,
    PreRelease,
}

impl Channel {
    pub fn label(self) -> &'static str {
        match self {
            Self::Release => "Release",
            Self::PreRelease => "Pre-release",
        }
    }
}

pub struct Update {
    pub commit: String,
    pub date: String,
    pub tag: String,
    pub version: String,
    pub commits: Vec<Commit>,
    pub available: bool,
    pub newer: bool,
    pub message: String,
    pub changelog_error: Option<String>,
    pub install_error: Option<String>,
    asset: Asset,
}

#[derive(Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetails,
}

#[derive(Deserialize)]
pub struct CommitDetails {
    pub message: String,
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    name: Option<String>,
    published_at: String,
    draft: bool,
    prerelease: bool,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
    size: u64,
    digest: Option<String>,
}

#[derive(Deserialize)]
struct Comparison {
    status: String,
    total_commits: usize,
    commits: Vec<Commit>,
}

pub fn short_commit(commit: &str) -> &str {
    commit.get(..7).unwrap_or(commit)
}

fn cancelled(cancel: &AtomicBool) -> Result<(), String> {
    if cancel.load(Ordering::Relaxed) {
        Err("Cancelled".into())
    } else {
        Ok(())
    }
}

fn agent(timeout: u64) -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(timeout)))
        .timeout_connect(Some(Duration::from_secs(15)))
        .timeout_recv_body(Some(Duration::from_secs(30)))
        .build()
        .new_agent()
}

fn get<T: DeserializeOwned>(agent: &ureq::Agent, path: &str, cancel: &AtomicBool) -> Result<T, String> {
    get_optional(agent, path, cancel)?
        .ok_or_else(|| "The requested release or commit was not found on GitHub.".into())
}

fn get_optional<T: DeserializeOwned>(
    agent: &ureq::Agent,
    path: &str,
    cancel: &AtomicBool,
) -> Result<Option<T>, String> {
    cancelled(cancel)?;
    let response = agent
        .get(&format!("{API}/{path}"))
        .header("User-Agent", "rs-shadLauncher4")
        .header("Accept", "application/vnd.github+json")
        .call();
    if matches!(&response, Err(ureq::Error::StatusCode(404))) {
        cancelled(cancel)?;
        return Ok(None);
    }
    let mut response = response.map_err(|error| match error {
        ureq::Error::StatusCode(403 | 429) => {
            "GitHub refused the request or its API rate limit was reached. Try again later.".into()
        }
        error => format!("Could not check GitHub: {error}"),
    })?;
    let bytes = response
        .body_mut()
        .with_config()
        .limit(16 * 1024 * 1024)
        .read_to_vec()
        .map_err(|error| format!("Could not read GitHub response: {error}"))?;
    cancelled(cancel)?;
    serde_json::from_slice(&bytes)
        .map(Some)
        .map_err(|error| format!("Invalid GitHub response: {error}"))
}

fn encode_segment(value: &str) -> String {
    value
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-_.~".contains(&byte) {
                (byte as char).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}

fn asset_name() -> Result<&'static str, String> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => Ok("rs-shadLauncher4-linux-x86_64.AppImage"),
        ("windows", "x86_64") => Ok("rs-shadLauncher4-windows-x86_64.exe"),
        ("macos", "aarch64") => Ok("rs-shadLauncher4-macos-arm64"),
        _ => Err("No launcher download is published for this operating system and architecture.".into()),
    }
}

fn latest_release(
    agent: &ureq::Agent,
    channel: Channel,
    cancel: &AtomicBool,
) -> Result<Option<Release>, String> {
    if channel == Channel::Release {
        let release: Option<Release> = get_optional(agent, "releases/latest", cancel)?;
        return Ok(release.filter(|release| !release.draft && !release.prerelease));
    }
    // GitHub's /latest endpoint excludes pre-releases. Walk the release list
    // until we find the newest published pre-release, ignoring stable releases.
    for page in 1.. {
        let releases: Vec<Release> = get(agent, &format!("releases?per_page=100&page={page}"), cancel)?;
        let count = releases.len();
        if let Some(release) = releases
            .into_iter()
            .find(|release| !release.draft && release.prerelease)
        {
            return Ok(Some(release));
        }
        if count < 100 {
            return Ok(None);
        }
    }
    unreachable!()
}

pub fn check(channel: Channel, cancel: &AtomicBool) -> Result<Option<Update>, String> {
    let name = asset_name()?;
    let agent = agent(60);
    let Some(release) = latest_release(&agent, channel, cancel)? else {
        return Ok(None);
    };
    let asset = release
        .assets
        .into_iter()
        .find(|asset| asset.name.eq_ignore_ascii_case(name))
        .ok_or("The latest release has no download for this platform. Its build may still be uploading.")?;
    // Resolve the tag, not target_commitish (which can be a moving branch).
    let latest: Commit = get(&agent, &format!("commits/{}", encode_segment(&release.tag_name)), cancel)?;
    if latest.sha.len() != 40 || !latest.sha.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("GitHub returned an invalid release commit.".into());
    }
    let mut update = Update {
        available: latest.sha != CURRENT_COMMIT,
        newer: false,
        commit: latest.sha,
        date: release.published_at,
        version: release
            .name
            .filter(|name| !name.trim().is_empty())
            .unwrap_or_else(|| release.tag_name.clone()),
        tag: release.tag_name,
        commits: Vec::new(),
        message: format!("The latest {} is available to download.", channel.label().to_lowercase()),
        changelog_error: None,
        install_error: installation_target().err(),
        asset,
    };
    if !update.available {
        update.message = "You are already running this version.".into();
        return Ok(Some(update));
    }
    if CURRENT_COMMIT == "unknown" {
        update.changelog_error =
            Some("This build has no commit information, so its changes cannot be compared.".into());
        return Ok(Some(update));
    }
    let comparison: Result<(), String> = (|| {
        let mut page = 1;
        loop {
            let comparison: Comparison = get_optional(
                &agent,
                &format!("compare/{CURRENT_COMMIT}...{}?per_page=100&page={page}", update.commit),
                cancel,
            )?.ok_or_else(|| format!(
                "GitHub cannot compare your build ({}) with {}. Your commit may not be published, or the histories may be unrelated. You can still download the selected version.",
                short_commit(CURRENT_COMMIT), update.version
            ))?;
            match comparison.status.as_str() {
                "identical" => {
                    update.available = false;
                    update.message = "You are already running this version.".into();
                    break;
                }
                "behind" => {
                    update.message = "The selected version is older than your current build. Updating will switch to this version.".into();
                    break;
                }
                "diverged" => {
                    update.message = "This version has a different history from your build. Updating will switch to the selected version.".into();
                }
                "ahead" => {
                    update.newer = true;
                }
                _ => return Err("GitHub returned an unknown comparison status.".into()),
            }
            let count = comparison.commits.len();
            update.commits.extend(comparison.commits);
            if update.commits.len() >= comparison.total_commits {
                break;
            }
            if count == 0 {
                return Err("GitHub returned an incomplete changelog. Try again.".into());
            }
            page += 1;
        }
        Ok(())
    })();
    if let Err(error) = comparison {
        cancelled(cancel)?;
        update.changelog_error = Some(error);
    }
    update.commits.reverse();
    Ok(Some(update))
}

fn installation_target() -> Result<PathBuf, String> {
    if cfg!(target_os = "linux") {
        let path =
            std::env::var_os("APPIMAGE").ok_or("Automatic updates on Linux require the AppImage build.")?;
        fs::canonicalize(path).map_err(|error| format!("Could not locate the AppImage: {error}"))
    } else {
        std::env::current_exe().map_err(|error| format!("Could not locate the launcher: {error}"))
    }
}

pub struct Download {
    directory: PathBuf,
    target: PathBuf,
    handed_off: bool,
}

impl Drop for Download {
    fn drop(&mut self) {
        if !self.handed_off {
            let _ = fs::remove_dir_all(&self.directory);
        }
    }
}

pub fn download(
    update: &Update,
    cancel: &AtomicBool,
    mut progress: impl FnMut(f32),
) -> Result<Download, String> {
    cancelled(cancel)?;
    let target = installation_target()?;
    let expected_digest = update
        .asset
        .digest
        .as_deref()
        .and_then(|value| value.strip_prefix("sha256:"))
        .filter(|value| value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .ok_or("This release has no SHA-256 checksum. The update cannot be verified.")?;
    if !update.asset.browser_download_url.starts_with(DOWNLOAD_PREFIX) {
        return Err("The download does not belong to the launcher repository.".into());
    }
    if update.asset.size == 0 || update.asset.size > 512 * 1024 * 1024 {
        return Err("The release download has an invalid size.".into());
    }
    let parent = target.parent().ok_or("The launcher has no parent directory.")?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();
    let directory = parent.join(format!(".launcher-update-{}-{timestamp}", std::process::id()));
    fs::create_dir(&directory).map_err(|error| format!("Cannot write beside the launcher: {error}"))?;
    let download = Download {
        directory,
        target,
        handed_off: false,
    };
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(download.directory.join("launcher.new"))
        .map_err(|error| error.to_string())?;
    let mut response = agent(600)
        .get(&update.asset.browser_download_url)
        .header("User-Agent", "rs-shadLauncher4")
        .call()
        .map_err(|error| format!("Could not download the update: {error}"))?;
    let mut reader = response.body_mut().as_reader();
    let mut hash = digest::Context::new(&digest::SHA256);
    let mut buffer = [0; 64 * 1024];
    let mut received = 0u64;
    let mut last_progress = Instant::now();
    loop {
        cancelled(cancel)?;
        let count = reader
            .read(&mut buffer)
            .map_err(|error| format!("Download failed: {error}"))?;
        if count == 0 {
            break;
        }
        received += count as u64;
        if received > update.asset.size {
            return Err("The download exceeds its expected size.".into());
        }
        file.write_all(&buffer[..count])
            .map_err(|error| format!("Could not save the update: {error}"))?;
        hash.update(&buffer[..count]);
        if last_progress.elapsed() >= Duration::from_millis(100) {
            progress(received as f32 / update.asset.size as f32);
            last_progress = Instant::now();
        }
    }
    cancelled(cancel)?;
    if received != update.asset.size {
        return Err("The update download is incomplete. Try again.".into());
    }
    let actual: String = hash
        .finish()
        .as_ref()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    if !actual.eq_ignore_ascii_case(expected_digest) {
        return Err("The downloaded update failed its checksum check. Try again.".into());
    }
    file.sync_all()
        .map_err(|error| format!("Could not save the update: {error}"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(download.directory.join("launcher.new"), fs::Permissions::from_mode(0o755))
            .map_err(|error| error.to_string())?;
    }
    progress(1.0);
    Ok(download)
}

// A separate process waits for this launcher to exit before replacing the file.
// Paths are passed as arguments, never interpolated into shell source.
pub fn install(mut download: Download) -> Result<(), String> {
    let script = download.directory.join(if cfg!(windows) {
        "install.ps1"
    } else {
        "install.sh"
    });
    fs::write(
        &script,
        if cfg!(windows) {
            WINDOWS_INSTALLER
        } else {
            UNIX_INSTALLER
        },
    )
    .map_err(|error| format!("Could not write the installer: {error}"))?;
    let log_path = download.directory.join("install.log");
    let log = File::create(&log_path).map_err(|error| error.to_string())?;
    let mut command;
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command = Command::new("powershell.exe");
        command.args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
        ]);
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }
    #[cfg(not(windows))]
    {
        command = Command::new("/bin/sh");
    }
    command
        .arg(&script)
        .arg(std::process::id().to_string())
        .arg(&download.target)
        .arg(&download.directory)
        .current_dir(
            download
                .target
                .parent()
                .ok_or("The launcher has no parent directory.")?,
        )
        .stdin(Stdio::null())
        .stdout(log.try_clone().map_err(|error| error.to_string())?)
        .stderr(log);
    // AppImage runtime variables point into the old mounted image.
    for key in [
        "APPIMAGE",
        "APPDIR",
        "ARGV0",
        "OWD",
        "LD_LIBRARY_PATH",
        "LD_PRELOAD",
    ] {
        command.env_remove(key);
    }
    let mut child = command
        .spawn()
        .map_err(|error| format!("Could not start the installer: {error}"))?;
    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait().map_err(|error| error.to_string())? {
            let details = fs::read_to_string(&log_path).unwrap_or_default();
            return Err(format!("The installer exited ({status}): {details}"));
        }
        if download.directory.join("ready").exists() {
            // Keep the old executable and log here for recovery if installation fails.
            download.handed_off = true;
            return Ok(());
        }
        if start.elapsed() >= Duration::from_secs(10) {
            let _ = child.kill();
            let _ = child.wait();
            return Err("The installer did not start. The launcher has not been changed.".into());
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

const UNIX_INSTALLER: &str = r#"#!/bin/sh
set -eu
parent_pid=$1
target=$2
staging=$3
touch "$staging/ready"
attempt=0
while kill -0 "$parent_pid" 2>/dev/null; do
    attempt=$((attempt + 1))
    if [ "$attempt" -ge 180 ]; then echo 'Launcher did not exit; update cancelled.'; exit 1; fi
    sleep 1
done
mv "$target" "$staging/previous"
if ! mv "$staging/launcher.new" "$target"; then
    mv "$staging/previous" "$target"
    exit 1
fi
"$target" </dev/null >>"$staging/install.log" 2>&1 &
launcher_pid=$!
sleep 2
if ! kill -0 "$launcher_pid" 2>/dev/null; then
    echo 'Updated launcher exited immediately; restoring the previous executable.'
    mv "$target" "$staging/failed"
    mv "$staging/previous" "$target"
    exit 1
fi
# Keep the previous executable and installer log for recovery.
"#;

const WINDOWS_INSTALLER: &str = r#"param([int]$LauncherPid, [string]$Target, [string]$Staging)
$ErrorActionPreference = 'Stop'
try {
    $launcher = Get-Process -Id $LauncherPid -ErrorAction SilentlyContinue
    New-Item -ItemType File -Path (Join-Path $Staging 'ready') | Out-Null
    if ($launcher -and -not $launcher.WaitForExit(180000)) { throw 'Launcher did not exit; update cancelled.' }
    $previous = Join-Path $Staging 'previous.exe'
    Move-Item -LiteralPath $Target -Destination $previous
    try {
        Move-Item -LiteralPath (Join-Path $Staging 'launcher.new') -Destination $Target
        $started = Start-Process -FilePath $Target -WorkingDirectory (Split-Path -LiteralPath $Target) -PassThru
        Start-Sleep -Seconds 2
        if ($started.HasExited) { throw 'Updated launcher exited immediately.' }
    } catch {
        if (Test-Path -LiteralPath $Target) { Move-Item -LiteralPath $Target -Destination (Join-Path $Staging 'failed.exe') }
        Move-Item -LiteralPath $previous -Destination $Target
        throw
    }
} catch {
    Write-Error $_
    exit 1
}
"#;
