use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
    math::Vec3,
};
use std::collections::HashMap;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, calculate_strain_system);
}

#[derive(Component)]
pub struct MeshHandle(pub Handle<Mesh>);

#[rustfmt::skip]
pub fn create_cube_mesh() -> Mesh {
    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // The camera coordinate space is right-handed x-right, y-up, z-back. This means "forward" is -Z.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        vec![
            // top (facing towards +y)
            [-0.5, 0.5, -0.5], // vertex with index 0
            [0.5, 0.5, -0.5], // vertex with index 1
            [0.5, 0.5, 0.5], // etc. until 23
            [-0.5, 0.5, 0.5],
            // bottom   (-y)
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [-0.5, -0.5, 0.5],
            // right    (+x)
            [0.5, -0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
            [0.5, 0.5, -0.5],
            // left     (-x)
            [-0.5, -0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            // back     (+z)
            [-0.5, -0.5, 0.5],
            [-0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5],
            [0.5, -0.5, 0.5],
            // forward  (-z)
            [-0.5, -0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, -0.5],
        ],
    )
    // Set-up UV coordinates to point to the upper (V < 0.5), "dirt+grass" part of the texture.
    // Take a look at the custom image (assets/textures/array_texture.png)
    // so the UV coords will make more sense
    // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Assigning the UV coords for the top side.
            [0.0, 0.2], [0.0, 0.0], [1.0, 0.0], [1.0, 0.2],
            // Assigning the UV coords for the bottom side.
            [0.0, 0.45], [0.0, 0.25], [1.0, 0.25], [1.0, 0.45],
            // Assigning the UV coords for the right side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the left side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the back side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
            // Assigning the UV coords for the forward side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
        ],
    )
    // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
    // the surface.
    // Normals are required for correct lighting calculations.
    // Each array represents a normalized vector, which length should be equal to 1.0.
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
    )
    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    //
    // The first two defined triangles look like this (marked with the vertex indices,
    // and the axis), when looking down at the top (+y) of the cube:
    //   -Z
    //   ^
    // 0---1
    // |  /|
    // | / | -> +X
    // |/  |
    // 3---2
    //
    // The right face's (+x) triangles look like this, seen from the outside of the cube.
    //   +Y
    //   ^
    // 10--11
    // |  /|
    // | / | -> -Z
    // |/  |
    // 9---8
    //
    // The back face's (+z) triangles look like this, seen from the outside of the cube.
    //   +Y
    //   ^
    // 17--18
    // |\  |
    // | \ | -> +X
    // |  \|
    // 16--19
    .with_inserted_indices(Indices::U32(vec![
        0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
        4,5,7 , 5,6,7, // bottom (-y)
        8,11,9 , 9,11,10, // right (+x)
        12,13,15 , 13,14,15, // left (-x)
        16,19,17 , 17,19,18, // back (+z)
        20,21,23 , 21,22,23, // forward (-z)
    ]))
}

/// Creates a new icosphere mesh with the given radius and number of subdivisions.
///
/// # Arguments
/// * `radius` - The radius of the icosphere
/// * `subdivisions` - Number of subdivisions (higher = more detailed, but more vertices)
///
/// # Returns
/// A new `Mesh` representing the icosphere
#[rustfmt::skip]
pub fn create_icosphere_mesh(radius: f32, subdivisions: usize) -> Mesh {
    // Start with an icosahedron (20-sided polyhedron)
    let (mut positions, mut indices) = generate_icosahedron(radius);

    // Subdivide the mesh if requested
    if subdivisions > 0 {
        (positions, indices) = subdivide_mesh(positions, indices, subdivisions);

        // Project all vertices onto the sphere
        for position in &mut positions {
            let vec = Vec3::from(*position);
            let len = vec.length();
            *position = (vec / len * radius).into();
        }
    }

    // Calculate UV coordinates (spherical mapping)
    let uvs: Vec<[f32; 2]> = positions.iter()
        .map(|&p| {
            let normalized = Vec3::from(p).normalize();
            [
                (1.0 + normalized.x.atan2(normalized.z) / std::f32::consts::PI) * 0.5,
                (normalized.y + 1.0) * 0.5,
            ]
        })
        .collect();

    // Calculate normals (all pointing out from the center)
    let normals: Vec<[f32; 3]> = positions.iter()
        .map(|&p| (Vec3::from(p).normalize()).into())
        .collect();

    // Create the mesh
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(
        indices.into_iter().map(|i| i as u32).collect()
    ));

    mesh
}

