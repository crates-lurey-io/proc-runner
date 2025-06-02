/// Describes what to do with a standard I/O stream for a child process.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stdio {
    /// The child inherits from the corresponding parent descriptor.
    Inherit,

    /// This stream will be ignored.
    ///
    /// This is the equivalent of attaching the stream to `/dev/null`.
    Null,

    /// A new pipe should be arranged to connect the parent and child processes.
    Piped,
}
