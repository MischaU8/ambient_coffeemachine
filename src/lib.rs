use ambient_api::{
    components::core::{
        game_objects::player_camera,
        transform::{lookat_center, translation, rotation}, prefab::{prefab_from_url, spawned}, rendering::{sun, light_diffuse, fog_density, sky, cast_shadows}, app::main_scene,
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

#[main]
pub async fn main() -> EventResult {
    make_transformable()
        .with_default(sun())
        .with(rotation(), Quat::from_rotation_y(-45_f32.to_radians()))
        .with(light_diffuse(), Vec3::ONE)
        .with(fog_density(), 0.)
        .with_default(main_scene())
        .spawn();

    make_transformable().with_default(sky()).spawn();

    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with_default(player_camera())
        .with(translation(), Vec3::ONE * 3.)
        .with(lookat_center(), vec3(0., 0., 0.))
        .spawn();

    let coffeemachine = Entity::new()
        .with_merge(make_transformable())
        .with(prefab_from_url(), asset_url("assets/coffeemachine.glb").unwrap())
        .with_default(cast_shadows())
        .spawn();
    entity::wait_for_component(coffeemachine, spawned()).await;

    on(event::FRAME, move |_| {
        entity::set_component(
            coffeemachine,
            rotation(),
            Quat::from_axis_angle(Vec3::Z, 0.75 * time().sin() - 45_f32.to_radians() ),
        );

        EventOk
    });

    EventOk
}
