use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NiriCwdError {
    #[error("unable to obtain niri socket")]
    NiriSocketError(#[from] io::Error),
    #[error("niri reply error: {0}")]
    NiriReplyError(String),
    #[error("focused window not found")]
    NoFocusedWindow,
    #[error("no pid for focused window not found")]
    NoPid,
    #[error("unexpected response from niri")]
    UnexpectedResponse,
    #[error("unable to obtain cwd for pid: {0}")]
    CwdResolveFailed(i32),
    #[error("procfs error: {0}")]
    ProcfsError(#[from] procfs::ProcError),
}
