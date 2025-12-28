use bevy::prelude::*;
use std::f32::consts::TAU;

use crate::components::Orchestrator;
use std::env;

#[derive(Resource)]
struct GalaxyTopologyResource {
    topology: Vec<Vec<bool>>
}

pub fn main() -> Result<(), String>{

    // Load env
    dotenv::dotenv().ok();
    //Init and check orchestrator
    let mut orchestrator = Orchestrator::new()?;

    //Give the absolute path for the init file
    let file_path = env::var("INPUT_FILE")
        .expect("Imposta INPUT_FILE nel file .env o come variabile d'ambiente");

    orchestrator.initialize_galaxy_by_file(file_path.as_str().trim())?;

    let topology = orchestrator.get_topology();

    let mut app = App::new();
    app
    .insert_resource(GalaxyTopologyResource{topology})
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup);
    app.run();
    Ok(())
}

const PLANET_RAD: f32 = 20.;
const GALAXY_RADIUS: f32 = 150.;

fn setup(
    topology: Res<GalaxyTopologyResource>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    commands.spawn(Camera2d);

    let planet_num = topology.into_inner().topology.len();

    let shape = meshes.add(Circle::new(PLANET_RAD));

    for i in 0..planet_num {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / planet_num as f32, 0.95, 0.7);

        let angle = TAU * (i as f32) / (planet_num as f32);

        let x = GALAXY_RADIUS * angle.cos();
        let y = GALAXY_RADIUS * angle.sin();

        commands.spawn((
            Mesh2d(shape.clone()),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                x,
                y,
                0.0,
            ),
        ));
    }

}