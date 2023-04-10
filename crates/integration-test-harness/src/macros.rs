#[cfg(feature = "macros")]
#[macro_export]
macro_rules! cli_bin_path {
    () => {
        env!("CARGO_BIN_EXE_cloudtruth")
    };
}

/// Creates a cloudtruth command.
/// Example:
///     cloudtruth!("projects list")
///
/// Also accepts format! syntax:
///     cloudtruth!("projects set {name}", name=expr)
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! cloudtruth {
    ($($fmt:tt)*) => (
        $crate::command::commandify(format!("{} {}", cli_bin_path!(), format!($($fmt)*)))
            .wrap_err_with(|| format!("Invalid command: cloudtruth {}", format!($($fmt)*)))
            .unwrap()
    )
}

/// Helper macro equivalent to contains(format!(...))
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! contains {
    ($($fmt:tt)*) => (
        contains(format!($($fmt)*))
    )
}

/// Helper macro equivalent to diff(format!(...))
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! diff {
    ($($fmt:tt)*) => (
        contains(format!($($fmt)*))
    )
}