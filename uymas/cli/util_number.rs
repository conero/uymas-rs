/// 文件大小
/// ### Example
/// ```
/// use uymas_cli::util_number::size::Size;
///
/// let byte = 10993038444f64;
/// println!("{} => {}", byte, byte.human_size());
///
/// let byte_u64 = 10993038444u64;
/// println!("{} => {}", byte_u64, byte_u64.human_size());
/// ```
pub mod size;
