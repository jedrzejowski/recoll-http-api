use crate::config::{get_prefixed_env, BinPaths};
use async_process::Command;
use std::ffi::OsStr;

pub fn make_command<S: AsRef<OsStr>>(cmd: S) -> Command {
  let mut cmd = if let Some(_) = get_prefixed_env("USE_FIREJAIL") {
    let mut firejail = Command::new(BinPaths::firejail());
    firejail.arg(cmd);
    firejail
  } else {
    Command::new(cmd)
  };

  cmd
}
