/// Formats the given error using the Debug trait.
///
/// In addition, it also converts tabs to spaces to make
/// the returned string comparable to the string literals in the tests.
pub fn error_to_debug_string<E>(e: &E) -> String
where
    E: std::fmt::Debug,
{
    format!("{:?}", e).replace("\t", "    ")
}

/// Formats the given error using the Display trait.
///
/// In addition, it also converts tabs to spaces to make
/// the returned string comparable to the string literals in the tests.
pub fn error_to_display_string<E>(e: &E) -> String
where
    E: std::fmt::Display,
{
    format!("{}", e).replace("\t", "    ")
}

/// Format an error and all its sources.
pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
