use std::ffi::{c_char, c_int};

#[link(name = "systemd")]
extern "C" {
    /// Notify service manager about start-up completion and other service status changes
    ///
    /// # Description
    /// [`sd_notify`] may be called by a service to notify the service manager about state changes.
    /// It can be used to send arbitrary information, encoded in an environment-block-like string.
    /// Most importantly, it can be used for start-up or reload completion notifications.
    ///
    /// If the `unset_environment` parameter is non-zero, [`sd_notify`] will unset the
    /// `$NOTIFY_SOCKET` environment variable before returning (regardless of whether the function
    /// call itself succeeded or not). Further calls to [`sd_notify`] will then silently do
    /// nothing, and the variable is no longer inherited by child processes.
    ///
    /// The `state` parameter should contain a newline-separated list of variable assignments,
    /// similar in style to an environment block. A trailing newline is implied if none is
    /// specified. The string may contain any kind of variable assignments, but see the next
    /// section for a list of assignments understood by the service manager.
    ///
    /// Note that systemd will accept status data sent from a service only if the "NotifyAccess="
    /// option is correctly set in the service definition file.
    ///
    /// Note that [`sd_notify`] notifications may be attributed to units correctly only if either
    /// the sending process is still around at the time PID 1 processes the message, or if the
    /// sending process is explicitly runtime-tracked by the service manager. The latter is the
    /// case if the service manager originally forked off the process, i.e. on all processes that
    /// match "NotifyAccess=main" or "NotifyAccess=exec". Conversely, if an auxiliary process of
    /// the unit sends an [`sd_notify`] message and immediately exits, the service manager might
    /// not be able to properly attribute the message to the unit, and thus will ignore it, even if
    /// "NotifyAccess=all" is set for it.
    ///
    /// Hence, to eliminate all race conditions involving lookup of the client's unit and
    /// attribution of notifications to units correctly, [`sd_notify_barrier`] may be used. This
    /// call acts as a synchronization point and ensures all notifications sent before this call
    /// have been picked up by the service manager when it returns successfully. Use of
    /// [`sd_notify_barrier`] is needed for clients which are not invoked by the service manager,
    /// otherwise this synchronization mechanism is unnecessary for attribution of notifications to
    /// the unit.
    ///
    /// # Return Value
    ///  On failure, this call returns a negative errno-style error code. If `$NOTIFY_SOCKET` was
    /// not set and hence no status message could be sent, 0 is returned. If the status was sent,
    /// these functions return a positive value. In order to support both service managers that
    /// implement this scheme and those which do not, it is generally recommended to ignore the
    /// return value of this call. Note that the return value simply indicates whether the
    /// notification message was enqueued properly, it does not reflect whether the message could
    /// be processed successfully. Specifically, no error is returned when a file descriptor is
    /// attempted to be stored using "FDSTORE=1" but the service is not actually configured to
    /// permit storing of file descriptors.
    pub fn sd_notify(unset_environment: c_int, state: *const c_char) -> c_int;
}
