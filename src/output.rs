use core::fmt;
use std::fmt::Debug;

use proc_result::ProcResult;

/// The output of a finished process.
#[derive(Clone, PartialEq, Eq)]
pub struct Output {
    /// The status or exit code of the process.
    pub result: ProcResult,

    /// The data that the process wrote to stdout.
    pub stdout: Vec<u8>,

    /// The data that the process wrote to stderr.
    pub stderr: Vec<u8>,
}

// If either stderr or stdout are valid utf8 strings it prints the strings, otherwise the bytes.
impl Debug for Output {
    #[allow(clippy::ref_binding_to_reference)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stdout_utf8 = std::str::from_utf8(&self.stdout);
        let stdout_debug: &dyn fmt::Debug = match stdout_utf8 {
            Ok(ref s) => s,
            Err(_) => &self.stdout,
        };
        let stderr_utf8 = std::str::from_utf8(&self.stderr);
        let stderr_debug: &dyn fmt::Debug = match stderr_utf8 {
            Ok(ref s) => s,
            Err(_) => &self.stderr,
        };

        f.debug_struct("Output")
            .field("result", &self.result)
            .field("stdout", stdout_debug)
            .field("stderr", stderr_debug)
            .finish()
    }
}
