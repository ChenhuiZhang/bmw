use anyhow::{Context, Result};
use std::fs;

pub trait GaugeBase {
    fn path(&self) -> &'static str;

    fn get_capacity(&self) -> Result<u32> {
        read_u32_property(format!("{}{}", self.path(), "/capacity").as_str(), 1)
    }

    fn get_voltage(&self) -> Result<u32> {
        read_u32_property(format!("{}{}", self.path(), "/voltage_now").as_str(), 1)
    }

    fn get_current(&self) -> Result<i32> {
        read_i32_property(format!("{}{}", self.path(), "/current_now").as_str(), 1)
    }

    fn get_full_charge_capacity(&self) -> Result<u32> {
        read_u32_property(format!("{}{}", self.path(), "/charge_full").as_str(), 1000)
    }

    fn get_charge_now_capacity(&self) -> Result<u32> {
        read_u32_property(format!("{}{}", self.path(), "/charge_now").as_str(), 1000)
    }
}

pub trait GaugeAdvance: GaugeBase {
    fn get_time_to_full(&self) -> Result<u32> {
        read_u32_property(
            format!("{}{}", self.path(), "/time_to_full_now").as_str(),
            1,
        )
    }

    fn get_time_to_empty(&self) -> Result<u32> {
        read_u32_property(
            format!("{}{}", self.path(), "/time_to_empty_now").as_str(),
            1,
        )
    }

    fn get_cycle_count(&self) -> Result<u32> {
        read_u32_property(format!("{}{}", self.path(), "/cycle_count").as_str(), 1)
    }
}

fn read_i32_property(path: &str, factor: u32) -> Result<i32> {
    let s = fs::read_to_string(path).context(format!("Failed to read {}", path))?;

    let value = s
        .trim_end_matches('\n')
        .parse::<i32>()
        .context(format!("Failed to parse {} into i32", s))?;

    Ok(value / (factor as i32))
}

fn read_u32_property(path: &str, factor: u32) -> Result<u32> {
    let s = fs::read_to_string(path).context(format!("Failed to read {}", path))?;

    let value = s
        .trim_end_matches('\n')
        .parse::<u32>()
        .context(format!("Failed to parse {} into u32", s))?;

    Ok(value / factor)
}
