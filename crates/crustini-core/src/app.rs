use crate::{Buttons, Screen};

/// Minimal app trait used by generated Crustini apps.
///
/// The trait is deliberately tiny and `no_std` friendly.
/// The generated app owns its state. The host/runtime owns the screen buffer.
pub trait CrustiniApp {
    fn setup(&mut self, _screen: &mut Screen<'_>) {}
    fn update(&mut self, _input: Buttons) {}
    fn draw(&mut self, _screen: &mut Screen<'_>) {}
}
