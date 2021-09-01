use bevy::{
    app::PluginGroupBuilder,
    asset::AssetPlugin,
    core::CorePlugin,
    diagnostic::DiagnosticsPlugin,
    input::InputPlugin,
    // log::LogPlugin,
    prelude::*,
    render::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    scene::ScenePlugin,
    wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions},
    window::WindowPlugin,
};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WgpuOptions {
            features: WgpuFeatures {
                // The Wireframe requires NonFillPolygonMode feature
                features: vec![WgpuFeature::NonFillPolygonMode],
            },
            ..Default::default()
        })
        .add_plugins(TestDefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // To draw the wireframe on all entities, set this to 'true'
    wireframe_config.global = false;
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        // This enables wireframe drawing on this entity
        .insert(Wireframe);
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

pub struct TestDefaultPlugins;

impl PluginGroup for TestDefaultPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        // group.add(LogPlugin::default());
        group.add(CorePlugin::default());
        group.add(TransformPlugin::default());
        group.add(DiagnosticsPlugin::default());
        group.add(InputPlugin::default());
        group.add(WindowPlugin::default());
        group.add(AssetPlugin::default());
        group.add(ScenePlugin::default());

        #[cfg(feature = "bevy_render")]
        group.add(RenderPlugin::default());

        #[cfg(feature = "bevy_sprite")]
        group.add(SpritePlugin::default());

        #[cfg(feature = "bevy_pbr")]
        group.add(PbrPlugin::default());

        #[cfg(feature = "bevy_ui")]
        group.add(UiPlugin::default());

        #[cfg(feature = "bevy_text")]
        group.add(TextPlugin::default());

        #[cfg(feature = "bevy_audio")]
        group.add(AudioPlugin::default());

        #[cfg(feature = "bevy_gilrs")]
        group.add(GilrsPlugin::default());

        #[cfg(feature = "bevy_gltf")]
        group.add(GltfPlugin::default());

        #[cfg(feature = "bevy_winit")]
        group.add(WinitPlugin::default());

        #[cfg(feature = "bevy_wgpu")]
        group.add(WgpuPlugin::default());
    }
}
