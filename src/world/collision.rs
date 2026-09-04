use std::collections::HashSet;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::translation_to_grid_coords;

#[derive(Component, Default)]
pub struct Solid;

#[derive(Bundle, LdtkIntCell, Default)]
pub struct SolidBundle {
    solid: Solid,
}

#[derive(Resource, Default)]
pub struct SolidCells {
    cell_set: HashSet<GridCoords>
}

pub fn cache_solid_cells(
    mut reader: MessageReader<LevelEvent>,
    query: Query<&GlobalTransform, With<Solid>>,
    mut cells: ResMut<SolidCells>,
) {
    for message in reader.read() {
        if let LevelEvent::Spawned(..) = message {
            cells.cell_set.clear();
            for transform in &query {
                let new_coords = translation_to_grid_coords(
                    transform.translation().truncate(),
                    IVec2::splat(16),
                );

                cells.cell_set.insert(new_coords);
            }
            for x in &cells.cell_set {
                println!("{x:?}");
            }
        }
    }
}

