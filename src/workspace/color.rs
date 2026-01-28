use std::str::FromStr;

use egui::Color32;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr)]
#[strum(serialize_all = "PascalCase")]
pub enum NamedColor {
    DarkPink,
    Pink,
    Magenta,
    LightPurple,
    Purple,
    Blue,
    Grey,
    LightBlue,
    Aqua,
    Green,
    Olive,
    Yellow,
    Orange,
    Red,
    DarkBlue,
}

impl NamedColor {
    pub fn hex_argb(&self) -> u32 {
        match self {
            Self::Aqua => 0x00FFFF,
            Self::Blue => 0x0000FF,
            Self::DarkBlue => 0x00008B,
            Self::Grey => 0x808080,
            Self::Green => 0x008000,
            Self::LightBlue => 0xADD8E6,
            Self::Magenta => 0xFF00FF,
            Self::Olive => 0x808000,
            Self::Orange => 0xFFA500,
            Self::Pink => 0xFFC0CB,
            Self::Purple => 0x800080,
            Self::Red => 0xFF0000,
            Self::Yellow => 0xFFFF00,
            Self::DarkPink => 0x916D74,
            Self::LightPurple => 0xF300F3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorValue {
    Named(NamedColor),
    Raw(u8, u8, u8),
}

impl Default for ColorValue {
    fn default() -> Self {
        Self::Named(NamedColor::Grey)
    }
}

impl ColorValue {
    pub fn to_egui_color(&self) -> Color32 {
        match self {
            Self::Named(val) => {
                let argb = val.hex_argb();
                let r = ((argb >> 16) & 0xFF) as u8;
                let g = ((argb >> 8) & 0xFF) as u8;
                let b = ((argb) & 0xFF) as u8;

                Color32::from_rgb(r, g, b)
            }
            Self::Raw(r, g, b) => Color32::from_rgb(*r, *g, *b),
        }
    }
}

impl Into<Color32> for ColorValue {
    fn into(self) -> Color32 {
        self.to_egui_color()
    }
}

impl Serialize for ColorValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Named(val) => serializer.serialize_str(val.as_ref()),
            Self::Raw(r, g, b) => serializer.serialize_str(format!("{}, {}, {}", r, g, b).as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for ColorValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        if let Ok(noesis_enum) = NamedColor::from_str(&s) {
            return Ok(ColorValue::Named(noesis_enum));
        }

        // "R, G, B" (e.g., "255, 128, 64")
        let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
        if parts.len() == 3 {
            let r = parts[0].parse::<u8>().map_err(serde::de::Error::custom)?;
            let g = parts[1].parse::<u8>().map_err(serde::de::Error::custom)?;
            let b = parts[2].parse::<u8>().map_err(serde::de::Error::custom)?;
            return Ok(ColorValue::Raw(r, g, b));
        }

        Err(serde::de::Error::custom(format!(
            "Invalid color format: '{}'. Expected Noesis name or 'R, G, B'",
            s
        )))
    }
}
