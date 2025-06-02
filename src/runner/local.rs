use std::{
    collections::HashMap,
    ffi::OsString,
    path::{Path, PathBuf},
};

use crate::{Context, Stdio};

#[derive(Debug, Default)]
pub struct LocalRunner {
    working_dir: Option<PathBuf>,
    inherit_env: bool,
    env: HashMap<OsString, OsString>,
    stderr: Option<Stdio>,
    stdout: Option<Stdio>,
    stdin: Option<Stdio>,
}

impl Context for LocalRunner {
    fn working_dir(&self) -> Option<&Path> {
        self.working_dir.as_deref()
    }

    fn working_dir_mut(&mut self) -> Option<&mut PathBuf> {
        self.working_dir.as_mut()
    }

    fn env(&self) -> &HashMap<OsString, OsString> {
        &self.env
    }

    fn env_mut(&mut self) -> &mut HashMap<OsString, OsString> {
        &mut self.env
    }

    fn inherit_env(&self) -> bool {
        self.inherit_env
    }

    fn inherit_env_mut(&mut self) -> &mut bool {
        &mut self.inherit_env
    }

    fn stderr(&self) -> Option<Stdio> {
        self.stderr
    }

    fn stderr_mut(&mut self) -> Option<&mut Stdio> {
        self.stderr.as_mut()
    }

    fn stdout(&self) -> Option<Stdio> {
        self.stdout
    }

    fn stdout_mut(&mut self) -> Option<&mut Stdio> {
        self.stdout.as_mut()
    }

    fn stdin(&self) -> Option<Stdio> {
        self.stdin
    }

    fn stdin_mut(&mut self) -> Option<&mut Stdio> {
        self.stdin.as_mut()
    }
}
