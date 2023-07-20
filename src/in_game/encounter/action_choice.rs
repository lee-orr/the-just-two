use crate::ui::classes::*;
use bevy::prelude::*;
use bevy_ui_dsl::*;

use super::sequencing::{EncounterState, FlushAvailableActions, PublishAvailableActions};

pub struct ActionChoicePlugin;

impl Plugin for ActionChoicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(EncounterState::ActionChoice),
            (
                apply_deferred
                    .in_set(FlushAvailableActions)
                    .after(PublishAvailableActions),
                setup,
            )
                .chain(),
        )
        .add_systems(OnExit(EncounterState::ActionChoice), exit);
    }
}

#[derive(Component)]
struct Screen;

#[derive(Component)]
pub struct ActionChoice {
    pub title: String,
    pub content: String,
    pub fail: usize,
    pub success: usize,
    pub critical_success: usize,
}

#[derive(Component)]
pub struct ChosenAction;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    actions: Query<(Entity, &ActionChoice)>,
) {
    let r = root(c_action_choice_root, &asset_server, &mut commands, |p| {
        for (_entity, choice) in actions.iter() {
            node(card, p, |p| {
                node(card_title.nb(), p, |p| {
                    text(choice.title.as_str(), (), (card_title_text, druid_text), p);
                });
                node(card_content.nb(), p, |p| {
                    text(choice.content.as_str(), (), standard_text, p);
                });
                node(card_footer.nb(), p, |p| {
                    node(
                        card_fail.nb(),
                        p,
                        |p: &mut UiChildBuilder<'_, '_, '_, '_>| {
                            text(
                                format!("{}", choice.fail),
                                (),
                                (card_fail_text, druid_text),
                                p,
                            );
                        },
                    );
                    node(card_success.nb(), p, |p| {
                        text(
                            format!("{}", choice.critical_success),
                            (),
                            (card_critical, druid_text),
                            p,
                        );
                        text(
                            format!("{}", choice.success),
                            (),
                            (card_success_text, druid_text),
                            p,
                        );
                    });
                });
            });
        }
    });
    commands.entity(r).insert(Screen);
}

fn exit(mut commands: Commands, query: Query<Entity, With<Screen>>) {
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}
