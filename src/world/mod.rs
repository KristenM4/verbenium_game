use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use collision::SolidBundle;
use collision::SolidCells;

pub mod collision;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_int_cell::<SolidBundle>(1)
            .init_resource::<SolidCells>()
            .add_systems(PostUpdate, collision::cache_solid_cells
                .after(TransformSystems::Propagate));
    }
}
