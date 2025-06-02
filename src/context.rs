use std::{
    collections::HashMap,
    ffi::OsString,
    path::{Path, PathBuf},
};

use crate::Stdio;

pub trait Context {
    /// Returns the working directory that is used on the process execution.
    ///
    /// By default, a value of `None` represents "inherited from the runner".
    #[must_use]
    fn working_dir(&self) -> Option<&Path>;

    /// Returns a mutable reference to the working directory.
    ///
    /// By default, a value of `None` represents "inherited from the runner".
    #[must_use]
    fn working_dir_mut(&mut self) -> Option<&mut PathBuf>;

    /// Sets the working directory that is used on the process execution.
    ///
    /// Replaces the existing working directory, if any.
    ///
    /// Returns `this` for use in a build-pattern.
    fn set_working_dir(&mut self, dir: impl Into<PathBuf>) -> &mut Self {
        if let Some(working_dir) = self.working_dir_mut() {
            *working_dir = dir.into();
        }
        self
    }

    /// Clears the working directory that is used on the process execution.
    ///
    /// The default inherited from the runner will be used instead.
    ///
    /// Returns `this` for use in a build-pattern.
    fn clear_working_dir(&mut self) -> &mut Self {
        if let Some(working_dir) = self.working_dir_mut() {
            working_dir.clear();
        }
        self
    }

    /// Returns the environment variables that are used on the process execution.
    #[must_use]
    fn env(&self) -> &HashMap<OsString, OsString>;

    /// Returns a mutable reference to the environment variables are used on the process execution.
    #[must_use]
    fn env_mut(&mut self) -> &mut HashMap<OsString, OsString>;

    /// Sets the environment variables that are used on the process execution.
    ///
    /// Returns `this` for use in a build-pattern.
    fn set_env(&mut self, env: impl Into<HashMap<OsString, OsString>>) -> &mut Self {
        *self.env_mut() = env.into();
        self
    }

    /// Clears the environment variables that are used on the process execution.
    ///
    /// Returns `this` for use in a build-pattern.
    fn clear_env(&mut self) -> &mut Self {
        self.env_mut().clear();
        self
    }

    /// Adds an environment variable to the process execution context.
    ///
    /// If the variable already exists, its value is replaced.
    ///
    /// Returns `this` for use in a build-pattern.
    fn add_env(&mut self, key: impl Into<OsString>, value: impl Into<OsString>) -> &mut Self {
        self.env_mut().insert(key.into(), value.into());
        self
    }

    /// Adds multiple environment variables to the process execution context.
    ///
    /// Existing variables with the same keys will be replaced.
    ///
    /// Returns `this` for use in a build-pattern.
    fn add_envs(
        &mut self,
        vars: impl IntoIterator<Item = (impl Into<OsString>, impl Into<OsString>)>,
    ) -> &mut Self {
        for (k, v) in vars {
            self.env_mut().insert(k.into(), v.into());
        }
        self
    }

    /// Returns whether the parent environment is inherited when executing a process.
    #[must_use]
    fn inherit_env(&self) -> bool;

    /// Returns a mutable reference to whether the parent environment is inherited.
    #[must_use]
    fn inherit_env_mut(&mut self) -> &mut bool;

    /// Sets whether the parent environment is inherited when executing a process.
    ///
    /// Returns `this` for use in a build-pattern.
    fn set_inherit_env(&mut self, inherit: bool) -> &mut Self {
        *self.inherit_env_mut() = inherit;
        self
    }

    /// Returns the configuration for the child process’s standard error (stderr) handle.
    ///
    /// By default, a value of `None` represents "inherited from the runner".
    #[must_use]
    fn stderr(&self) -> Option<Stdio>;

    /// Returns a mutable reference to the configuration for the child process’s standard error (stderr) handle.
    ///
    /// By default, a value of `None` represents "inherited from the runner".
    #[must_use]
    fn stderr_mut(&mut self) -> Option<&mut Stdio>;

    /// Sets the configuration for the child process’s standard error (stderr) handle.
    ///
    /// Returns `this` for use in a build-pattern.
    fn set_stderr(&mut self, stderr: Stdio) -> &mut Self {
        if let Some(s) = self.stderr_mut() {
            *s = stderr;
        }
        self
    }

    /// Returns the configuration for the child process’s standard output (stdout) handle.
    ///
    /// By default, a value of `None` represents "inherited from the runner".
    #[must_use]
    fn stdout(&self) -> Option<Stdio>;

    /// Returns a mutable reference to the configuration for the child process’s standard output (stdout) handle.
    ///
    /// By default, a value of `None` represents "inherited from the runner".
    #[must_use]
    fn stdout_mut(&mut self) -> Option<&mut Stdio>;

    /// Sets the configuration for the child process’s standard output (stdout) handle.
    ///
    /// Returns `this` for use in a build-pattern.
    fn set_stdout(&mut self, stdout: Stdio) -> &mut Self {
        if let Some(s) = self.stdout_mut() {
            *s = stdout;
        }
        self
    }

    /// Returns the configuration for the child process’s standard input (stdin) handle.
    ///
    /// By default, a value of `None` represents "inherited from the runner".
    #[must_use]
    fn stdin(&self) -> Option<Stdio>;

    /// Returns a mutable reference to the configuration for the child process’s standard input (stdin) handle.
    ///
    /// By default, a value of `None` represents "inherited from the runner".
    #[must_use]
    fn stdin_mut(&mut self) -> Option<&mut Stdio>;

    /// Sets the configuration for the child process’s standard input (stdin) handle.
    ///
    /// Returns `this` for use in a build-pattern.
    fn set_stdin(&mut self, stdin: Stdio) -> &mut Self {
        if let Some(s) = self.stdin_mut() {
            *s = stdin;
        }
        self
    }
}
