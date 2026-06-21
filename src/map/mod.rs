use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::settings::*;
use noise::{NoiseFn, Perlin};

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map)
            .insert_resource(GlobalAmbientLight {
                color: Color::WHITE,
                brightness: 1000.0,
                affects_lightmapped_meshes: false,
            });
    }
}

#[derive(Resource, SettingsGroup, Reflect, Default)]
#[reflect(Resource, SettingsGroup, Default)]
pub struct MapSettings {
    seed: u32,
    chunk_size: u32,
}

pub fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<MapSettings>,
) {
    let seed = settings.seed;
    let perlin = Perlin::new(seed);
    let perlin2 = Perlin::new(seed * 2);
    let chunk_size = settings.chunk_size;
    let step = 1.0;

    let mut positions = Vec::new();

    for y in 0..=chunk_size {
        let yf = y as f64 * step;
        for x in 0..=chunk_size {
            let xf = x as f64 * step;
            let perlin_result = perlin.get([xf * 0.01, yf * 0.01]) * 100.0;
            let perlin_result2 = perlin2.get([xf * 0.1, yf * 0.1]) * 10.0;
            positions.push([
                xf as f32,
                (perlin_result + perlin_result2) as f32,
                yf as f32,
            ]);
        }
    }

    let indices: Vec<u32> = calculate_indices(chunk_size);

    let normals = calculate_normals(&positions, &indices);

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        Default::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        normals,
    );
    mesh.insert_indices(Indices::U32(indices));

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
    ));
}
    fn calculate_indices(chunk_size: u32) -> Vec<u32>
    {
        let mut indices: Vec<u32> = Vec::new();

        // 2 triangles per square
        for y in 0..chunk_size {
            for x in 0..chunk_size {
                let i = y * (chunk_size + 1) + x;

                let a = i;
                let b = i + 1;
                let c = i + (chunk_size + 1);
                let d = i + (chunk_size + 1) + 1;

                // triangle 1
                indices.push(a);
                indices.push(c);
                indices.push(b);

                // triangle 2
                indices.push(b);
                indices.push(c);
                indices.push(d);
            }
        }

        indices
    }

    pub fn calculate_normals(
        vertices: &[[f32; 3]],
        indices: &[u32],
    ) -> Vec<[f32; 3]> {
        let mut normals = vec![[0.0; 3]; vertices.len()];

        for triangle in indices.chunks_exact(3) {
            let ia = triangle[0] as usize;
            let ib = triangle[1] as usize;
            let ic = triangle[2] as usize;

            let a = Vec3::from_array(vertices[ia]);
            let b = Vec3::from_array(vertices[ib]);
            let c = Vec3::from_array(vertices[ic]);

            let face_normal = (b - a).cross(c - a);

            for i in [ia, ib, ic] {
                normals[i][0] += face_normal.x;
                normals[i][1] += face_normal.y;
                normals[i][2] += face_normal.z;
            }
        }

        for normal in &mut normals {
            let n = Vec3::from_array(*normal);

            if n.length_squared() > 0.0 {
                *normal = n.normalize().to_array();
            }
        }

        normals
    }