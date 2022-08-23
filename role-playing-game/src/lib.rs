// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    fn new(level: u32) -> Player {
        Self {
            health: 100,
            mana: if level < 10 {None} else {Some(100)}, // If the Player's level is below 10, their mana should be None
            level: level
        }
    }

    pub fn revive(&self) -> Option<Player> {
        // If the revive method is called on a Player whose health is 1 or above, 
        // then the method should return None.
        match self.health {
            0 => Some(Player::new(self.level)), // The revive method should preserve the Player's level.
            _ => None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(x) => { 
                if x >= mana_cost {
                    self.mana = Some(x - mana_cost);
                    2 * mana_cost // Return cast spell damage (2x mana cost)
                } else {
                    // If the player has a mana pool but insufficient mana, 
                    // the method should not affect the pool, but instead return 0
                    0
                } 
            },
            None => {
                // If the player does not have access to a mana pool, 
                // attempting to cast the spell must decrease their health by the mana cost of the spell. 
                // The damage returned must be 0.
                self.health -= min(self.health, mana_cost); //if over damaged health can't under 0
                0
            }
        }
    }
}
