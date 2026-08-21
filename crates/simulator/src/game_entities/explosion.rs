use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;
use derive_more::From;

pub const EXPLOSION_LIFETIME_SECS: f32 = 1.;
pub const EXPLOSION_START_RADIUS: f32 = 10.;
pub const EXPLOSION_END_RADIUS: f32 = 15.;
pub const EXPLOSION_COLOR: Color = Color::srgb(255., 0.3, 0.);


pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, explosions_setup);
    }
}

fn explosions_setup(mut meshes: ResMut<Assets<Mesh>>,
                    mut commands: Commands) {
    let explosion_mesh = meshes.add(Circle::default());
    commands.insert_resource(ExplosionMesh(explosion_mesh));
}

#[derive(Component, Debug)]
#[require(Transform)]
pub struct Explosion;

#[derive(Resource, Deref)]
pub struct ExplosionMesh(pub Handle<Mesh>);

#[derive(Component, From)]
pub struct ExplosionSecsLived(pub f32);

#[derive(Bundle)]
pub struct ExplosionEntity {
    mesh: Mesh2d,
    mat: MeshMaterial2d<ColorMaterial>,
    tr: Transform,
    identity: Explosion,
    secs_lived: ExplosionSecsLived
}

impl ExplosionEntity {
    pub fn new(mesh: &Res<ExplosionMesh>,
               materials: &mut ResMut<Assets<ColorMaterial>>,
               pos: Vec3) -> Self {
        Self {
            mesh: Mesh2d(mesh.0.clone()),
            mat: MeshMaterial2d(materials.add(ColorMaterial {
                color: EXPLOSION_COLOR,
                alpha_mode: AlphaMode2d::Blend,
                ..default()
            })),
            tr: Transform::from_translation(pos).with_scale(Vec3::splat(EXPLOSION_START_RADIUS)),
            identity: Explosion,
            secs_lived: 0.0.into(),
        }
    }
}