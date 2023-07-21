use std::{iter::Peekable, ops::Div};

use bevy::{ecs::query::Has, prelude::*};
use bevy_turborand::{DelegatedRng, GlobalRng};

use super::{
    actions::{ActionChoice, ActionResult, ChallengerAction, Resolution},
    dice_pools::*,
    powers::Power,
    sequencing::EncounterState,
};

use bevy_ui_dsl::*;

use crate::{
    assets::MainGameAssets,
    in_game::InGameUpdate,
    ui::{
        buttons::{focus_text_button, focused_button_activated, TypedFocusedButtonQuery},
        classes::*,
        colors,
        intermediary_node_bundles::*,
        DisplayBundle,
    },
};
pub struct ProbabilitySetupPlugin;

impl Plugin for ProbabilitySetupPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DiceType>()
            .register_type::<DicePoolType>()
            .register_type::<DicePool>()
            .register_type::<InitialPools>()
            .add_systems(
                OnEnter(EncounterState::ProbabilitySetup),
                (setup, setup_initial_pools),
            )
            .add_systems(
                OnExit(EncounterState::ProbabilitySetup),
                (resolve_actions, exit),
            )
            .add_systems(
                InGameUpdate,
                (
                    update_dice_pool_display.before(clear_updated_dice_pool),
                    update_powers.before(clear_updated_powers),
                    update_probability_distibution,
                    focused_button_activated.pipe(process_input),
                )
                    .run_if(in_state(EncounterState::ProbabilitySetup)),
            );
    }
}

#[derive(Component)]
struct Screen;

#[derive(Component)]
struct DicePoolControl(Entity);

#[derive(Component)]
struct ProbabilityVisualizer(Entity, Vec<(u8, f32)>);

#[derive(Component)]
struct PowerContainer;

#[derive(Component)]
struct ResolveButton;

#[derive(Component)]
struct UpdatedDicePool;

#[derive(Component)]
struct UpdatePowers;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    actions: Query<(Entity, &ActionChoice, Has<ChallengerAction>)>,
) {
    let mut dice_pool_controls = Vec::new();
    let mut probability_visualizers = Vec::new();
    let mut resolve_button = None;
    let mut power_container = None;
    let r = root(
        c_probability_setup_root,
        &asset_server,
        &mut commands,
        |p| {
            node(probability_grid, p, |p| {
                for (entity, choice, is_challenger) in actions.iter() {
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
                            dice_pool_controls.push((
                                node(probability_card_dice_pool_container.nb(), p, |_| {}),
                                entity,
                            ));
                            probability_visualizers
                                .push((node(probability_card_visualizer.nb(), p, |_| {}), entity));
                        },
                    );
                }
            });
            node((probability_power_container, probability_grid), p, |p| {
                node((), p, |_| {}).set(&mut power_container);
                focus_text_button(
                    "Resolve!",
                    (c_button.nb(), primary_box_item.nb()),
                    apply_button_state,
                    button_text,
                    p,
                )
                .set(&mut resolve_button);
            });
        },
    );
    commands.entity(r).insert(Screen);

    if let Some(resolve_button) = resolve_button {
        commands.entity(resolve_button).insert(ResolveButton);
    }
    if let Some(power_container) = power_container {
        commands
            .entity(power_container)
            .insert((PowerContainer, UpdatePowers));
    }

    for (ctl, target) in dice_pool_controls.iter() {
        commands.entity(*ctl).insert(DicePoolControl(*target));
    }
    for (ctl, target) in probability_visualizers.iter() {
        commands
            .entity(*ctl)
            .insert(ProbabilityVisualizer(*target, vec![]));
    }
}