/// Generates the vertices and indices for an icosahedron.
fn generate_icosahedron(radius: f32) -> (Vec<[f32; 3]>, Vec<u16>) {
    // Golden ratio
    let t = (1.0 + 5.0f32.sqrt()) / 2.0;

    // Normalize the vertices to the given radius
    let normalize = |v: [f32; 3]| -> [f32; 3] {
        let len = (v[0]*v[0] + v[1]*v[1] + v[2]*v[2]).sqrt();
        [v[0]/len*radius, v[1]/len*radius, v[2]/len*radius]
    };

    // The 12 vertices of an icosahedron
    let mut vertices = Vec::with_capacity(12);

    // Vertices of an icosahedron centered at the origin
    vertices.push(normalize([-1.0,   t, 0.0]));
    vertices.push(normalize([ 1.0,   t, 0.0]));
    vertices.push(normalize([-1.0,  -t, 0.0]));
    vertices.push(normalize([ 1.0,  -t, 0.0]));

    vertices.push(normalize([0.0, -1.0,   t]));
    vertices.push(normalize([0.0,  1.0,   t]));
    vertices.push(normalize([0.0, -1.0,  -t]));
    vertices.push(normalize([0.0,  1.0,  -t]));

    vertices.push(normalize([  t, 0.0, -1.0]));
    vertices.push(normalize([  t, 0.0,  1.0]));
    vertices.push(normalize([ -t, 0.0, -1.0]));
    vertices.push(normalize([ -t, 0.0,  1.0]));

    // Indices for the 20 triangular faces
    let indices = vec![
        0, 11, 5,   0, 5, 1,    0, 1, 7,    0, 7, 10,   0, 10, 11,
        1, 5, 9,    5, 11, 4,   11, 10, 2,  10, 7, 6,   7, 1, 8,
        3, 9, 4,    3, 4, 2,     3, 2, 6,    3, 6, 8,    3, 8, 9,
        4, 9, 5,    2, 4, 11,     6, 2, 10,   8, 6, 7,    9, 8, 1,
    ];

    (vertices, indices.into_iter().map(|i| i as u16).collect())
}

/// Subdivides a mesh by splitting each triangle into 4 smaller triangles.
fn subdivide_mesh(
    positions: Vec<[f32; 3]>,
    indices: Vec<u16>,
    levels: usize,
) -> (Vec<[f32; 3]>, Vec<u16>) {
    use std::collections::HashMap;

    let mut positions = positions;
    let mut indices = indices;

    for _ in 0..levels {
        let mut new_indices = Vec::with_capacity(indices.len() * 4);
        let mut new_vertices = positions.clone();
        let mut edge_map = HashMap::new();

        // For each triangle
        for chunk in indices.chunks_exact(3) {
            let i1 = chunk[0] as usize;
            let i2 = chunk[1] as usize;
            let i3 = chunk[2] as usize;

            // Get or create midpoints for each edge
            let m12 = *edge_map.entry((i1.min(i2), i1.max(i2))).or_insert_with(|| {
                let v1 = positions[i1];
                let v2 = positions[i2];
                let mid = [
                    (v1[0] + v2[0]) * 0.5,
                    (v1[1] + v2[1]) * 0.5,
                    (v1[2] + v2[2]) * 0.5,
                ];
                let idx = new_vertices.len();
                new_vertices.push(mid);
                idx as u16
            });

            let m23 = *edge_map.entry((i2.min(i3), i2.max(i3))).or_insert_with(|| {
                let v2 = positions[i2];
                let v3 = positions[i3];
                let mid = [
                    (v2[0] + v3[0]) * 0.5,
                    (v2[1] + v3[1]) * 0.5,
                    (v2[2] + v3[2]) * 0.5,
                ];
                let idx = new_vertices.len();
                new_vertices.push(mid);
                idx as u16
            });

            let m31 = *edge_map.entry((i3.min(i1), i3.max(i1))).or_insert_with(|| {
                let v3 = positions[i3];
                let v1 = positions[i1];
                let mid = [
                    (v3[0] + v1[0]) * 0.5,
                    (v3[1] + v1[1]) * 0.5,
                    (v3[2] + v1[2]) * 0.5,
                ];
                let idx = new_vertices.len();
                new_vertices.push(mid);
                idx as u16
            });

            // Add 4 new triangles
            new_indices.push(chunk[0]);
            new_indices.push(m12);
            new_indices.push(m31);

            new_indices.push(chunk[1]);
            new_indices.push(m23);
            new_indices.push(m12);

            new_indices.push(chunk[2]);
            new_indices.push(m31);
            new_indices.push(m23);

            new_indices.push(m12);
            new_indices.push(m23);
            new_indices.push(m31);
        }

        positions = new_vertices;
        indices = new_indices;
    }

    (positions, indices)
}

