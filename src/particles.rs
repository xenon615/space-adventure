use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub fn engine_effect() ->EffectAsset {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(2., 2., 2., 1.));
    gradient.add_key(1., Vec4::new(0.0, 0.0, 10.0, 0.0));
    
    let color_modifier =  ColorOverLifetimeModifier {
        gradient: gradient,
    };
    let size_modifier = SetSizeModifier {
        size: Vec2::new(0.2, 0.2).into(),
        ..default()
    };

    let writer = ExprWriter::new();

    let age = writer.lit(0.).uniform(writer.lit(0.02)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(0.5).uniform(writer.lit(0.3)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionCone3dModifier {
        height: writer.lit(2.).expr(),
        base_radius: writer.lit(0.2).expr(),
        top_radius: writer.lit(0.3).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(1.) + writer.lit(2.)).expr(),
    };

    EffectAsset::new(
        vec![32768],
        Spawner::once(5000.0.into(), false),
        writer.finish(),
    )
    .with_name("engine")
    .init(init_pos)
    .init(init_vel)
    .init(init_age)
    .init(init_lifetime)
    .render(color_modifier)
    .render(size_modifier)
}

// ---

pub fn steer_effect() ->EffectAsset {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
    color_gradient.add_key(1.0, Vec4::new(0.0, 4.0, 0.0, 0.5));

    let size_modifier = SetSizeModifier {
        size: Vec2::new(0.2, 0.2).into(),
        ..default()
    };

    let writer = ExprWriter::new();

    let init_pos = SetPositionCone3dModifier {
        height: writer.lit(3.).expr(),
        base_radius: writer.lit(0.1).expr(),
        top_radius: writer.lit(0.2).expr(),
        dimension: ShapeDimension::Volume,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(5.).expr(),
    };

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(0.2).uniform(writer.lit(0.3)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    
    EffectAsset::new(
        vec![32768], 
        Spawner::once(500.0.into(), false), 
        writer.finish()
    )
    .with_name("steer")
    .init(init_vel)
    .init(init_pos)
    .init(init_age)
    .init(init_lifetime)
    .render(size_modifier)
    .render(ColorOverLifetimeModifier {
        gradient: color_gradient,
    })
    
}

// ---

pub fn aura_effect() ->EffectAsset {
    let writer = ExprWriter::new();
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(4.0, 4.0, 0.0, 1.));
    color_gradient.add_key(0.4, Vec4::new(0.0, 4.0, 0.0, 0.5));
    color_gradient.add_key(1.0, Vec4::new(4.0, 0.0, 4.0, 0.5));

    let size_modifier = SetSizeModifier {
        size: Vec2::new(0.05, 0.05).into(),
        ..default()
    };
    
    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(0.2).uniform(writer.lit(1.9)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionSphereModifier{
        center: writer.lit(Vec3::ZERO).expr(),
        dimension: ShapeDimension::Surface,
        radius: writer.lit(2.).expr()
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(1.).expr(),
    };

    EffectAsset::new(
        vec![1000], 
        Spawner::rate(500.0.into()), 
        writer.finish()
    )
    .with_name("aura")
    
    .init(init_pos)
    .init(init_age)
    .init(init_lifetime)
    .init(init_vel)
    .render(size_modifier)

    .render(ColorOverLifetimeModifier {
        gradient: color_gradient,
    })
} 

// ---

pub fn dock_aura_effect() ->EffectAsset {
    let writer = ExprWriter::new();

    let p_color = writer.add_property("p_color", Color::rgba(0.0, 14.0, 4.0, 1.).as_rgba_u32().into());
    let init_color = SetAttributeModifier::new(Attribute::COLOR, writer.prop(p_color).expr());

    let size_modifier = SetSizeModifier {
        size: Vec2::new(0.06, 0.06).into(),
        ..default()
    };


    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(0.2).uniform(writer.lit(1.9)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionSphereModifier{
        center: writer.lit(Vec3::ZERO).expr(),
        dimension: ShapeDimension::Surface,
        radius: writer.lit(10.).expr()
    };


    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(1.).expr(),
    };

    EffectAsset::new(
        vec![1000], 
        Spawner::rate(1000.0.into()), 
        writer.finish()
    )
    .with_name("dock aura")
    
    .init(init_pos)
    .init(init_age)
    .init(init_lifetime)
    .init(init_vel)
    .init(init_color)
    .render(size_modifier)
    
} 


