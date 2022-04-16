use anyhow::{Context, Result};
use std::fs;

pub trait ChargerBase {
    fn path(&self) -> &'static str;

    fn enable_charger(&self, en: bool) -> Result<()> {
        let v = if en { 1 } else { 0 };

        write_u32_property(format!("{}{}", self.path(), "/f_chg_config").as_str(), v)
    }

    fn set_current(&self, c: u32) -> Result<()> {
        write_u32_property(
            format!("{}{}", self.path(), "/constant_charge_current").as_str(),
            c * 1000,
        )
    }

    fn set_voltage(&self, v: u32) -> Result<()> {
        write_u32_property(
            format!("{}{}", self.path(), "/constant_charge_voltage").as_str(),
            v * 1000,
        )
    }
}

fn write_u32_property(path: &str, value: u32) -> Result<()> {
    fs::write(path, value.to_string()).context(format!("Failed to write {}", path))?;

    Ok(())
}
