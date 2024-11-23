
/// intermediary versions of page objects, these are easier
/// to work with when constructing a page from source but
/// are not able to be "const" variables
pub mod intermed;

/// structs / enums that do not need separate implementations
/// for intermediary and final implementations
pub mod common;

/// module defining implementations for converting between
/// intermediary and final versions of the struct, this also
/// makes it easier to dispose of &'_ [T] as they are converted
/// into Vec<T>
pub mod convert;

/// module implementing ToTokens for all finalized types, allowing
/// marker_macro to simply invoke the trait's functions for its
/// functionality
pub mod tokenize;

/// module defining generalized errors
pub mod errors;
