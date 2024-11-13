//! WIP

use core::{num::NonZeroU32, ptr::NonNull, sync::atomic::AtomicU32};
use std::sync::Arc;

use aya_obj::generated::{
    xdp_desc, xdp_mmap_offsets, xdp_ring_offset, xdp_statistics, xsk_umem_config,
};
use libc::SOL_XDP;

mod iface;
mod ring;
mod socket;
mod umem;
mod user;

pub use self::user::{ReadComplete, ReadRx, WriteFill, WriteTx};

/// WIP
pub type XdpDesc = xdp_desc;

/// WIP
#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct XdpUmemReg {
    /// WIP
    pub addr: u64,
    /// WIP
    pub len: u64,
    /// WIP
    pub chunk_size: u32,
    /// WIP
    pub headroom: u32,
    /// WIP
    pub flags: u32,
    /// WIP
    pub tx_metadata_len: u32,
}

/// WIP
pub type XdpMmapOffsets = xdp_mmap_offsets;

/// WIP
pub type XdpRingOffsets = xdp_ring_offset;

// XSK
/// Internal structure shared for all rings.
///
/// TODO: copied from <xdp.h>, does everything make sense in Rust?
#[repr(C)]
#[derive(Debug)]
struct XskRing {
    /// _owned_ version of the producer head, may lag.
    cached_producer: u32,
    /// _owned_ version of the consumer head, may lag.
    cached_consumer: u32,
    /// Bit mask to quickly validate/force entry IDs.
    mask: u32,
    /// Number of entries (= mask + 1).
    size: u32,
    /// The mmaped-producer base.
    ///
    /// Note: Using lifetime static here, but we point into an `mmap` area and it is important that
    /// we do not outlive the binding. The constructor promises this.
    producer: &'static AtomicU32,
    /// The mmaped-consumer base.
    consumer: &'static AtomicU32,
    /// The mmaped-consumer ring control base.
    ring: NonNull<core::ffi::c_void>,
    /// The mmaped-consumer flags base.
    flags: NonNull<u32>,
}

/// WIP
pub struct UmemConfig {
    inner: xsk_umem_config,
}

/// WIP
pub struct SocketFd(libc::c_int);

/// WIP
#[derive(Debug, Default, Clone)]
pub struct SocketConfig {
    /// The number of receive descriptors in the ring.
    pub rx_size: Option<NonZeroU32>,
    /// The number of transmit descriptors in the ring.
    pub tx_size: Option<NonZeroU32>,
    /// Additional flags to pass to the `bind` call as part of `sockaddr_xdp`.
    pub bind_flags: u16,
}

/// WIP
#[repr(C)]
#[derive(Default)]
pub struct SockAddrXdp {
    /// WIP
    sxdp_family: u16,
    /// WIP
    sxdp_flags: u16,
    /// WIP
    sxdp_ifindex: u32,
    /// WIP
    sxdp_queue_id: u32,
    /// WIP
    sxdp_shared_umem_fd: u32,
}

/// WIP
#[allow(dead_code)]
pub struct XdpStatistics {
    /// WIP
    inner: xdp_statistics,
}

impl Default for XdpStatistics {
    fn default() -> Self {
        Self {
            inner: xdp_statistics {
                rx_dropped: u64::default(),
                rx_invalid_descs: u64::default(),
                tx_invalid_descs: u64::default(),
                rx_ring_full: u64::default(),
                rx_fill_ring_empty_descs: u64::default(),
                tx_ring_empty_descs: u64::default(),
            },
        }
    }
}

/// Prior version of XdpMmapOffsets (<= Linux 5.3).
#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct XdpRingOffsetsV1 {
    /// the relative address of the producer.
    pub producer: u64,
    /// the relative address of the consumer.
    pub consumer: u64,
    /// the relative address of the descriptor.
    pub desc: u64,
}

/// Prior version of XdpMmapOffsets (<= Linux 5.3).
#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct XdpMmapOffsetsV1 {
    /// Offsets for the receive ring (kernel produced).
    pub rx: XdpRingOffsetsV1,
    /// Offsets for the transmit ring (user produced).
    pub tx: XdpRingOffsetsV1,
    /// Offsets for the fill ring (user produced).
    pub fr: XdpRingOffsetsV1,
    /// Offsets for the completion ring (kernel produced).
    pub cr: XdpRingOffsetsV1,
}

/// WIP
#[derive(Debug)]
pub struct SocketMmapOffsets {
    /// WIP
    inner: XdpMmapOffsets,
}

impl Default for SocketMmapOffsets {
    fn default() -> Self {
        Self {
            inner: XdpMmapOffsets {
                rx: XdpRingOffsets {
                    producer: u64::default(),
                    consumer: u64::default(),
                    desc: u64::default(),
                    flags: u64::default(),
                },
                tx: XdpRingOffsets {
                    producer: u64::default(),
                    consumer: u64::default(),
                    desc: u64::default(),
                    flags: u64::default(),
                },
                fr: XdpRingOffsets {
                    producer: u64::default(),
                    consumer: u64::default(),
                    desc: u64::default(),
                    flags: u64::default(),
                },
                cr: XdpRingOffsets {
                    producer: u64::default(),
                    consumer: u64::default(),
                    desc: u64::default(),
                    flags: u64::default(),
                },
            },
        }
    }
}

/// WIP
pub struct Umem {
    /// WIP
    umem_buffer: NonNull<[u8]>,
    /// WIP
    config: UmemConfig,
    /// WIP
    fd: Arc<SocketFd>,
    /// WIP
    devices: DeviceControl,
}

