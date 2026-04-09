use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, Ordering};

// ---------------------------------------------------------------------------
// Claimed<T> — private proof-of-ownership token
// ---------------------------------------------------------------------------

/// Proof that a successful CAS has granted exclusive access to `*mut T`.
///
/// This type is the key to avoiding `clippy::mut_from_ref`: the `unsafe`
/// pointer-to-reference conversion lives in [`into_ref`](Self::into_ref)
/// which consumes `self` by value (not `&self`), so no method ever has
/// both a `&self` parameter *and* a `&mut T` return with an `unsafe` block.
struct Claimed<T: 'static>(*mut T);

impl<T> Claimed<T> {
    /// Consume the proof token and yield the exclusive static reference.
    #[inline(always)]
    fn into_ref(self) -> &'static mut T {
        // SAFETY: The only way to construct `Claimed` is via a successful
        // `compare_exchange`, guaranteeing exclusive access for `'static`.
        unsafe { &mut *self.0 }
    }
}

// ---------------------------------------------------------------------------
// StaticCell<T>
// ---------------------------------------------------------------------------

/// Statically allocated, runtime-initialized single-write container.
///
/// Created empty. Calling [`init`](Self::init) (or a sibling method) fills
/// it **exactly once**, returning a `&'static mut T` valid for the remainder
/// of the program.
///
/// # Example
///
/// ```ignore
/// use pico_rs::kit::cell::StaticCell;
///
/// static USB_BUF: StaticCell<[u8; 512]> = StaticCell::new();
///
/// let buf: &'static mut [u8; 512] = USB_BUF.init([0u8; 512]);
/// ```
pub struct StaticCell<T> {
    used: AtomicBool,
    val: UnsafeCell<MaybeUninit<T>>,
}

// SAFETY: The atomic flag ensures at most one caller obtains `&mut T`.
// Once handed out the reference is never aliased.
unsafe impl<T> Send for StaticCell<T> {}
unsafe impl<T> Sync for StaticCell<T> {}

impl<T> StaticCell<T> {
    /// Create a new, empty `StaticCell`.
    #[inline]
    pub const fn new() -> Self {
        Self {
            used: AtomicBool::new(false),
            val: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    /// Attempt to claim the storage via an atomic CAS.
    ///
    /// Returns a [`Claimed`] proof token on success. No `unsafe` is needed
    /// here because [`UnsafeCell::get`] is a safe projection to `*mut T`.
    #[inline]
    fn try_claim(&'static self) -> Option<Claimed<MaybeUninit<T>>> {
        self.used
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .ok()
            .map(|_| Claimed(self.val.get()))
    }

    /// Initialize with `val`, returning `&'static mut T`.
    ///
    /// # Panics
    ///
    /// Panics if already initialized.
    #[inline]
    #[track_caller]
    pub fn init(&'static self, val: T) -> &'static mut T {
        self.uninit().write(val)
    }

    /// Initialize in-place via a closure, avoiding stack copies for large `T`.
    ///
    /// # Panics
    ///
    /// Panics if already initialized.
    #[inline]
    #[track_caller]
    pub fn init_with(&'static self, f: impl FnOnce() -> T) -> &'static mut T {
        self.uninit().write(f())
    }

    /// Obtain a mutable reference to the uninitialized storage for manual
    /// placement.
    ///
    /// # Panics
    ///
    /// Panics if already initialized.
    #[inline]
    #[track_caller]
    pub fn uninit(&'static self) -> &'static mut MaybeUninit<T> {
        match self.try_uninit() {
            Some(v) => v,
            None => panic!("StaticCell already initialized"),
        }
    }

    /// Try to initialize; returns `None` if already full.
    #[inline]
    pub fn try_init(&'static self, val: T) -> Option<&'static mut T> {
        Some(self.try_uninit()?.write(val))
    }

    /// Try to initialize in-place via a closure; returns `None` if already
    /// full.
    #[inline]
    pub fn try_init_with(&'static self, f: impl FnOnce() -> T) -> Option<&'static mut T> {
        Some(self.try_uninit()?.write(f()))
    }

    /// Try to obtain the uninitialized storage; returns `None` if already
    /// taken.
    #[inline]
    pub fn try_uninit(&'static self) -> Option<&'static mut MaybeUninit<T>> {
        self.try_claim().map(Claimed::into_ref)
    }
}

impl<T> Default for StaticCell<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// ConstStaticCell<T>
// ---------------------------------------------------------------------------

/// Statically allocated and const-initialized, taken at runtime.
///
/// Unlike [`StaticCell`], the value is provided at **compile time**. At
/// runtime [`take`](Self::take) hands out a `&'static mut T` exactly once.
///
/// Zero-initialized values (e.g. `[0u8; 4096]`) are placed in `.bss` and
/// consume no flash.
///
/// # Example
///
/// ```ignore
/// use pico_rs::kit::cell::ConstStaticCell;
///
/// static DMA_BUF: ConstStaticCell<[u8; 4096]> = ConstStaticCell::new([0u8; 4096]);
///
/// let buf: &'static mut [u8; 4096] = DMA_BUF.take();
/// ```
pub struct ConstStaticCell<T> {
    taken: AtomicBool,
    val: UnsafeCell<T>,
}

// SAFETY: The value is effectively *sent* to whichever context calls `take`.
// `T: Send` is required so that the transfer is sound.
unsafe impl<T: Send> Send for ConstStaticCell<T> {}
unsafe impl<T: Send> Sync for ConstStaticCell<T> {}

impl<T> ConstStaticCell<T> {
    /// Create a new `ConstStaticCell` with the given compile-time value.
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            taken: AtomicBool::new(false),
            val: UnsafeCell::new(value),
        }
    }

    /// Attempt to claim the storage via an atomic CAS.
    #[inline]
    fn try_claim(&'static self) -> Option<Claimed<T>> {
        self.taken
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .ok()
            .map(|_| Claimed(self.val.get()))
    }

    /// Take the value, returning `&'static mut T`.
    ///
    /// # Panics
    ///
    /// Panics if already taken.
    #[inline]
    #[track_caller]
    pub fn take(&'static self) -> &'static mut T {
        match self.try_take() {
            Some(v) => v,
            None => panic!("ConstStaticCell already taken"),
        }
    }

    /// Try to take the value; returns `None` if already taken.
    #[inline]
    pub fn try_take(&'static self) -> Option<&'static mut T> {
        self.try_claim().map(Claimed::into_ref)
    }
}

// ---------------------------------------------------------------------------
// make_static!
// ---------------------------------------------------------------------------

/// Allocate a value in a hidden `static` and return `&'static mut T`.
///
/// Each **call-site** expands to its own `static StaticCell`, so each site
/// can only be executed once (a second execution will panic).
///
/// # Examples
///
/// ```ignore
/// use pico_rs::make_static;
///
/// // Explicit type + value
/// let buf: &'static mut [u8; 256] = make_static!([u8; 256], [0u8; 256]);
///
/// // Explicit type + closure (in-place init, avoids stack copy)
/// let big: &'static mut [u8; 8192] = make_static!([u8; 8192], || [0u8; 8192]);
/// ```
#[macro_export]
macro_rules! make_static {
    ($t:ty, || $body:expr) => {{
        static CELL: $crate::kit::cell::StaticCell<$t> = $crate::kit::cell::StaticCell::new();
        CELL.init_with(|| $body)
    }};
    ($t:ty, $val:expr) => {{
        static CELL: $crate::kit::cell::StaticCell<$t> = $crate::kit::cell::StaticCell::new();
        CELL.init($val)
    }};
}
