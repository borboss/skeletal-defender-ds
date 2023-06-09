use super::components::*;
use bevy::prelude::*;

pub fn move_projectile(
    mut commands: Commands,
    mut projectile_query: Query<
        (
            Entity,
            &mut Transform,
            &mut SpawnedProjectile,
            &mut TextureAtlasSprite,
        ),
        With<SpawnedProjectile>,
    >,
    time: Res<Time>,
) {
    let x_min: f32 = 0.0 + 16.0;
    let x_max: f32 = 960.0 - 16.0;
    let y_min: f32 = (10.0f32 * 5.0f32) + 16.0;
    let y_max: f32 = 540.0 - 100.0 - 16.0;
    for (entity, mut projectile_transform, mut spawned_projectile, mut sprite) in
        projectile_query.iter_mut()
    {
        /*
        2.0 Attacks (Upper Half of Screen)
        1.4 Enemies (Upper Half of Screen)
        1.0 is Player
        0.95 is Attacks (Lower Half of Screen)
        0.9 is Enemies (Lower Half of Screen)
        */

        let direction: Vec3 = Vec3::new(
            spawned_projectile.direction.x,
            spawned_projectile.direction.y,
            0.0f32,
        );
        projectile_transform.translation += direction * 600.0 * time.delta_seconds();

        let mut translation: Vec3 = projectile_transform.translation;

        // Bound x position
        if translation.x < x_min {
            translation.x = x_min;
            spawned_projectile.direction.x *= -1.0;
            spawned_projectile.total_bounces += 1 as u8;
        } else if translation.x > x_max {
            translation.x = x_max;
            spawned_projectile.direction.x *= -1.0;
            spawned_projectile.total_bounces += 1 as u8;
        }

        // Bound y position
        if translation.y < y_min {
            translation.y = y_min;
            spawned_projectile.direction.y *= -1.0;
            spawned_projectile.total_bounces += 1 as u8;
        } else if translation.y > y_max {
            translation.y = y_max;
            spawned_projectile.direction.y *= -1.0;
            spawned_projectile.total_bounces += 1 as u8;
        }

        if direction.x > 0.0 {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }

        projectile_transform.translation = translation;
        if spawned_projectile.total_bounces >= spawned_projectile.max_bounces {
            // todo: animate poof
            commands.entity(entity).despawn();
        }
    }
}

pub fn projectile_z_manager(mut projectile_query: Query<&mut Transform, Without<DontManageZ>>) {
    for mut projectile_transform in projectile_query.iter_mut() {
        if projectile_transform.translation.y < 540.0 / 2.0 {
            projectile_transform.translation.z = 2.0;
        } else {
            projectile_transform.translation.z = 0.95;
        }
    }
}

pub fn despawn_projectiles(mut commands: Commands, projectiles: Query<Entity, With<SpawnedProjectile>>) {
    for entity in projectiles.iter() {
        commands.entity(entity).despawn();
    }
}