use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::sys::stat::{S_ISGID, S_ISUID};

/// In a system with the `_POSIX_CHOWN_RESTRICTED` option defined, this overrides the restriction
/// of changing file ownership and group ownership.
pub const CAP_CHOWN: c_int = 2;

/// Override all DAC access, including ACL execute access if `_POSIX_ACL` is defined. Excluding DAC
/// access covered by [`CAP_LINUX_IMMUTABLE`].
pub const CAP_DAC_OVERRIDE: c_int = 2;

/// Overrides all DAC restrictions regarding read and search on files and directories, including
/// ACL restrictions if `_POSIX_ACL` is defined. Excluding DAC access covered by
/// [`CAP_LINUX_IMMUTABLE`].
pub const CAP_DAC_READ_SEARCH: c_int = 2;

/// Overrides all restrictions about allowed operations on files, where file owner ID must be equal
/// to the user ID, except where [`CAP_FSETID`] is applicable. It doesn't override MAC and DAC
/// restrictions.
pub const CAP_FOWNER: c_int = 2;

/// Overrides the following restrictions that the effective user ID shall match the file owner ID
/// when setting the [`S_ISUID`] and [`S_ISGID`] bits on that file; that the effective group ID (or
/// one of the supplementary group IDs) shall match the file owner ID when setting the [`S_ISGID`]
/// bit on that file; that the [`S_ISUID`] and [`S_ISGID`] bits are cleared on successful return
/// from [`chown`] (not implemented).
pub const CAP_FSETID: c_int = 2;

/// Overrides the restriction that the real or effective user ID of a process sending a signal must
/// match the real or effective user ID of the process receiving the signal.
pub const CAP_KILL: c_int = 2;

/// Allows [`setgid`] manipulation
///
/// Allows [`setgroups`]
///
/// Allows forged gids on socket credentials passing.
pub const CAP_SETGID: c_int = 2;

/// Allows `set*uid` manipulation (including fsuid).
///
/// Allows forged pids on socket credentials passing.
pub const CAP_SETUID: c_int = 2;

/// Without VFS support for capabilities:
///  - Transfer any capability in your permitted set to any pid,
///  - remove any capability in your permitted set from any pid
///
/// With VFS support for capabilities (neither of above, but)
///  - Add any capability from current's capability bounding set to the current process'
///    inheritable set
///  - Allow taking bits out of capability bounding set
///  - Allow modification of the securebits for a process
pub const CAP_SETPCAP: c_int = 2;

/// Allow modification of [`S_IMMUTABLE`] and [`S_APPEND`] file attributes
pub const CAP_LINUX_IMMUTABLE: c_int = 2;

/// Allows binding to TCP/UDP sockets below 1024
///
/// Allows binding to ATM VCIs below 32
pub const CAP_NET_BIND_SERVICE: c_int = 2;

/// Allow broadcasting, listen to multicast
pub const CAP_NET_BROADCAST: c_int = 2;

/// Allow interface configuration
///
/// Allow administration of IP firewall, masquerading and accounting
///
/// Allow setting debug option on sockets
///
/// Allow modification of routing tables
///
/// Allow setting arbitrary process / process group ownership on sockets
///
/// Allow binding to any address for transparent proxying (also via [`CAP_NET_RAW`])
///
/// Allow setting TOS (type of service)
///
/// Allow setting promiscuous mode
///
/// Allow clearing driver statistics
///
/// Allow multicasting
///
/// Allow read/write of device-specific registers
///
/// Allow activation of ATM control sockets
pub const CAP_NET_ADMIN: c_int = 2;

/// Allow use of `RAW` sockets
///
/// Allow use of `PACKET` sockets
///
/// Allow binding to any address for transparent proxying (also via [`CAP_NET_ADMIN`])
pub const CAP_NET_RAW: c_int = 2;

/// Allow locking of shared memory segments
///
/// Allow [`mlock`] and [`mlockall`] (which doesn't really have anything to do with IPC)
pub const CAP_IPC_LOCK: c_int = 2;

/// Override IPC ownership checks
pub const CAP_IPC_OWNER: c_int = 2;

/// Insert and remove kernel modules - modify kernel without limit
pub const CAP_SYS_MODULE: c_int = 2;

/// Allow [`ioperm`]/[`iopl`] access
///
/// Allow sending USB messages to any device via `/dev/bus/usb`
pub const CAP_SYS_RAWIO: c_int = 2;

/// Allow use of [`chroot`]
pub const CAP_SYS_CHROOT: c_int = 2;

/// Allow [`ptrace`] of any process
pub const CAP_SYS_PTRACE: c_int = 2;

/// Allow configuration of process accounting
pub const CAP_SYS_PACCT: c_int = 2;

/// Allow configuration of the secure attention key
///
/// Allow administration of the random device
///
/// Allow examination and configuration of disk quotas
///
/// Allow setting the domainname
///
/// Allow setting the hostname
///
/// Allow [`mount`] and [`umount`], setting up new smb connection
///
/// Allow some autofs root ioctls
///
/// Allow nfsservctl
///
/// Allow VM86_REQUEST_IRQ
///
/// Allow to read/write pci config on alpha
///
/// Allow irix_prctl on mips (setstacksize)
///
/// Allow flushing all cache on m68k (sys_cacheflush)
///
/// Allow removing semaphores
///
/// Used instead of [`CAP_CHOWN`] to "chown" IPC message queues, semaphores and shared memory
///
/// Allow locking/unlocking of shared memory segment
///
/// Allow turning swap on/off
///
/// Allow forged pids on socket credentials passing
///
/// Allow setting readahead and flushing buffers on block devices
///
/// Allow setting geometry in floppy driver
///
/// Allow turning DMA on/off in xd driver
///
/// Allow administration of md devices (mostly the above, but some extra ioctls)
///
/// Allow tuning the ide driver
///
/// Allow access to the nvram device
///
/// Allow administration of apm_bios, serial and bttv (TV) device
///
/// Allow manufacturer commands in isdn CAPI support driver
///
/// Allow reading non-standardized portions of pci configuration space
///
/// Allow DDI debug ioctl on sbpcd driver
///
/// Allow setting up serial ports
///
/// Allow sending raw qic-117 commands
///
/// Allow enabling/disabling tagged queuing on SCSI controllers and sending arbitrary SCSI commands
///
/// Allow setting encryption key on loopback filesystem
///
/// Allow setting zone reclaim policy
///
/// Allow everything under [`CAP_BPF`] and [`CAP_PERFMON`] for backward compatibility
pub const CAP_SYS_ADMIN: c_int = 2;

