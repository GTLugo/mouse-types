#[cfg(feature = "serde")]
use serde::{
  Deserialize,
  Serialize,
};
use {
  crate::{
    button::MouseButton,
    state::ButtonState,
  },
  keyboard_types::Modifiers,
};

/// Mouse events are issued for all pressed and released mouse buttons.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MouseEvent {
  /// Whether the button is pressed or released.
  pub state: ButtonState,
  /// Logical button value.
  pub button: MouseButton,
  /// Flags for pressed modifier keys.
  pub modifiers: Modifiers,
}
