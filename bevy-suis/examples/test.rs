use bevy::{color::palettes::css, prelude::*};
use bevy_mod_openxr::{add_xr_plugins, session::OxrSession};
use bevy_mod_xr::{session::session_running, types::XrPose};
use bevy_suis::openxr_low_level_actions::{binding::SuisOxrBindings, SuisOxrActionSet};

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(add_xr_plugins(DefaultPlugins));
    app.add_plugins((bevy_suis::openxr_low_level_actions::SuisOxrActionPlugin));
    app.insert_resource(
        SuisOxrBindings::default()
            .add_set("test", "Test")
            .add_action_xr_space("grip", "grip pose")
            .add_profile_bindings("/interaction_profiles/khr/simple_controller")
            .add_binding("/user/hand/left/input/grip/pose")
            .finish()
            .finish()
            .finish(),
    );
    app.add_systems(
        Update,
        add_space.run_if(session_running.and_then(run_once())),
    );
    app.add_systems(Update, draw_gizmo);
    app.run()
}

fn draw_gizmo(query: Query<&GlobalTransform, With<Test>>, mut gizmos: Gizmos) {
    for t in &query {
        info!("{}", t.translation());
        gizmos.circle(t.translation(), Dir3::NEG_Z, 0.1, css::RED);
    }
}

#[derive(Component)]
struct Test;
fn add_space(mut sets: Query<&mut SuisOxrActionSet>, mut cmds: Commands, session: Res<OxrSession>) {
    for mut set in &mut sets {
        let action = set.actions.first_mut().unwrap();
        let space = match action {
            bevy_suis::openxr_low_level_actions::SuisOxrAction::Bool(_) => todo!(),
            bevy_suis::openxr_low_level_actions::SuisOxrAction::F32(_) => todo!(),
            bevy_suis::openxr_low_level_actions::SuisOxrAction::Vec2(_) => todo!(),
            bevy_suis::openxr_low_level_actions::SuisOxrAction::Space(v) => {
                v.get_space(&session, openxr::Path::NULL, XrPose::IDENTITY)
            }
        };
        cmds.spawn((SpatialBundle::default(), space.unwrap(), Test));
    }
}
