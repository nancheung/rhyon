#[macro_export]
macro_rules! load_section {
    ($section:ident, $type:ty) => {{
        use serde::Deserialize;
        use $crate::core::env;

        #[derive(Deserialize, Debug)]
        struct Config {
            $section: $type,
        }

        let wrapper: Config = env::load_config();
        wrapper.$section
    }};
}
