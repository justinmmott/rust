use crate::ops::{ControlFlow, Try};

/// The `??` operator
///
/// Can be implemented on types that implement the try trait

#[unstable(feature = "try_coalescing_v1", issue = "N/A")]
#[rustc_on_unimplemented(on(
    all(from_desugaring = "DoubleQuestionMark"),
    message = "the `??` operator can only be applied to values that implement `{TryCoalescing}`",
    label = "the `??` operator cannot be applied to type `{Self}`"
))]
#[doc(alias = "??")]
#[lang = "TryCoalescing"]
#[const_trait]
pub trait TryCoalescing<Rhs = Self::Output>: Try {
  #[lang = "try_coalescing"]
  fn try_coalescing(self, rhs: Self::Output) =>  ControlFlow<Self::Output, Self::Output>  {
    match self {
      Some(v) => ControlFlow::Continue(v),
      None => ControlFlow::Continue(rhs),
    }
  }
}
