use crate::player::PlayerMarker;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::settings::*;
use noise::{NoiseFn, Perlin};
use std::collections::HashMap;

const CHUNK_SIZE: u32 = 128;
const DRAWN_AREA: i32 = 2;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_area)
            .add_systems(PreUpdate, generate_chunks)
            .add_systems(PreUpdate, undraw_area)
            .insert_resource(GlobalAmbientLight {
                color: Color::WHITE,
                brightness: 1000.0,
                affects_lightmapped_meshes: false,
            })
            .insert_resource(Map::default());
    }
}

#[derive(Resource, SettingsGroup, Reflect, Default)]
#[reflect(Resource, SettingsGroup, Default)]
pub struct MapSettings {
    seed: u32,
    chunk_size: u32,
}

type ChunkIndex = i32;
type ChunkData = [[f32; 3]; (CHUNK_SIZE * CHUNK_SIZE) as usize];
#[derive(Default, Resource)]
struct Map {
    chunks: HashMap<ChunkIndex, HashMap<ChunkIndex, Chunk>>,
}

impl Map {
    fn get_chunk(&self, x: ChunkIndex, y: ChunkIndex) -> Option<&Chunk> {
        self.chunks.get(&x).and_then(|map| map.get(&y))
    }

    fn insert_chunk(&mut self, x: ChunkIndex, y: ChunkIndex, chunk: Chunk) {
        if !self.chunks.contains_key(&x) {
            self.chunks.insert(x, HashMap::new());
        }

        self.chunks.get_mut(&x).unwrap().insert(y, chunk);
    }

    fn chunk_is_present(&self, x: ChunkIndex, y: ChunkIndex) -> bool {
        self.get_chunk(x, y).is_some()
    }
}

#[derive(Component, Default, Debug)]
struct ChunkMarker {}
struct Chunk {
    terrain: ChunkData,
}

impl Chunk {
    pub fn new(terrain: ChunkData) -> Self {
        Self { terrain }
    }

    pub fn to_mesh(&self) -> Mesh {
        let indices: Vec<u32> = calculate_indices(CHUNK_SIZE);
        let normals = calculate_normals(&self.terrain, &indices);
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, Default::default());

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.terrain.to_vec());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_indices(Indices::U32(indices));

        mesh
    }
}

fn draw_area(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map: Res<Map>,
    player_pos_queue: Query<&Transform, With<PlayerMarker>>,
) {
    let player_pos = player_pos_queue.single().unwrap();
    let player_chunk_x = player_pos.translation.x.floor() as i32;
    let player_chunk_z = player_pos.translation.z.floor() as i32;

    for chunk_x in player_chunk_x - DRAWN_AREA..player_chunk_x + DRAWN_AREA {
        for chunk_z in player_chunk_z - DRAWN_AREA..player_chunk_z + DRAWN_AREA {
            if let Some(chunk) = map
                .get_chunk(chunk_x, chunk_z)
                .and_then(|chunk| Some(chunk.to_mesh()))
            {
                commands.spawn((
                    Mesh3d(meshes.add(chunk)),
                    MeshMaterial3d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
                    ChunkMarker {},
                    Transform::from_xyz(
                        player_chunk_x as f32 * CHUNK_SIZE as f32,
                        0.0,
                        player_chunk_z as f32 * CHUNK_SIZE as f32,
                    ),
                    // Wireframe,
                ));
            }
        }
    }
}

fn undraw_area(mut commands: Commands, chunks: Query<Entity, With<ChunkMarker>>) {
    info!("{}", chunks.iter().count());
    for entity in chunks.iter() {
        commands.entity(entity).despawn();
    }
}

fn generate_chunks(
    settings: Res<MapSettings>,
    mut map: ResMut<Map>,
    player_pos_queue: Query<&Transform, With<PlayerMarker>>,
) {
    let player_pos = player_pos_queue.single().unwrap();
    let player_chunk_x = player_pos.translation.x.floor() as i32;
    let player_chunk_z = player_pos.translation.z.floor() as i32;

    for chunk_x in player_chunk_x - DRAWN_AREA..player_chunk_x + DRAWN_AREA {
        for chunk_z in player_chunk_z - DRAWN_AREA..player_chunk_z + DRAWN_AREA {
            if !map.chunk_is_present(chunk_x, chunk_z) {
                let seed = settings.seed;
                let perlin = Perlin::new(seed);
                let step = 1.0;

                let mut positions = Vec::new();

                for y in 0..=CHUNK_SIZE - 1 {
                    let yf = y as f64 * step;
                    for x in 0..=CHUNK_SIZE - 1 {
                        let xf = x as f64 * step;
                        let perlin_result = perlin.get([xf * 0.01, yf * 0.01]) * 100.0;
                        positions.push([xf as f32, perlin_result as f32, yf as f32]);
                    }
                }

                map.insert_chunk(
                    player_chunk_x,
                    player_chunk_z,
                    Chunk::new(positions.try_into().unwrap()),
                );
            }
        }
    }
}

fn calculate_indices(chunk_size: u32) -> Vec<u32> {
    let mut indices = Vec::new();

    for y in 0..chunk_size - 1 {
        for x in 0..chunk_size - 1 {
            let top_left = (y * chunk_size + x) as u32;
            let top_right = top_left + 1;
            let bottom_left = ((y + 1) * chunk_size + x) as u32;
            let bottom_right = bottom_left + 1;

            indices.extend_from_slice(&[top_left, bottom_left, top_right]);
            indices.extend_from_slice(&[top_right, bottom_left, bottom_right]);
        }
    }

    indices
}

pub fn calculate_normals(vertices: &ChunkData, indices: &[u32]) -> Vec<[f32; 3]> {
    let mut normals = vec![[0.0; 3]; vertices.len()];

    for triangle in indices.chunks_exact(3) {
        let i0 = triangle[0] as usize;
        let i1 = triangle[1] as usize;
        let i2 = triangle[2] as usize;

        let v0 = vertices[i0];
        let v1 = vertices[i1];
        let v2 = vertices[i2];

        // Two edges of the triangle
        let edge1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];

        let edge2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

        // Cross product
        let normal = [
            edge1[1] * edge2[2] - edge1[2] * edge2[1],
            edge1[2] * edge2[0] - edge1[0] * edge2[2],
            edge1[0] * edge2[1] - edge1[1] * edge2[0],
        ];

        // Add the face normal to each vertex
        for &index in &[i0, i1, i2] {
            normals[index][0] += normal[0];
            normals[index][1] += normal[1];
            normals[index][2] += normal[2];
        }
    }

    // Normalize the accumulated normals
    for normal in &mut normals {
        let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();

        if length > 0.0 {
            normal[0] /= length;
            normal[1] /= length;
            normal[2] /= length;
        }
    }

    normals
}