/// WIP
#[derive(Clone, Copy, Debug)]
pub struct UmemChunk {
    /// WIP
    pub addr: NonNull<[u8]>,
    /// WIP
    pub offset: u64,
}

/// WIP
#[derive(Clone)]
struct DeviceControl {
    /// WIP
    inner: Arc<dyn ControlSet>,
}

#[allow(dead_code)]
trait ControlSet: Send + Sync + 'static {
    fn insert(&self, _: IfCtx) -> bool;
    fn contains(&self, _: &IfCtx) -> bool;
    fn remove(&self, _: &IfCtx);
}

/// WIP
pub struct Socket {
    /// WIP
    info: Arc<IfInfo>,
    /// WIP
    fd: Arc<SocketFd>,
}

/// One device queue associated with an XDP socket.
///
/// A socket is more specifically a set of receive and transmit queues for packets (mapping to some
/// underlying hardware mapping those bytes with a network). The fill and completion queue can, in
/// theory, be shared with other sockets of the same `Umem`.
pub struct DeviceQueue {
    /// Fill and completion queues.
    fcq: DeviceRings,
    /// This is also a socket.
    socket: Socket,
    /// Reference to de-register.
    devices: DeviceControl,
}

/// An owner of receive/transmit queues.
///
/// This represents a configured version of the raw `Socket`. It allows you to map the required
/// rings and _then_ [`Umem::bind`] the socket, enabling the operations of the queues with the
/// interface.
pub struct User {
    /// A clone of the socket it was created from.
    socket: Socket,
    /// The configuration with which it was created.
    config: Arc<SocketConfig>,
    /// A cached version of the map describing receive/tranmit queues.
    map: SocketMmapOffsets,
}

/// A receiver queue.
///
/// This also maintains the mmap of the associated queue.
// Implemented in <xsk/user.rs>
pub struct RingRx {
    ring: RingCons,
    fd: Arc<SocketFd>,
}

/// A transmitter queue.
///
/// This also maintains the mmap of the associated queue.
// Implemented in <xsk/user.rs>
pub struct RingTx {
    ring: RingProd,
    fd: Arc<SocketFd>,
}

/// A complete (cached) information about a socket.
///
/// Please allocate this, the struct is quite large. For instance, put it into an `Arc` as soon as
/// it is no longer mutable, or initialize it in-place with [`Arc::get_mut`].
#[derive(Clone, Copy)]
pub struct IfInfo {
    ctx: IfCtx,
    ifname: [libc::c_char; libc::IFNAMSIZ],
}

/// Reduced version of `IfCtx`, only retaining numeric IDs for the kernel.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct IfCtx {
    ifindex: libc::c_uint,
    queue_id: u32,
    /// The namespace cookie, associated with a *socket*.
    /// This field is filled by some surrounding struct containing the info.
    netnscookie: u64,
}

pub(crate) struct DeviceRings {
    pub prod: RingProd,
    pub cons: RingCons,
    // Proof that we obtained this. Not sure if and where we'd use it.
    #[allow(dead_code)]
    pub(crate) map: SocketMmapOffsets,
}

/// An index to an XDP buffer.
///
/// Usually passed from a call of reserved or available buffers(in [`RingProd`] and
/// [`RingCons`] respectively) to one of the access functions. This resolves the raw index to a
/// memory address in the ring buffer.
///
/// This is _not_ a pure offset, a masking is needed to access the raw offset! The kernel requires
/// the buffer count to be a power-of-two for this to be efficient. Then, producer and consumer
/// heads operate on the 32-bit number range, _silently_ mapping to the same range of indices.
/// (Similar to TCP segments, actually). Well-behaving sides will maintain the order of the two
/// numbers in this wrapping space, which stays perfectly well-defined as long as less than `2**31`
/// buffer are identified in total.
///
/// In other words, you need a configured ring to determine an exact offset or compare two indices.
///
/// This type does _not_ implement comparison traits or hashing! Nevertheless, there's nothing
/// unsafe about creating or observing this detail, so feel free to construct your own or use the
/// transparent layout to (unsafely) treat the type as a `u32` instead.
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BufIdx(pub u32);

/// A producer ring.
///
/// Here, user space maintains the write head and the kernel the read tail.
#[derive(Debug)]
pub struct RingProd {
    inner: XskRing,
    mmap_addr: NonNull<[u8]>,
}

/// A consumer ring.
///
/// Here, kernel maintains the write head and user space the read tail.
#[derive(Debug)]
pub struct RingCons {
    inner: XskRing,
    mmap_addr: NonNull<[u8]>,
}

impl Default for UmemConfig {
    fn default() -> Self {
        Self {
            inner: xsk_umem_config {
                fill_size: 1 << 11,
                comp_size: 1 << 11,
                frame_size: 1 << 12,
                frame_headroom: 0,
                flags: 0,
            },
        }
    }
}

impl Drop for SocketFd {
    fn drop(&mut self) {
        let _ = unsafe { libc::close(self.0) };
    }
}

// FIXME: pending stabilization, use pointer::len directly.
// <https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.len>
//
// FIXME: In 1.79 this was stabilized. Bump MSRV fine?
fn ptr_len(ptr: *mut [u8]) -> usize {
    unsafe { (*(ptr as *mut [()])).len() }
}

impl Socket {
    /// Get the raw file descriptor number underlying this socket.
    pub fn as_raw_fd(&self) -> i32 {
        self.fd.0
    }
}

impl User {
    /// Get the raw file descriptor number underlying this socket.
    pub fn as_raw_fd(&self) -> i32 {
        self.socket.as_raw_fd()
    }
}