fn exit(mut commands: Commands, query: Query<Entity, With<Screen>>) {
    info!("Exiting Probability Resolution");
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn setup_initial_pools(mut commands: Commands, query: Query<(Entity, &ActionChoice)>) {
    for (entity, choice) in query.iter() {
        commands
            .entity(entity)
            .insert(UpdatedDicePool)
            .with_children(|p| {
                for pool in choice.dice_pool.iter() {
                    p.spawn(pool.clone());
                }
            });
    }
}

fn update_dice_pool_display(
    mut commands: Commands,
    dice_pools: Query<&DicePool>,
    updated_actions: Query<&Children, With<UpdatedDicePool>>,
    dice_pool_display: Query<(Entity, &DicePoolControl)>,
    asset_server: Res<AssetServer>,
    assets: Res<MainGameAssets>,
) {
    for (display_entity, DicePoolControl(action_entity)) in dice_pool_display.iter() {
        let Ok(dice_pool_entities) = updated_actions.get(*action_entity) else {
            continue;
        };
        let dice_pool_root = root((), &asset_server, &mut commands, |p| {
            for child in dice_pool_entities.iter() {
                let Ok(dice_pool) = dice_pools.get(*child) else {
                    continue;
                };
                dice_pool.display_bundle(&assets, Val::Px(20.), p);
            }
        });
        commands
            .entity(display_entity)
            .despawn_descendants()
            .add_child(dice_pool_root);
    }
}

fn clear_updated_dice_pool(mut commands: Commands, actions: Query<Entity, With<UpdatedDicePool>>) {
    for action in actions.iter() {
        commands.entity(action).remove::<UpdatedDicePool>();
    }
}

fn resolve_actions(
    mut commands: Commands,
    dice_pools: Query<&DicePool>,
    updated_actions: Query<(Entity, &ActionChoice, &Children)>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for (entity, action, dice_pool_entities) in updated_actions.iter() {
        let dice_pools = dice_pool_entities
            .iter()
            .flat_map(|e| dice_pools.get(*e).ok())
            .collect::<Vec<_>>();
        let roll = dice_pools.as_slice().roll(global_rng.get_mut());
        let (result, gap) = action.evaluate(roll);
        commands
            .entity(entity)
            .despawn_descendants()
            .insert(Resolution { roll, result, gap });
    }
}

fn update_probability_distibution(
    mut commands: Commands,
    dice_pools: Query<&DicePool>,
    updated_actions: Query<(&Children, &ActionChoice)>,
    dice_pool_display: Query<(Entity, &ProbabilityVisualizer)>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for (display_entity, ProbabilityVisualizer(action_entity, stored_simulation)) in
        dice_pool_display.iter()
    {
        let Ok((dice_pool_entities, action)) = updated_actions.get(*action_entity) else {
            continue;
        };
        let dice_pools = dice_pool_entities
            .iter()
            .flat_map(|e| dice_pools.get(*e).ok())
            .collect::<Vec<_>>();
        let simulation =
            SimulateDice::<100>::simulate(&dice_pools.as_slice(), global_rng.get_mut());

        let simulation = Averager::<'_, _, _, 1, 20>(
            simulation.iter().peekable(),
            stored_simulation.iter().peekable(),
        )
        .collect::<Vec<(u8, f32)>>();

        commands
            .entity(display_entity)
            .despawn_descendants()
            .with_children(|p| {
                for (value, rate) in simulation.iter() {
                    let (result_type, _) = action.evaluate(*value);
                    let result_type = match result_type {
                        ActionResult::CriticalFail => colors::CRITICAL_FAIL_COLOR,
                        ActionResult::Fail => colors::FAIL_COLOR,
                        ActionResult::Success => colors::SUCCESS_COLOR,
                        ActionResult::CriticalSuccess => colors::CRITICAL_COLOR,
                    };
                    p.spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(*rate * 100.),
                            flex_grow: 1.,
                            flex_shrink: 1.,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|p| {
                        p.spawn(NodeBundle {
                            style: Style {
                                height: Val::Percent(100.),
                                position_type: PositionType::Absolute,
                                top: Val::Px(0.),
                                left: Val::Px(1.5),
                                right: Val::Px(1.5),
                                ..Default::default()
                            },
                            background_color: BackgroundColor(result_type),
                            ..Default::default()
                        });
                    });
                }
            })
            .insert(ProbabilityVisualizer(*action_entity, simulation));
    }
}

fn update_powers(
    mut commands: Commands,
    power_containers: Query<Entity, With<UpdatePowers>>,
    powers: Query<(Entity, &Power)>,
    assets: Res<MainGameAssets>,
    asset_server: Res<AssetServer>,
) {
    for container in power_containers.iter() {
        let root = root((), &asset_server, &mut commands, |p| {
            for (_entity, power) in powers.iter() {
                power.display_bundle(&assets, Val::Px(50.), p);
            }
        });
        commands
            .entity(container)
            .despawn_descendants()
            .add_child(root);
    }
}

fn clear_updated_powers(
    mut commands: Commands,
    power_containers: Query<Entity, With<UpdatePowers>>,
) {
    for entity in power_containers.iter() {
        commands.entity(entity).remove::<UpdatePowers>();
    }
}

fn process_input(
    In(focused): In<Option<Entity>>,
    mut commands: Commands,
    interaction_query: TypedFocusedButtonQuery<'_, '_, '_, ResolveButton>,
) {
    let Some(focused) = focused else {
        return;
    };
    let Some((_, _)) = interaction_query.get(focused).ok() else {
        return;
    };
    commands.insert_resource(NextState(Some(EncounterState::OutcomeResolution)));
}

struct Averager<
    'a,
    T: Iterator<Item = &'a (u8, f32)>,
    R: Iterator<Item = &'a (u8, f32)>,
    const WEIGHT_A: usize,
    const WEIGHT_B: usize,
>(Peekable<T>, Peekable<R>);

impl<
        'a,
        T: Iterator<Item = &'a (u8, f32)>,
        R: Iterator<Item = &'a (u8, f32)>,
        const WEIGHT_A: usize,
        const WEIGHT_B: usize,
    > Iterator for Averager<'a, T, R, WEIGHT_A, WEIGHT_B>
{
    type Item = (u8, f32);

    fn next(&mut self) -> Option<Self::Item> {
        let Some((a, _ap)) = self.0.peek() else {
            return self.1.next().cloned();
        };
        let Some((b, _bp)) = self.1.peek() else {
            return self.0.next().cloned();
        };
        match a.cmp(b) {
            std::cmp::Ordering::Less => self.0.next().cloned(),
            std::cmp::Ordering::Equal => {
                let (a, ap) = self.0.next().unwrap();
                let (_b, bp) = self.1.next().unwrap();
                Some((
                    *a,
                    (*ap * (WEIGHT_A as f32) + *bp * (WEIGHT_B as f32))
                        .div(WEIGHT_A as f32 + WEIGHT_B as f32),
                ))
            }
            std::cmp::Ordering::Greater => self.1.next().cloned(),
        }
    }
}
