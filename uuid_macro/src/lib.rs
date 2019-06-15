use proc_macro_hack::proc_macro_hack;

/// Generate a UUID at compile time.
/// # Example
/// ```
/// use uuid_macro::uuid_v4;
///
/// let mut uuids = Vec::new();
///
/// for _ in 0..2 {
///     uuids.push(uuid_v4!());
/// }
///
/// assert_eq!(uuids[0], uuids[1]);
/// ```
#[proc_macro_hack]
pub use uuid_macro_impl::uuid_v4;
