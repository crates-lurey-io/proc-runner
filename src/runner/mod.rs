pub mod local;

use proc_result::ProcResult;

use crate::{Child, Context, Output};

/// A trait for types that can run commands.
pub trait Runner: Context {
    type Child: Child;

    /// Executes the command as a child process, returning a handle to it.
    ///
    /// By default, stdin, stdout and stderr are inherited from the parent.
    ///
    /// # Errors
    ///
    /// Returns an error if the process fails to start.
    fn spawn(&mut self) -> std::io::Result<Self::Child>;

    /// Executes the command as a child process, waiting for it to finish and collecting all of its output.
    ///
    /// By default, stdout and stderr are captured (and used to provide the resulting output). Stdin
    /// is not inherited from the parent and any attempt by the child process to read from the stdin
    /// stream will result in the stream immediately closing.
    ///
    /// # Errors
    ///
    /// Returns an error if the process fails to start.
    fn output(&mut self) -> std::io::Result<Output> {
        let mut child = self.spawn()?;
        match child.wait_with_output() {
            Ok(output) => Ok(output),
            Err(e) => Err(e),
        }
    }

    /// Executes a command as a child process, waiting for it to finish and collecting its status.
    ///
    /// By default, stdin, stdout and stderr are inherited from the parent.
    ///
    /// # Errors
    ///
    /// Returns an error if the process fails to start.
    fn status(&mut self) -> std::io::Result<ProcResult> {
        let mut child = self.spawn()?;
        match child.wait() {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }
}
