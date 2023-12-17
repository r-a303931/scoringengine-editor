// state.rs: State management of the application itself, to include model and messages
//
// Copyright (C) 2023 Andrew Rioux
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::rc::Rc;

use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::config::{
    BlueTeamEditor, ConfigurationEditor, IpGeneratorScheme, MachineEditor, RedWhiteTeamEditor,
    ServiceEditor,
};

const STORAGE_KEY: &str = "stored_configurations";

fn save_changes(state: EditorState) -> EditorState {
    let _ = LocalStorage::set(STORAGE_KEY, state.configs.clone());
    state
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct StoredConfigurations {
    pub name: String,
    pub config: ConfigurationEditor,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CurrentView {
    Input,
    Teams,
    Machines,
    IpSettings,
    Output,
}

pub enum EditorMessage {
    EditConfigName(String, u8),
    FinishInit(u8),
    DeleteConfig(u8),
    CreateNew(String),
    Copy(String, u8),
    ChangeToView(CurrentView),
    UpdateIpSettings(IpGeneratorScheme),
    Error(String),
    AddRedWhiteTeam(RedWhiteTeamEditor),
    EditRedWhiteTeam(u8, RedWhiteTeamEditor),
    RemoveRedWhiteTeam(u8),
    AddBlueTeam(BlueTeamEditor),
    EditBlueTeam(u8, BlueTeamEditor),
    RemoveBlueTeam(u8),
    AddMachine(MachineEditor),
    UpdateMachine(u8, MachineEditor),
    RemoveMachine(u8),
    DropService(u8),
    PickupService(ServiceEditor),
    HoverOverMachine(u8),
    StopHoveringOverMachines,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EditorState {
    pub error: Option<String>,
    pub configs: Vec<StoredConfigurations>,
    pub state: EditingState,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EditingState {
    Initializing,
    HasConfig {
        config: u8,
        current_view: CurrentView,
        currently_hovered_machine_name: Option<u8>,
        service_to_drop: Box<Option<ServiceEditor>>,
    },
}

impl EditorState {
    #[track_caller]
    pub fn force_init(
        &self,
    ) -> (
        &ConfigurationEditor,
        &CurrentView,
        Option<u8>,
        Option<&ServiceEditor>,
    ) {
        match &self.state {
            EditingState::Initializing { .. } => panic!("forced init on uninit state"),
            EditingState::HasConfig {
                config,
                current_view,
                currently_hovered_machine_name,
                service_to_drop,
            } => (
                &(self.configs[*config as usize].config),
                &current_view,
                currently_hovered_machine_name.clone(),
                service_to_drop.as_ref().as_ref(),
            ),
        }
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
}

pub type EditorStateContext = UseReducerHandle<EditorState>;

impl Reducible for EditorState {
    type Action = EditorMessage;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match (&self.state, action) {
            (_, EditorMessage::EditConfigName(n, i)) => {
                let mut cconfigs = self.configs.clone();
                cconfigs[i as usize].name = n;

                save_changes(EditorState {
                    configs: cconfigs,
                    ..(*self).clone()
                })
                .into()
            }
            (_, EditorMessage::DeleteConfig(i)) => {
                let mut cconfigs = self.configs.clone();
                cconfigs.remove(i as usize);

                save_changes(EditorState {
                    configs: cconfigs,
                    ..(*self).clone()
                })
                .into()
            }
            (_, EditorMessage::Copy(name, i)) => {
                let mut cconfigs = self.configs.clone();
                let config = self.configs[i as usize].clone().config;
                cconfigs.push(StoredConfigurations { name, config });

                save_changes(EditorState {
                    configs: cconfigs,
                    ..(*self).clone()
                })
                .into()
            }
            (_, EditorMessage::FinishInit(i)) => EditorState {
                state: EditingState::HasConfig {
                    config: i,
                    current_view: CurrentView::Machines,
                    currently_hovered_machine_name: None,
                    service_to_drop: Box::new(None),
                },
                ..(*self).clone()
            }
            .into(),
            (_, EditorMessage::CreateNew(name)) => {
                let mut cconfigs = self.configs.clone();
                cconfigs.push(StoredConfigurations {
                    name,
                    config: ConfigurationEditor {
                        red_white_teams: vec![],
                        blue_teams: vec![],
                        machines: vec![],
                        ip_generator: IpGeneratorScheme::OneTeam,
                    },
                });
                save_changes(EditorState {
                    configs: cconfigs,
                    state: EditingState::HasConfig {
                        config: self.configs.len() as u8,
                        current_view: CurrentView::Machines,
                        currently_hovered_machine_name: None,
                        service_to_drop: Box::new(None),
                    },
                    ..(*self).clone()
                })
                .into()
            }
            (EditingState::HasConfig { config, .. }, EditorMessage::ChangeToView(view)) => {
                EditorState {
                    state: EditingState::HasConfig {
                        config: *config,
                        current_view: view,
                        currently_hovered_machine_name: None,
                        service_to_drop: Box::new(None),
                    },
                    ..(*self).clone()
                }
                .into()
            }
            (
                EditingState::HasConfig { config, .. },
                EditorMessage::UpdateIpSettings(new_ip_settings),
            ) => {
                let mut cconfigs = self.configs.clone();
                cconfigs[*config as usize].config.ip_generator = new_ip_settings;
                save_changes(EditorState {
                    configs: cconfigs,
                    ..(*self).clone()
                })
                .into()
            }
            (_, EditorMessage::Error(e)) => EditorState {
                error: Some(e),
                ..(*self).clone()
            }
            .into(),
            (EditingState::HasConfig { config, .. }, EditorMessage::AddRedWhiteTeam(team)) => {
                let mut cconfigs = self.configs.clone();
                cconfigs[*config as usize].config.red_white_teams.push(team);
                save_changes(EditorState {
                    configs: cconfigs,
                    ..(*self).clone()
                })
                .into()
            }
            (EditingState::HasConfig { config, .. }, EditorMessage::AddBlueTeam(team)) => {
                let mut cconfigs = self.configs.clone();
                cconfigs[*config as usize].config.blue_teams.push(team);
                save_changes(EditorState {
                    configs: cconfigs,
                    ..(*self).clone()
                })
                .into()
            }
            (
                EditingState::HasConfig { config, .. },
                EditorMessage::EditRedWhiteTeam(ind, team),
            ) => {
                let mut cconfigs = self.configs.clone();
                cconfigs[*config as usize].config.red_white_teams[ind as usize] = team;

                save_changes(EditorState {
                    configs: cconfigs,
                    ..(*self).clone()
                })
                .into()
            }
            (EditingState::HasConfig { config, .. }, EditorMessage::EditBlueTeam(ind, team)) => {
                let mut cconfigs = self.configs.clone();
                cconfigs[*config as usize].config.blue_teams[ind as usize] = team;

                save_changes(EditorState {
                    configs: cconfigs,
                    ..(*self).clone()
                })
                .into()
            }
            (EditingState::HasConfig { config, .. }, EditorMessage::RemoveRedWhiteTeam(team)) => {
                let mut cconfigs = self.configs.clone();
                cconfigs[*config as usize]
                    .config
                    .red_white_teams
                    .remove(team as usize);
                save_changes(EditorState {
                    configs: cconfigs,
                    ..(*self).clone()
                })
                .into()
            }
            (EditingState::HasConfig { config, .. }, EditorMessage::RemoveBlueTeam(team)) => {
                let mut cconfigs = self.configs.clone();
                cconfigs[*config as usize]
                    .config
                    .blue_teams
                    .remove(team as usize);
                save_changes(EditorState {
                    configs: cconfigs,
                    ..(*self).clone()
                })
                .into()
            }
            (EditingState::HasConfig { config, .. }, EditorMessage::AddMachine(machine)) => {
                let mut cconfigs = self.configs.clone();
                cconfigs[*config as usize].config.machines.push(machine);
                save_changes(EditorState {
                    configs: cconfigs,
                    ..(*self).clone()
                })
                .into()
            }
            (
                EditingState::HasConfig { config, .. },
                EditorMessage::UpdateMachine(ind, machine),
            ) => {
                let mut cconfigs = self.configs.clone();
                cconfigs[*config as usize].config.machines[ind as usize] = machine;
                save_changes(EditorState {
                    configs: cconfigs,
                    ..(*self).clone()
                })
                .into()
            }
            (EditingState::HasConfig { config, .. }, EditorMessage::RemoveMachine(ind)) => {
                let mut cconfigs = self.configs.clone();
                cconfigs[*config as usize]
                    .config
                    .machines
                    .remove(ind as usize);
                save_changes(EditorState {
                    configs: cconfigs,
                    ..(*self).clone()
                })
                .into()
            }
            (
                EditingState::HasConfig {
                    config,
                    current_view,
                    currently_hovered_machine_name,
                    service_to_drop,
                },
                EditorMessage::DropService(ind),
            ) => match *service_to_drop.clone() {
                Some(service) => {
                    let mut cconfigs = self.configs.clone();
                    cconfigs[*config as usize].config.machines[ind as usize]
                        .services
                        .push(service);

                    save_changes(EditorState {
                        configs: cconfigs,
                        state: EditingState::HasConfig {
                            service_to_drop: Box::new(None),
                            config: *config,
                            current_view: *current_view,
                            currently_hovered_machine_name: currently_hovered_machine_name.clone(),
                        },
                        ..(*self).clone()
                    })
                    .into()
                }
                None => EditorState {
                    state: EditingState::HasConfig {
                        service_to_drop: Box::new(None),
                        config: *config,
                        current_view: *current_view,
                        currently_hovered_machine_name: currently_hovered_machine_name.clone(),
                    },
                    ..(*self).clone()
                }
                .into(),
            },
            (
                EditingState::HasConfig {
                    config,
                    current_view,
                    currently_hovered_machine_name,
                    ..
                },
                EditorMessage::PickupService(service_to_drop),
            ) => EditorState {
                state: EditingState::HasConfig {
                    config: *config,
                    current_view: *current_view,
                    currently_hovered_machine_name: currently_hovered_machine_name.clone(),
                    service_to_drop: Box::new(Some(service_to_drop)),
                },
                ..(*self).clone()
            }
            .into(),
            (
                EditingState::HasConfig {
                    config,
                    current_view,
                    service_to_drop,
                    ..
                },
                EditorMessage::HoverOverMachine(name),
            ) => EditorState {
                state: EditingState::HasConfig {
                    config: *config,
                    current_view: *current_view,
                    currently_hovered_machine_name: Some(name),
                    service_to_drop: service_to_drop.clone(),
                },
                ..(*self).clone()
            }
            .into(),
            (
                EditingState::HasConfig {
                    config,
                    current_view,
                    service_to_drop,
                    ..
                },
                EditorMessage::StopHoveringOverMachines,
            ) => EditorState {
                state: EditingState::HasConfig {
                    config: *config,
                    current_view: *current_view,
                    currently_hovered_machine_name: None,
                    service_to_drop: service_to_drop.clone(),
                },
                ..(*self).clone()
            }
            .into(),

            (EditingState::Initializing { .. }, _) => self, // misconfigured case, shouldn't happen
        }
    }
}

#[derive(Properties, Debug, PartialEq)]
pub struct EditorStateProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn EditorStateProvider(props: &EditorStateProviderProps) -> Html {
    let state = use_reducer(|| {
        let configs =
            LocalStorage::get::<Vec<StoredConfigurations>>(STORAGE_KEY).unwrap_or_default();

        EditorState {
            configs,
            error: None,
            state: EditingState::Initializing,
        }
    });

    html! {
        <ContextProvider<EditorStateContext> context={state}>
            {props.children.clone()}
        </ContextProvider<EditorStateContext>>
    }
}
