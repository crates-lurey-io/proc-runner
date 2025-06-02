use std::{
    collections::HashMap,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

use crate::{Context, Stdio};

/// Configures how to execute a process using a [`crate::Runner`].
#[derive(Clone, Debug)]
pub struct Command {
    program: OsString,
    args: Vec<OsString>,
    working_dir: Option<PathBuf>,
    inherit_env: bool,
    env: HashMap<OsString, OsString>,
    stderr: Option<Stdio>,
    stdout: Option<Stdio>,
    stdin: Option<Stdio>,
}

impl Command {
    /// Returns the program that running command will execute.
    #[must_use]
    pub fn program(&self) -> &OsStr {
        &self.program
    }

    /// Sets the `program` that will be executed by a runner.
    ///
    /// Returns `this` for use in a build-pattern.
    pub fn set_program(&mut self, program: impl Into<OsString>) -> &mut Self {
        self.program = program.into();
        self
    }

    /// Returns the arguments that are passed to [`Command::program`] when executed.
    #[must_use]
    pub fn args(&self) -> &[OsString] {
        &self.args
    }

    /// Adds `arg` as an agument to [`Command::program`].
    ///
    /// The argument is added to the end of the argument list.
    ///
    /// Returns `this` for use in a build-pattern.
    pub fn add_arg(&mut self, arg: impl Into<OsString>) -> &mut Self {
        self.args.push(arg.into());
        self
    }

    /// Adds `args` as arguments to [`Command::program`].
    ///
    /// Each argument is added to the end of the argument list in the order iterated.
    ///
    /// Returns `this` for use in a build-pattern.
    pub fn add_args(&mut self, args: impl IntoIterator<Item = impl Into<OsString>>) -> &mut Self {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    /// Clears all arguments to [`Command::program`].
    ///
    /// Returns `this` for use in a build-pattern.
    pub fn clear_args(&mut self) -> &mut Self {
        self.args.clear();
        self
    }

    /// Sets the arguments that are passed to [`Command::program`] when executed.
    ///
    /// Any existing arguments are cleared.
    ///
    /// Returns `this` for use in a build-pattern.
    pub fn set_args(&mut self, args: impl IntoIterator<Item = impl Into<OsString>>) -> &mut Self {
        self.clear_args();
        self.add_args(args);
        self
    }

    /// Returns a mutable slice to the arguments that would be passed to [`Command::program`].
    #[must_use]
    pub fn args_mut(&mut self) -> &mut [OsString] {
        &mut self.args
    }
}

impl Context for Command {
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
