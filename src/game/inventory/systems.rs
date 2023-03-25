use bevy::prelude::*;
use rand::{thread_rng, Rng, rngs::ThreadRng};


use super::{components::{Inventory, InventoryMarker}, card_components::{Card, CardType}};

pub const INVENTORY_SIZE: i32 = 3;


pub fn maintain_inventory(mut commands: Commands, mut query: Query<&mut Inventory>) {
    for mut inventory in query.iter_mut() {
        let needed_cards = INVENTORY_SIZE - (inventory.cards.len() as i32);
        for _ in 0..needed_cards {
            inventory.cards.push(draw_card());
        }

    }
}
pub fn init_inventory(mut commands: Commands) {
    println!("Init Inventory");
    commands.spawn((Inventory{ cards: vec![] }, InventoryMarker {}));
}

fn draw_card() -> Card {
    let cards = vec![
        Card {
            card_type: CardType::ATTACK,
            name: "1".to_string(),
            description: "A".to_string(),
            cost: 1,
            sprite_path: "sprites/cards/attacks/blank_attack.png".to_string(),
        },
        Card {
            card_type: CardType::ATTACK,
            name: "2".to_string(),
            description: "B".to_string(),
            cost: 2,
            sprite_path: "sprites/cards/attacks/blank_attack.png".to_string(),
        },
        Card {
            card_type: CardType::ATTACK,
            name: "3".to_string(),
            description: "C".to_string(),
            cost: 3,
            sprite_path: "sprites/cards/attacks/blank_attack.png".to_string(),
        },
        Card {
            card_type: CardType::ENEMY,
            name: "4".to_string(),
            description: "D".to_string(),
            cost: 4,
            sprite_path: "sprites/cards/enemies/blank_enemy.png".to_string(),
        }
    ];

    let mut rng: ThreadRng = thread_rng();
    let index = rng.gen_range(0..cards.len());
    let card = cards[index].clone();
    return card;

}

pub fn print_inventory(query: Query<&Inventory>) {
    for inventory in query.iter() {
       // println!("{:?}", inventory.cards);
    }
}

