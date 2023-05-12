use anyhow::Result;
use symmetry_core::color_scheme::ColorScheme;
use symmetry_core::configuration::Configuration;

fn main() -> Result<()> {
    let mut config = Configuration::new();
    config.init()?;
    config.color_scheme = ColorScheme::new();
    config.write()?;
    Ok(())
}
