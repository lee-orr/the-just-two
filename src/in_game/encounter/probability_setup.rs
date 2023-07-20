use bevy::{ecs::query::Has, prelude::*};

use super::{
    actions::{ActionChoice, ChallengerAction},
    dice_pools::*,
    sequencing::EncounterState,
};

use bevy_ui_dsl::*;

use crate::ui::{classes::*, intermediary_node_bundles::*};
pub struct ProbabilitySetupPlugin;

impl Plugin for ProbabilitySetupPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DiceType>()
            .register_type::<DicePoolType>()
            .register_type::<DicePool>()
            .register_type::<InitialPools>()
            .add_systems(OnEnter(EncounterState::ProbabilitySetup), setup)
            .add_systems(OnExit(EncounterState::ProbabilitySetup), exit);
    }
}

#[derive(Component)]
struct Screen;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    actions: Query<(Entity, &ActionChoice, Has<ChallengerAction>)>,
) {
    let r = root(
        c_probability_setup_root,
        &asset_server,
        &mut commands,
        |p| {
            node(probability_grid, p, |p| {
                for (_entity, choice, is_challenger) in actions.iter() {
                    node(
                        (
                            probability_card.nb(),
                            if is_challenger {
                                challenger_card.nb()
                            } else {
                                player_card.nb()
                            },
                        ),
                        p,
                        |p| {
                            node(probability_card_title.nb(), p, |p| {
                                text(
                                    choice.title.as_str(),
                                    (),
                                    (probability_card_title_text, druid_text),
                                    p,
                                );
                            });
                        },
                    );
                }
            });
        },
    );
    commands.entity(r).insert(Screen);
}

fn exit(mut commands: Commands, query: Query<Entity, With<Screen>>) {
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}
