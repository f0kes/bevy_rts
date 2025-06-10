use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use movement::follow::Follow;

use crate::loading::TextureAssets;

#[derive(Component)]
pub struct SpawnOnHitParticles;

pub fn spawn_on_hit_particles(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<SpawnOnHitParticles>>,
    mut effects: ResMut<Assets<EffectAsset>>,
    textures: Res<TextureAssets>,
) {
    for (entity, transform) in query.iter() {
        let writer = ExprWriter::new();

        let age = writer.lit(0.).expr();
        let init_age = SetAttributeModifier::new(Attribute::AGE, age);

        let lifetime = writer.lit(0.8).normal(writer.lit(1.2)).expr();
        let init_lifetime =
            SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

        let mut size_gradient1 = Gradient::new();
        size_gradient1.add_key(0.0, Vec3::splat(0.3));
        size_gradient1.add_key(0.3, Vec3::splat(0.10));
        size_gradient1.add_key(1.0, Vec3::splat(0.0));

        let mut color_gradient1 = Gradient::new();
        color_gradient1.add_key(0.0, Vec4::new(1.0, 0., 0., 1.0));
        color_gradient1.add_key(0.6, Vec4::new(1.0, 0., 0., 1.0));
        color_gradient1.add_key(1.0, Vec4::new(1.0, 0.1, 0.1, 0.5));

        let init_pos = SetPositionSphereModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            radius: writer.lit(0.1).expr(),
            dimension: ShapeDimension::Volume,
        };

        let  size = writer.lit(0.5).normal(writer.lit(5.0)).expr();
        let init_size = SetAttributeModifier::new(Attribute::SIZE, size);
        let accel = writer.lit(Vec3::Y * -16.).expr();
        let update_accel = AccelModifier::new(accel);

        let drag = writer.lit(4.).expr();
        let update_drag = LinearDragModifier::new(drag);

        let init_vel = SetVelocitySphereModifier {
            center: writer.lit(Vec3::ZERO).expr(),
            speed: (writer.rand(ScalarType::Float) * writer.lit(5.)
                + writer.lit(10.))
            .expr(),
        };
        let texture_handle = textures.particle.clone();
        let color = writer.rand(VectorType::VEC4F).pack4x8unorm();
        let init_color =
            SetAttributeModifier::new(Attribute::COLOR, color.expr());

        let rotation = (writer.rand(ScalarType::Float)
            * writer.lit(std::f32::consts::TAU))
        .expr();
        let init_rotation =
            SetAttributeModifier::new(Attribute::F32_0, rotation);

        let rotation_attr = writer.attr(Attribute::F32_0).expr();

        let texture_slot = writer.lit(0u32).expr();

        let mut module = writer.finish();
        module.add_texture_slot("color");

        let effect = effects.add(
            EffectAsset::new(
                32768,
                Spawner::burst(100.0.into(), 1.0.into()),
                module,
            )
            .with_name("billboard")
            .with_alpha_mode(bevy_hanabi::AlphaMode::Blend)
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_size)
            .init(init_lifetime)
            .init(init_rotation)
            .update(update_accel)
            .update(update_drag)
            .render(ParticleTextureModifier {
                texture_slot,
                sample_mapping: ImageSampleMapping::Modulate,
            })
            .render(OrientModifier {
                mode: OrientMode::FaceCameraPosition,
                rotation: Some(rotation_attr),
            })
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient1,
                screen_space_size: false,
            }), /* .render(ColorOverLifetimeModifier {
                    gradient: color_gradient1,
                }) */
        );
        commands.spawn((
            ParticleEffectBundle::new(effect),
            EffectMaterial {
                images: vec![texture_handle],
            },
            Name::new("effect"),
            Follow::target(entity),
        ));
        commands.entity(entity).remove::<SpawnOnHitParticles>();
    }
}
