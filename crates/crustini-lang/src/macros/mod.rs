pub mod app;
pub mod block;
pub mod input;
pub mod screen;

#[cfg(test)]
mod tests {
    use super::app::{AppMacro, APP_MACROS};
    use super::block::{BlockMacro, BLOCK_MACROS};
    use super::input::{field_name, INPUT_NAMES};
    use super::screen::{ScreenArgs, ScreenMacro, SCREEN_MACROS};
    use std::collections::HashSet;

    #[test]
    fn app_macro_registry_parses_all_names() {
        assert_unique(APP_MACROS.iter().map(|spec| spec.name));

        for spec in APP_MACROS {
            assert_eq!(AppMacro::parse(spec.name), Some(spec.kind));
            assert_eq!(spec.kind.name(), spec.name);
        }
    }

    #[test]
    fn block_macro_registry_parses_all_names() {
        assert_unique(BLOCK_MACROS.iter().map(|spec| spec.name));

        for spec in BLOCK_MACROS {
            assert_eq!(BlockMacro::parse(spec.name), Some(spec.kind));
            assert_eq!(spec.kind.name(), spec.name);
        }
    }

    #[test]
    fn screen_macro_registry_has_lowering_metadata() {
        assert_unique(SCREEN_MACROS.iter().map(|spec| spec.name));

        for spec in SCREEN_MACROS {
            assert_eq!(ScreenMacro::parse(spec.name), Some(spec.kind));
            assert_eq!(spec.kind.spec(), spec);
        }

        let pixel = ScreenMacro::parse("pixel").unwrap();
        assert_eq!(pixel.rust_method(), "set");
        assert_eq!(pixel.spec().args, ScreenArgs::Exact(3));

        let text = ScreenMacro::parse("text").unwrap();
        assert_eq!(text.rust_method(), "text");
        assert_eq!(text.spec().args, ScreenArgs::Exact(4));
    }

    #[test]
    fn input_registry_maps_aliases_to_buttons_fields() {
        let aliases = INPUT_NAMES
            .iter()
            .flat_map(|spec| spec.aliases.iter().copied());
        assert_unique(aliases);

        for spec in INPUT_NAMES {
            for alias in spec.aliases {
                assert_eq!(field_name(alias), Some(spec.field));
            }
        }

        assert_eq!(field_name("mousex"), Some("mouse_x"));
        assert_eq!(field_name("mousey"), Some("mouse_y"));
        assert_eq!(field_name("mousedown"), Some("mouse_down"));
    }

    fn assert_unique<'a>(items: impl IntoIterator<Item = &'a str>) {
        let mut seen = HashSet::new();

        for item in items {
            assert!(
                seen.insert(item),
                "duplicate macro registry name `{}`",
                item
            );
        }
    }
}
