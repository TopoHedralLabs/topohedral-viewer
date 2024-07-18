//! This submodule provides a minimal colormap implementation
//..................................................................................................


use std::fmt;
use std::io;

use serde::de::DeserializeSeed;
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize,
};
use thiserror::Error;
//..................................................................................................

const VIRIDIS_JSON: &str = include_str!("colormaps/viridis.json");

struct ColorArrayVisitor;

impl<'de> Visitor<'de> for ColorArrayVisitor
{
    type Value = [[f32; 3]; 256];

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result
    {
        formatter.write_str("an array of 256 RGB color values")
    }

    fn visit_seq<A>(
        self,
        mut seq: A,
    ) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut colors = [[0.0; 3]; 256];
        for i in 0..256
        {
            let triple: [f32; 3] = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(i, &self))?;

            colors[i] = triple;
        }
        Ok(colors)
    }
}

impl<'de> DeserializeSeed<'de> for ColorArrayVisitor
{
    type Value = [[f32; 3]; 256];

    fn deserialize<D>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_seq(self)
    }
}
//..................................................................................................

struct ColormapVisitor;

impl<'de> Visitor<'de> for ColormapVisitor
{
    type Value = Colormap;

    fn expecting(
        &self,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        formatter.write_str("a JSON object with fields 'name' and 'colors'")
    }

    fn visit_map<A>(
        self,
        mut map: A,
    ) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        const FIELDS: &'static [&'static str] = &["name", "colors"];
        let mut name = None;
        let mut colors = None;
        while let Some(key) = map.next_key::<String>()?
        {
            match key.as_str()
            {
                "name" =>
                {
                    if name.is_some()
                    {
                        return Err(de::Error::duplicate_field("name"));
                    }
                    name = Some(map.next_value::<String>()?);
                }
                "colors" =>
                {
                    if colors.is_some()
                    {
                        return Err(de::Error::duplicate_field("colors"));
                    }
                    colors = Some(map.next_value_seed(ColorArrayVisitor)?);
                }
                _ => return Err(de::Error::unknown_field(&key, FIELDS)),
            }
        }
        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
        let colors = colors.ok_or_else(|| de::Error::missing_field("colors"))?;
        Ok(Colormap { name, colors })
    }
}
//..................................................................................................

/// This is a very simple Lin-Seg colormap, where the colors are linearly interpolated between the
/// given colors.
///
/// The colours are assumed to be linearly spaced therefore the spacing is defined
/// by the number of colors. The valid input range is $[0,1]$ values outside of this range
/// will simply be clamped to the valid range.
pub struct Colormap
{
    pub name: String,
    pub colors: [[f32; 3]; 256],
}

impl<'de> Deserialize<'de> for Colormap
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &["name", "colors"];
        deserializer.deserialize_struct("Colormap", FIELDS, ColormapVisitor)
    }
}

impl Colormap
{
    pub fn new(name: String) -> Result<Self, ColormapError>
    {
        match name.as_str() {
            "viridis" => Ok(serde_json::from_str(VIRIDIS_JSON)?),
            _ => Err(ColormapError::ColormapNotFoundError(name)),
        }
        
    }

    pub fn get_color(&self, value: f32) -> [f32; 3] {

        let clamped_value = value.max(0.0).min(1.0);
        let index = (clamped_value * 255.0) as usize;
        let color = self.colors[index];
        color
    }
}
//..................................................................................................

#[derive(Error, Debug)]
pub enum ColormapError
{
    #[error("Colormap not found: {0}")]
    ColormapNotFoundError(String), 
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Colormap failed to deserialize {0}")]
    DeserializationError(#[from] serde_json::Error),
}
//..................................................................................................

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn deserialization_test() {

        let colormap = Colormap::new("viridis".to_string());
        assert!(colormap.is_ok());
    }

    #[test]
    fn get_color_test() {

        let colormap = Colormap::new("viridis".to_string()).unwrap();

        assert_eq!(colormap.get_color(0.0), [0.267004, 0.004874, 0.329415]);

        let mut u = 0.0;
        let du = 1.0 / 255.0;

        for i in 0..256 {
            let color1 = colormap.get_color(u);
            let color2 = colormap.colors[i];
            assert_eq!(color1, color2);
            u += du;
        }
    }
}