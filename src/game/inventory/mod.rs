use bevy::prelude::*;

pub mod components;
pub mod card_components;
pub mod card_systems;
mod systems;

use card_components::*;
use card_systems::*;
use systems::*;


pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_inventory.before(init_render_cards))
            .add_startup_system(init_render_cards)
            .add_system(maintain_inventory.before(debug_cards))
            .add_system(debug_cards)
            .add_system(print_inventory);
    }
}