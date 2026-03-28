mod error;

use clap::Parser;
use error::NiriCwdError;
use niri_ipc::{Request, Response, socket::Socket};
use procfs::process::Process;
use std::{path::PathBuf, process::exit};

#[derive(Parser)]
struct Args {
    /// Directory to output in case of error
    #[arg(short, long, value_name = "DIR")]
    default_dir: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    match (get_focused_cwd(), args.default_dir) {
        (Ok(path), _) => {
            println!("{}", path.display());
        }

        (Err(_), Some(default_dir)) => {
            println!("{}", default_dir.display())
        }

        (Err(err), _) => {
            eprintln!("{}", err);
            exit(1)
        }
    }
}

fn get_focused_cwd() -> Result<PathBuf, NiriCwdError> {
    let mut socket = Socket::connect()?;
    let reply = socket.send(Request::FocusedWindow)?;

    let pid = match reply {
        Ok(Response::FocusedWindow(Some(win))) => win.pid.ok_or(NiriCwdError::NoPid),

        Ok(Response::FocusedWindow(None)) => Err(NiriCwdError::NoFocusedWindow),
        Ok(_) => Err(NiriCwdError::UnexpectedResponse),
        Err(msg) => Err(NiriCwdError::NiriReplyError(msg)),
    }?;

    let deepest = deepest_child(pid);
    let path = Process::new(deepest)?.cwd()?;

    path.exists()
        .then_some(path)
        .ok_or(NiriCwdError::CwdResolveFailed(pid))
}

fn deepest_child(pid: i32) -> i32 {
    get_children(pid)
        .and_then(|children| children.into_iter().max())
        .map_or(pid, |u| u as i32)
}

fn get_children(pid: i32) -> Option<Vec<u32>> {
    let proc = Process::new(pid).ok()?;
    let tasks = proc.tasks().ok()?;
    let children: Vec<u32> = tasks
        .flatten()
        .filter_map(|task| task.children().ok())
        .flatten()
        .collect();
    Some(children)
}
