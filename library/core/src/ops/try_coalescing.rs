use crate::ops::ControlFlow;

/// The `??` operator
///
/// Can be implemented on types that implement the try trait

#[unstable(feature = "try_coalescing_v2", issue = "N/A")]
#[rustc_on_unimplemented(on(
    all(from_desugaring = "DoubleQuestionMark"),
    message = "the `??` operator can only be applied to values that implement `{TryCoalescing}`",
    label = "the `??` operator cannot be applied to type `{Self}`"
))]
#[doc(alias = "??")]
#[lang = "TryCoalescing"]
#[const_trait]
pub trait TryCoalescing {}
