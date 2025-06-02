use std::io::Result;

use proc_result::ProcResult;

use crate::Output;

pub trait Child {
    /// Attempts to collect the exit status of the child if it has already exited.
    ///
    /// This function will not block the calling thread and will only check to see if the child
    /// process has exited or not. If the child has exited then on Unix the process ID is reaped.
    /// This function is guaranteed to repeatedly return a successful exit status so long as the
    /// child has already exited.
    ///
    /// If the child has exited, then `Ok(Some(()))` is returned.
    ///
    /// If the exit status is not available at this time then `Ok(None)` is returned.
    ///
    /// If an error occurs, then that error is returned.
    ///
    /// Note that unlike [`Child::wait`], this function will not attempt to drop `stdin`.
    ///
    /// # Errors
    ///
    /// Returns an error if the attempt to wait failed.
    fn try_wait(&mut self) -> Result<Option<ProcResult>>;

    /// Waits for the child to exit completely, returning the status that it exited with.
    ///
    /// This function will continue to have the same return value after it has been called at least
    /// once.
    ///
    /// The stdin handle to the child process, if any, will be closed before waiting. This helps
    /// avoid deadlock: it ensures that the child does not block waiting for input from the parent,
    /// while the parent waits for the child to exit.
    ///
    /// # Errors
    ///
    /// Returns an error if the attempt to wait failed.
    fn wait(&mut self) -> Result<ProcResult>;

    /// Simultaneously waits for the child to exit and collect all remaining output on the
    /// stdout/stderr handles, returning an [`Output`][`crate::Output`] instance.
    ///
    /// The stdin handle to the child process, if any, will be closed before waiting. This helps
    /// avoid deadlock: it ensures that the child does not block waiting for input from the parent,
    /// while the parent waits for the child to exit.
    ///
    /// By default, stdin, stdout and stderr are inherited from the parent. In order to capture the
    /// output into this `Result<Output>`` it is necessary to create new pipes between parent and
    /// child; see [`Stdio::Piped`][`crate::Stdio::Piped`].
    ///
    /// # Errors
    ///
    /// Returns an error if the attempt to wait failed.
    fn wait_with_output(&mut self) -> Result<Output>;
}