/// Returns the edge lengths of the mesh, assuming it is an unstressed
/// triangular mesh.
pub fn get_rest_lengths(mesh: &Mesh) -> Vec<(f32, f32, f32)> {
    let indices = mesh.indices().expect("Mesh has no indices");
    let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        .expect("Mesh has no positions")
        .as_float3()
        .expect("Positions are not f32x3");

    let mut rest_lengths = Vec::with_capacity(indices.len() / 3);

    match indices {
        Indices::U16(indices) => {
            for chunk in indices.chunks_exact(3) {
                let v1 = Vec3::from(positions[chunk[0] as usize]);
                let v2 = Vec3::from(positions[chunk[1] as usize]);
                let v3 = Vec3::from(positions[chunk[2] as usize]);
                rest_lengths.push((v1.distance(v2), v2.distance(v3), v3.distance(v1)));
            }
        }
        Indices::U32(indices) => {
            for chunk in indices.chunks_exact(3) {
                let v1 = Vec3::from(positions[chunk[0] as usize]);
                let v2 = Vec3::from(positions[chunk[1] as usize]);
                let v3 = Vec3::from(positions[chunk[2] as usize]);
                rest_lengths.push((v1.distance(v2), v2.distance(v3), v3.distance(v1)));
            }
        }
    }

    rest_lengths
}

/// Component that stores strain information for a deformable mesh
#[derive(Component, Debug, Default)]
pub struct Strain {
    /// Average strain value across all edges (positive = stretched, negative = compressed)
    pub average_strain: f32,
    /// Maximum strain value found in the mesh
    pub max_strain: f32,
    /// Minimum strain value found in the mesh
    pub min_strain: f32,
}

/// System that calculates strain for all meshes with RestState
///
/// This compares the current mesh state with its rest state to compute
/// strain values for visualization or physics simulation.
pub fn calculate_strain_system(
    mut query: Query<(
        &MeshHandle,
        &RestState,
        &mut Strain,
    )>,
    meshes: Res<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    for (mesh_handle, rest_state, mut strain) in query.iter_mut() {
        // Get the current mesh data
        let Some(mesh) = meshes.get(&mesh_handle.0) else { continue };

        // Get current positions
        let positions = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
            Some(attr) => match attr.as_float3() {
                Some(positions) => positions,
                None => continue,
            },
            None => continue,
        };

        // Initialize strain tracking
        let mut total_strain = 0.0;
        let mut max_strain = f32::NEG_INFINITY;
        let mut min_strain = f32::INFINITY;
        let mut edge_count = 0;

        // Calculate strain for each edge in the rest state
        for (edge, &rest_length) in &rest_state.rest_lengths {
            if rest_length <= f32::EPSILON {
                continue; // Skip zero-length edges
            }

            // Get current edge length
            let v1 = positions[edge.0 as usize];
            let v2 = positions[edge.1 as usize];
            let current_length = Vec3::from(v1).distance(Vec3::from(v2));

            // Calculate engineering strain: (L - L₀) / L₀
            let edge_strain = (current_length - rest_length) / rest_length;

            // Update statistics
            total_strain += edge_strain;
            max_strain = max_strain.max(edge_strain);
            min_strain = min_strain.min(edge_strain);
            edge_count += 1;
        }

        // Update strain component
        if edge_count > 0 {
            strain.average_strain = total_strain / edge_count as f32;
            strain.max_strain = max_strain;
            strain.min_strain = min_strain;
        }
    }
}

/// Component that stores the rest state of a deformable mesh
#[derive(Component, Debug)]
pub struct RestState {
    /// Original vertex positions
    pub rest_positions: Vec<[f32; 3]>,
    /// Map from edge (vertex index pair) to rest length
    pub rest_lengths: HashMap<(u32, u32), f32>,
    /// Original triangle indices
    pub indices: Vec<u32>,
}

impl RestState {
    /// Creates a new RestState from a mesh
    pub fn from_mesh(mesh: &Mesh) -> Self {
        let indices = match mesh.indices().expect("Mesh has no indices") {
            Indices::U16(idx) => idx.iter().map(|&i| i as u32).collect(),
            Indices::U32(idx) => idx.to_vec(),
        };

        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("Mesh has no positions")
            .as_float3()
            .expect("Positions are not f32x3")
            .to_vec();

        // Calculate rest lengths for all edges
        let mut rest_lengths = HashMap::new();
        for chunk in indices.chunks_exact(3) {
            let i1 = chunk[0];
            let i2 = chunk[1];
            let i3 = chunk[2];

            let v1 = Vec3::from(positions[i1 as usize]);
            let v2 = Vec3::from(positions[i2 as usize]);
            let v3 = Vec3::from(positions[i3 as usize]);

            // Store edges in both directions to make lookup easier
            rest_lengths.insert((i1.min(i2), i1.max(i2)), v1.distance(v2));
            rest_lengths.insert((i2.min(i3), i2.max(i3)), v2.distance(v3));
            rest_lengths.insert((i3.min(i1), i3.max(i1)), v3.distance(v1));
        }

        Self {
            rest_positions: positions,
            rest_lengths,
            indices,
        }
    }

    /// Gets the rest length between two vertices, if it exists
    pub fn get_rest_length(&self, v1: u32, v2: u32) -> Option<f32> {
        self.rest_lengths.get(&(v1.min(v2), v1.max(v2))).copied()
    }
}
