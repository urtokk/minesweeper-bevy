use crate::resources::tile::Tile;
use std::ops::{Deref, DerefMut};

/// Base tile map
#[derive(Debug, Default, Clone)]
pub struct TileMap {
    bomb_count: u16,
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    /// Generates an empty map
    pub fn empty(width: u16, height: u16) -> Self {
        let map = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Tile::Empty).collect())
            .collect();
        Self {
            bomb_count: 0,
            height,
            width,
            map,
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map ({}, {}) with {} bombs: \n",
            self.width, self.height, self.bomb_count
        );
        let line: String = (0..=(self.width + 1)).into_iter().map(|_| "-").collect();
        buffer = format!("{}{}\n", buffer, line);
        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }

    // Getter for 'width'
    pub fn width(&self) -> u16 {
        self.width
    }

    // Getter for 'height'
    pub fn height(&self) -> u16 {
        self.height
    }

    // Getter for 'bomb_count'
    pub fn bomb_count(&self) -> u16 {
        self.bomb_count
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

// TODO: https://dev.to/qongzi/bevy-minesweeper-part-2-1hi5 -- Bombs and neighbors
