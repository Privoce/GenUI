use std::{
    ffi::OsStr,
    io::{BufRead, BufReader},
    path::Path,
    process::{Child, Command, ExitStatus, Stdio},
};

use crate::error::Error;

use super::fs;

/// ## Stream terminal output from child process
/// - info: stdout callback
/// - err: stderr callback
/// - return: exit status
/// ### Example
/// ```rust
/// stream_terminal(
///     &mut child,
///     |line| TerminalLogger::new(&line).info(),
///     |line| TerminalLogger::new(&line).error(),
/// )
/// .map_or_else(
///     |e| Err(e),
///     |status| {
///         if status.success() {
///             StudioLogs::Stop.terminal().success();
///             Ok(())
///         } else {
///             Err(Error::from("Failed to run makepad studio!"))
///         }
///     },
/// )
/// ```
pub fn stream_terminal<I, E>(child: &mut Child, info: I, err: E) -> Result<ExitStatus, Error>
where
    I: Fn(String) -> () + Send + 'static,
    E: Fn(String) -> () + Send + 'static,
{
    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    let stdout_thread = std::thread::spawn(move || {
        for line in stdout_reader.lines() {
            if let Ok(line) = line {
                // TerminalLogger::new(&line).info();
                info(line);
            }
        }
    });

    let stderr_thread = std::thread::spawn(move || {
        for line in stderr_reader.lines() {
            if let Ok(line) = line {
                // TerminalLogger::new(&line).error();
                err(line);
            }
        }
    });

    let status = child.wait().map_err(|e| e.to_string())?;
    stdout_thread.join().expect("Failed to join stdout thread");
    stderr_thread.join().expect("Failed to join stderr thread");

    Ok(status)
}

pub fn exec_cmd<I, S, P>(name: &str, args: I, current_dir: Option<P>) -> Command
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<Path>,
{
    let mut cmd = Command::new(name);

    cmd.args(args);

    if let Some(dir) = current_dir {
        cmd.current_dir(dir);
    }
    cmd
}

pub fn shadow_cmd<I, S, P>(name: &str, args: I, current_dir: Option<P>) -> Result<(), Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<Path>,
{
    let mut cmd = exec_cmd(name, args, current_dir);

    cmd.status().map_or_else(
        |e| Err(Error::from(e.to_string())),
        |status| {
            if status.success() {
                Ok(())
            } else {
                let output = cmd.output().map_err(|e| e.to_string())?;
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                Err(Error::from(stderr))
            }
        },
    )
}

pub fn shadow_cmd_with<I, S, P, F, R>(
    name: &str,
    args: I,
    current_dir: Option<P>,
    f: F,
) -> Result<R, Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<Path>,
    F: Fn(ExitStatus, std::process::Output) -> Result<R, Error>,
{
    let mut cmd = exec_cmd(name, args, current_dir);

    cmd.status().map_or_else(
        |e| Err(Error::from(e.to_string())),
        |status| {
            let output = cmd.output().map_err(|e| e.to_string())?;
            f(status, output)
        },
    )
}

pub fn stream_cmd<I, S, P>(
    name: &str,
    args: I,
    current_dir: Option<P>,
) -> Result<Child, std::io::Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    P: AsRef<Path>,
{
    exec_cmd(name, args, current_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
}

pub fn git_download_from_github<I, E, P>(
    url: &str,
    branch: &str, 
    target: &str,
    path: P,
    info: I,
    err: E,
) -> Result<(), Error>
where
    P: AsRef<Path>,
    I: Fn(String) -> () + Send + 'static,
    E: Fn(String) -> () + Send + 'static,
{
    // [init a tmp git repo for downloading] -----------------------------------------------------------------------
    let tmp_download_path = path.as_ref().join(".tmp");
    fs::delete_dir(&tmp_download_path)?;
    fs::create_dir(&tmp_download_path)?;
    // - [git init .tmp] -------------------------------------------------------------------------------------------
    shadow_cmd("git", &["init"], Some(&tmp_download_path))?;
    // - [add remote] ----------------------------------------------------------------------------------------------
    shadow_cmd(
        "git",
        &["remote", "add", "origin", url],
        Some(&tmp_download_path),
    )?;
    // - [config core.sparseCheckout true] --------------------------------------------------------------------------
    shadow_cmd(
        "git",
        &["config", "core.sparseCheckout", "true"],
        Some(&tmp_download_path),
    )?;
    // - [echo "dir" >> .git/info/sparse-checkout] ------------------------------------------------------------------
    let to = tmp_download_path
        .join(".git")
        .join("info")
        .join("sparse-checkout");
    fs::write(to.as_path(), &target)?;
    // [pull down] --------------------------------------------------------------------------------------------------
    let mut child = stream_cmd("git", ["pull", "origin", branch], Some(&tmp_download_path))
        .map_err(|e| e.to_string())?;

    stream_terminal(&mut child, info, err).map_or_else(
        |e| Err(e),
        |status| {
            if status.success() {
                Ok(())
            } else {
                Err(Error::from("please check you network connection"))
            }
        },
    )
}

pub fn git_download_plugin_from_github<I, E, P>(
    plugin: &str,
    is_token: bool,
    path: P,
    info: I,
    err: E,
) -> Result<(), Error>
where
    P: AsRef<Path>,
    I: Fn(String) -> () + Send + 'static,
    E: Fn(String) -> () + Send + 'static,
{
    let tmp_path = path.as_ref().join(".tmp");
    let (target, from_path) = if is_token {
        (
            format!("tokens/{}/*", plugin),
            tmp_path.join("tokens").join(plugin),
        )
    } else {
        (
            format!("projects/{}/*", plugin),
            tmp_path.join("projects").join(plugin),
        )
    };

    let to_path = path.as_ref().join(plugin);
    let _ = git_download_from_github(
        "https://github.com/Privoce/genui_plugins.git",
        "main",
        &target,
        path.as_ref(),
        info,
        err,
    )?;
    fs::move_to(from_path, to_path)?;
    fs::delete_dir(tmp_path)?;

    Ok(())
}
