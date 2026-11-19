use extendr_api::prelude::*;

/// Sum of squares (Rust example)
///
/// An example function implemented in Rust via 'extendr'. Replace or delete
/// it. Roxygen tags go in `///` doc comments and are copied into
/// `R/extendr-wrappers.R` by `devtools::document()`.
///
/// @param x A numeric vector.
///
/// @returns A single number: the sum of the squared elements of `x`.
/// @export
///
/// @examples
/// sumSquaresRust(c(1, 2, 3))
// Rust names stay snake_case; `r_name` sets the camelCase name used in R.
#[extendr(r_name = "sumSquaresRust")]
fn sum_squares(x: &[f64]) -> f64 {
    x.iter().map(|v| v * v).sum()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod pkgname;
    fn sum_squares;
}
