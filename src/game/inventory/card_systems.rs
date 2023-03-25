use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_tweening::Lerp;

use crate::game::inventory::card_components::Card;
use crate::game::inventory::components::Inventory;

use super::card_components::CardType;

pub const IDLE_POSITIONS: [Vec3; 3] = [
    Vec3::new(75.0, 90.0, 9.01),
    Vec3::new(175.0, 95.0, 9.02),
    Vec3::new(275.0, 90.0, 9.03),
];

pub const HOVER_POSITIONS: [Vec3; 3] = [
    Vec3::new(75.0, 110.0, 9.11),
    Vec3::new(175.0, 115.0, 9.12),
    Vec3::new(275.0, 110.0, 9.13),
];

/*

    Z-Axis Meanings:
    10.0 is Camera
    9.0:
        9.1X: Hovered Cards
            9.11: Card1
            9.12: Card2
            9.13: Card3
        9.0X: Idle Cards
            9.01: Card1
            9.02: Card2
            9.03: Card3
    1.0 is Player
    0.91 is Attacks
    0.9 is Enemies
    0.0 is Background
*/

pub fn init_render_cards(
    mut commands: Commands,
    inventory_resource: Res<Inventory>,
    asset_server: Res<AssetServer>,
) {
    // get cards:
    let mut j: usize = 0;
    for card in &inventory_resource.cards {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(&card.sprite_path),
                transform: Transform::from_translation(IDLE_POSITIONS[j])
                    .with_scale(Vec3::new(2.5f32, 2.5f32, 2.5f32)),
                ..default()
            },
            Card {
                id: j as i8,
                ..default()
            },
        ));
        j += 1;
    }
}

pub fn card_handler(
    mut card_query: Query<(Entity, &mut Transform, &mut Sprite, &Card), With<Card>>,
    mut camera_query: Query<(&GlobalTransform, &mut Camera), With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    btn: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    mut inventory_resource: ResMut<Inventory>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let (camera_transform, camera) = camera_query.single_mut();

    if keys.any_just_released([KeyCode::Key1, KeyCode::Key2, KeyCode::Key3]) {
        let mut id: i8 = 0;
        if keys.just_released(KeyCode::Key1) {
            id = 0i8;
        } else if keys.just_released(KeyCode::Key2) {
            id = 1i8;
        } else if keys.just_released(KeyCode::Key3) {
            id = 2i8;
        }
        for (_, _, _, card) in card_query.iter() {
            if id == card.id {
                play_card(card);
                inventory_resource.cards.remove(card.id as usize);
            }
        }
        return;
    }
    

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if world_position.x > 400.0f32 || world_position.y > 200.0f32 {
            // mouse is too far
            update_card(&(40.0 / 255.0), &mut card_query, time);
        } else {
            // mouse is nearby
            update_card(&1.0f32, &mut card_query, time);
            let closest_card: i8 = find_closest_card(
                &mut card_query,
                Vec3::new(world_position.x, world_position.y, 9.0f32),
            );
            if btn.just_pressed(MouseButton::Left) {
                for (_, _, _, card) in card_query.iter() {
                    if card.id == closest_card {
                        play_card(card);

                        inventory_resource.cards.remove(card.id as usize);
                    }
                }
            }
        }
    }
}

fn update_card(
    new_transparency: &f32,
    card_query: &mut Query<(Entity, &mut Transform, &mut Sprite, &Card), With<Card>>,
    time: Res<Time>,
) {
    for (_, mut transform, mut sprite, card) in card_query.iter_mut() {
        let a = sprite.color.a();
        sprite
            .color
            .set_a(a.lerp(new_transparency, &(12.0 * time.delta_seconds())));
        transform.translation = IDLE_POSITIONS[card.id as usize];
    }
}

fn find_closest_card<'a>(
    card_query: &mut Query<(Entity, &mut Transform, &mut Sprite, &Card), With<Card>>,
    target_position: Vec3,
) -> i8 {
    if let Some((_, mut transform, _, card)) =
        card_query.iter_mut().min_by(|(_, a, _, _), (_, b, _, _)| {
            let distance_a = a.translation.distance_squared(target_position);
            let distance_b = b.translation.distance_squared(target_position);
            distance_a.partial_cmp(&distance_b).unwrap()
        })
    {
        if (transform.translation.x - target_position.x).abs() <= 60.0 {
            transform.translation = HOVER_POSITIONS[card.id as usize];
            return card.id;
        }
    }
    return -1;
}

fn play_projectile_card(card: &Card) {
    println!("play projectile card");
}
fn play_melee_card(card: &Card) {
    println!("play melee card");
}
fn play_card(card: &Card) {
    match card.card_type {
        CardType::Melee => play_melee_card(card),
        CardType::Projectile => play_projectile_card(card),
    }
}

pub fn inventory_changed(
    mut commands: Commands,
    inventory_resource: Res<Inventory>,
    asset_server: Res<AssetServer>,
    card_query: Query<Entity, With<Card>>,
) {
    if inventory_resource.is_changed() && inventory_resource.cards.len() == 3 {
        println!("{:?}", inventory_resource);
        for entity in card_query.iter() {
            commands.entity(entity).despawn();
        }
        init_render_cards(commands, inventory_resource, asset_server);
    }
}