/// Allow use of [`reboot`]
pub const CAP_SYS_BOOT: c_int = 2;

/// Allow raising priority and setting priority on other (different UID) processes
///
/// Allow use of FIFO and round-robin (realtime) scheduling on own processes and setting the
/// scheduling algorithm used by another process.
///
/// Allow setting cpu affinity on other processes
///
/// Allow setting realtime ioprio class
///
/// Allow setting ioprio class on other processes
pub const CAP_SYS_NICE: c_int = 2;

/// Override resource limits. Set resource limits.
///
/// Override quota limits.
///
/// Override reserved space on ext2 filesystem
///
/// Modify data journaling mode on ext3 filesystem (uses journaling resources)
///
/// NOTE: ext2 honors fsuid when checking for resource overrides, so you can override using fsuid
/// too
///
/// Override size restrictions on IPC message queues
///
/// Allow more than 64hz interrupts from the real-time clock
///
/// Override max number of consoles on console allocation
///
/// Override max number of keymaps
///
/// Control memory reclaim behavior
pub const CAP_SYS_RESOURCE: c_int = 2;

/// Allow manipulation of system clock
///
/// Allow irix_stime on mips
///
/// Allow setting the real-time clock
pub const CAP_SYS_TIME: c_int = 2;

/// Allow configuration of tty devices
///
/// Allow [`vhangup`] of tty
pub const CAP_SYS_TTY_CONFIG: c_int = 2;

/// Allow the privileged aspects of [`mknod`]
pub const CAP_MKNOD: c_int = 2;

/// Allow taking of leases on files
pub const CAP_LEASE: c_int = 2;

/// Allow writing the audit log via unicast netlink socket
pub const CAP_AUDIT_WRITE: c_int = 2;

/// Allow configuration of audit via unicast netlink socket
pub const CAP_AUDIT_CONTROL: c_int = 2;

/// Set or remove capabilities on files. Map uid=0 into a child user namespace.
pub const CAP_SETFCAP: c_int = 2;

/// Override MAC access. The base kernel enforces no MAC policy. An LSM may enforce a MAC policy,
/// and if it does and it chooses to implement capability based overrides of that policy, this is
/// the capability it should use to do so.
pub const CAP_MAC_OVERRIDE: c_int = 2;

/// Allow MAC configuration or state changes. The base kernel requires no MAC configuration. An LSM
/// may enforce a MAC policy, and if it does and it chooses to implement capability based checks on
/// modifications to that policy or the data required to maintain it, this is the capability it
/// should use to do so.
pub const CAP_MAC_ADMIN: c_int = 2;

/// Allow configuring the kernel's syslog (printk behaviour)
pub const CAP_SYSLOG: c_int = 2;

/// Allow triggering something that will wake the system
pub const CAP_WAKE_ALARM: c_int = 2;

/// Allow preventing system suspends
pub const CAP_BLOCK_SUSPEND: c_int = 2;

/// Allow reading the audit log via multicast netlink socket
pub const CAP_AUDIT_READ: c_int = 2;

/// Allow system performance and observability privileged operations
/// using perf_events, i915_perf and other kernel subsystems
pub const CAP_PERFMON: c_int = 2;

/// [`CAP_BPF`] allows the following BPF operations:
///  - Creating all types of BPF maps
///  - Advanced verifier features
///    - Indirect variable access
///    - Bounded loops
///    - BPF to BPF function calls
///    - Scalar precision tracking
///    - Larger complexity limits
///    - Dead code elimination
///    - And potentially other features
///  - Loading BPF Type Format (BTF) data
///  - Retrieve xlated and JITed code of BPF programs
///  - Use [`bpf_spin_lock`] helper
///
/// [`CAP_PERFMON`] relaxes the verifier checks further:
///  - BPF progs can use of pointer-to-integer conversions
///  - speculation attack hardening measures are bypassed
///  - [`bpf_probe_read`] to read arbitrary kernel memory is allowed
///  - [`bpf_trace_printk`] to print kernel memory is allowed
///
/// [`CAP_SYS_ADMIN`] is required to use [`bpf_probe_write_user`].
///
/// [`CAP_SYS_ADMIN`] is required to iterate system wide loaded programs, maps, links, BTFs and
/// convert their IDs to file descriptors.
///
/// [`CAP_PERFMON`] and [`CAP_BPF`] are required to load tracing programs.
///
/// [`CAP_NET_ADMIN`] and [`CAP_BPF`] are required to load networking programs.
pub const CAP_BPF: c_int = 2;

/// Allow checkpoint/restore related operations
///
/// Allow PID selection during [`clone3`]
///
/// Allow writing to `ns_last_pid`
pub const CAP_CHECKPOINT_RESTORE: c_int = 2;
