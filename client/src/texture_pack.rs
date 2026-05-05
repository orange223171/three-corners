use std::collections::HashMap;

use core_3c::{board::Triangle, kit::Kit};
use sfml::graphics::RcTexture;

/// A texture pack
pub struct TexturePack {
    empty_triangle: RcTexture,
    building_texture_pack: HashMap<String, RcTexture>,
}

impl TexturePack {
    /// Returns empty texture pack
    pub fn new() -> Self {
        Self {
            empty_triangle: RcTexture::new().expect("Error to create empty triangle texture"),
            building_texture_pack: HashMap::new(),
        }
    }

    /// Returns texture pack load from files by names froms kit
    pub fn from_kit(kit: &Kit) -> Self {
        let mut texture_pack = Self::new();

        texture_pack.empty_triangle = RcTexture::from_file("client/sprites/triangle.png")
            .expect("Error to load client/sprites/triangle.png");

        kit.building_kit().iter().for_each(|x| {
            texture_pack.building_texture_pack.insert(
                x.0.clone(),
                RcTexture::from_file(
                    (String::from("client/sprites/buildings/") + x.0.as_str() + ".png").as_str(),
                )
                .expect(
                    (String::from("Error to load client/sprites/buildings/")
                        + x.0.as_str()
                        + ".png")
                        .as_str(),
                ),
            );
        });

        texture_pack
    }

    /// Returns texture for specified triangle
    pub fn texture(&self, triangle: &Triangle) -> Option<&RcTexture> {
        match triangle {
            Some(building) => self.building_texture_pack.get(&building.name),
            None => Some(&self.empty_triangle),
        }
    }
}
