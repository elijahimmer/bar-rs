use super::VOLUME_COMMAND;
use anyhow::Result;
use std::process::Command;
use std::rc::Rc;

pub fn run_command(args: &[&str]) -> Result<Rc<str>> {
    let out = Command::new(VOLUME_COMMAND).args(args).output()?;

    if out.status.success() {
        Ok(std::str::from_utf8(&out.stdout)?.into())
    } else if let Some(code) = out.status.code() {
        Err(RunCommandError::StatusCode(code).into())
    } else {
        Err(RunCommandError::ProgramCrashed.into())
    }
}

#[derive(Debug)]
pub enum RunCommandError {
    StatusCode(i32),
    ProgramCrashed,
}

use std::error::Error;
impl Error for RunCommandError {}

use std::fmt::{Display, Error as FmtError, Formatter};
impl Display for RunCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::StatusCode(i) => write!(f, "status_code={}", i)?,
            Self::ProgramCrashed => write!(f, "program crashed")?,
        }

        Ok(())
    }
}
