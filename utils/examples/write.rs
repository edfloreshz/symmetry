use anyhow::Result;
use symmetry_utils::color_scheme::ColorScheme;
use symmetry_utils::configuration::Configuration;

fn main() -> Result<()> {
    let mut config = Configuration::new();
    config.init("".into())?;
    config.color_scheme = ColorScheme::new();
    config.write()?;
    Ok(())
}
