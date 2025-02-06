use bevy::{ecs::query, prelude::*};
use bevy_hanabi::prelude::*;

#[derive(Component)]
pub struct SpawnOnHitParticles;

pub fn spawn_on_hit_particles(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<SpawnOnHitParticles>>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    for (entity, transform) in query.iter() {
        let mut color_gradient = Gradient::new();
        color_gradient.add_key(0.0, Vec4::new(2.0, 0.5, 0.0, 1.0)); // Bright orange
        color_gradient.add_key(0.4, Vec4::new(1.0, 0.2, 0.0, 0.8)); // Dark orange
        color_gradient.add_key(1.0, Vec4::new(0.3, 0.1, 0.0, 0.0)); // Fade to transparent

        let mut size_gradient = Gradient::new();
        size_gradient.add_key(0.0, Vec3::splat(0.2)); // Start size
        size_gradient.add_key(0.3, Vec3::splat(0.1)); // Shrink a bit
        size_gradient.add_key(1.0, Vec3::splat(0.0)); // Fade to nothing

        let writer = ExprWriter::new();

        // Randomize initial particle age for variation
        let age = writer.lit(0.).uniform(writer.lit(0.1)).expr();
        let init_age = SetAttributeModifier::new(Attribute::AGE, age);

        // Randomize lifetime
        let lifetime = writer.lit(0.3).uniform(writer.lit(0.5)).expr();
        let init_lifetime =
            SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        // Initial position in small sphere
        let init_pos = SetPositionSphereModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            radius: writer.lit(0.1).expr(),
            dimension: ShapeDimension::Volume,
        };

        // Burst velocity outward
        let init_vel = SetVelocitySphereModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            speed: (writer.rand(ScalarType::Float) * writer.lit(8.)
                + writer.lit(12.))
            .expr(),
        };

        // Add drag to slow particles
        let drag = writer.lit(8.).expr();
        let update_drag = LinearDragModifier::new(drag);

        // Add gravity
        let accel = writer.lit(Vec3::Y * -20.).expr();
        let update_accel = AccelModifier::new(accel);

        let effect = EffectAsset::new(
            100, // Fewer particles for better performance
            Spawner::burst(100.0.into(), 0.01.into()), // Quick burst
            writer.finish(),
        )
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .update(update_drag)
        .update(update_accel)
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false,
        });

        let handle = effects.add(effect);

        commands.spawn((
            Name::new("on_hit_particles"),
            ParticleEffectBundle {
                effect: ParticleEffect::new(handle),
                transform: transform.clone(),
                ..default()
            },
        ));

        commands.entity(entity).remove::<SpawnOnHitParticles>();
    }
}
