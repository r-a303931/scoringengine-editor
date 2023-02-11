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

use yew::prelude::*;

use crate::config::{
    BlueTeamEditor, Configuration, ConfigurationEditor, IpGeneratorScheme, RedWhiteTeamEditor,
    ServiceEditor,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CurrentView {
    Input,
    Teams,
    Machines,
    IpSettings,
    Output,
}

pub enum EditorMessage {
    UpdateInitialEditor(String),
    FinishInit,
    CreateNew,
    ChangeToView(CurrentView),
    UpdateIpSettings(IpGeneratorScheme),
    Error(String),
    AddRedWhiteTeam(RedWhiteTeamEditor),
    EditRedWhiteTeam(u8, RedWhiteTeamEditor),
    RemoveRedWhiteTeam(u8),
    AddBlueTeam(BlueTeamEditor),
    EditBlueTeam(u8, BlueTeamEditor),
    RemoveBlueTeam(u8),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EditorState {
    Initializing {
        editor_box: String,
        error: Option<String>,
    },
    HasConfig {
        editor_box: String,
        error: Option<String>,
        config: ConfigurationEditor,
        current_view: CurrentView,
        currently_hovered_machine_name: Option<String>,
        service_to_drop: Option<ServiceEditor>,
    },
}

impl EditorState {
    #[track_caller]
    pub fn force_init(
        &self,
    ) -> (
        &str,
        Option<&str>,
        &ConfigurationEditor,
        &CurrentView,
        Option<&str>,
        Option<&ServiceEditor>,
    ) {
        match self {
            EditorState::Initializing { .. } => panic!("forced init on uninit state"),
            EditorState::HasConfig {
                editor_box,
                error,
                config,
                current_view,
                currently_hovered_machine_name,
                service_to_drop,
            } => (
                &editor_box,
                error.as_ref().map(|x| &**x),
                &config,
                &current_view,
                currently_hovered_machine_name.as_ref().map(|x| &**x),
                service_to_drop.as_ref(),
            ),
        }
    }

    pub fn error(&self) -> Option<&str> {
        match self {
            EditorState::Initializing { error, .. } => error.as_ref().map(|x| &**x),
            EditorState::HasConfig { error, .. } => error.as_ref().map(|x| &**x),
        }
    }
}

impl Default for EditorState {
    fn default() -> Self {
        EditorState::Initializing {
            editor_box: "".to_owned(),
            error: None,
        }
    }
}

pub type EditorStateContext = UseReducerHandle<EditorState>;

impl Reducible for EditorState {
    type Action = EditorMessage;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match (&*self, action) {
            (EditorState::Initializing { .. }, EditorMessage::UpdateInitialEditor(new_text)) => {
                EditorState::Initializing {
                    editor_box: new_text,
                    error: None,
                }
                .into()
            }
            (
                EditorState::HasConfig {
                    config,
                    current_view,
                    currently_hovered_machine_name,
                    service_to_drop,
                    ..
                },
                EditorMessage::UpdateInitialEditor(new_text),
            ) => EditorState::HasConfig {
                editor_box: new_text.clone(),
                config: config.clone(),
                current_view: *current_view,
                currently_hovered_machine_name: currently_hovered_machine_name.clone(),
                service_to_drop: service_to_drop.clone(),
                error: None,
            }
            .into(),
            (EditorState::Initializing { editor_box, .. }, EditorMessage::FinishInit) => {
                match serde_yaml::from_str::<Configuration>(&editor_box) {
                    Ok(conf) => EditorState::HasConfig {
                        editor_box: editor_box.to_string(),
                        error: None,
                        config: conf.editor_info,
                        current_view: CurrentView::Machines,
                        currently_hovered_machine_name: None,
                        service_to_drop: None,
                    }
                    .into(),
                    Err(e) => EditorState::Initializing {
                        editor_box: editor_box.to_string(),
                        error: Some(format!("Error parsing input! {}", e)),
                    }
                    .into(),
                }
            }
            (EditorState::HasConfig { editor_box, .. }, EditorMessage::FinishInit) => {
                match serde_yaml::from_str::<Configuration>(&editor_box) {
                    Ok(conf) => EditorState::HasConfig {
                        editor_box: editor_box.to_string(),
                        error: None,
                        config: conf.editor_info,
                        current_view: CurrentView::Machines,
                        currently_hovered_machine_name: None,
                        service_to_drop: None,
                    }
                    .into(),
                    Err(e) => EditorState::Initializing {
                        editor_box: editor_box.to_string(),
                        error: Some(format!("Error parsing input! {}", e)),
                    }
                    .into(),
                }
            }
            (_, EditorMessage::CreateNew) => EditorState::HasConfig {
                editor_box: "".to_string(),
                error: None,
                config: ConfigurationEditor {
                    red_white_teams: vec![],
                    blue_teams: vec![],
                    machines: vec![],
                    ip_generator: crate::config::IpGeneratorScheme::OneTeam,
                },
                current_view: CurrentView::Teams,
                currently_hovered_machine_name: None,
                service_to_drop: None,
            }
            .into(),
            (
                EditorState::HasConfig {
                    config,
                    currently_hovered_machine_name,
                    service_to_drop,
                    editor_box,
                    ..
                },
                EditorMessage::ChangeToView(view),
            ) => EditorState::HasConfig {
                editor_box: editor_box.to_string(),
                error: None,
                config: config.clone(),
                currently_hovered_machine_name: currently_hovered_machine_name.clone(),
                service_to_drop: service_to_drop.clone(),
                current_view: view,
            }
            .into(),
            (
                EditorState::HasConfig {
                    editor_box,
                    config,
                    current_view,
                    currently_hovered_machine_name,
                    service_to_drop,
                    ..
                },
                EditorMessage::UpdateIpSettings(new_ip_settings),
            ) => EditorState::HasConfig {
                editor_box: editor_box.clone(),
                error: None,
                config: ConfigurationEditor {
                    red_white_teams: config.red_white_teams.clone(),
                    blue_teams: config.blue_teams.clone(),
                    machines: config.machines.clone(),
                    ip_generator: new_ip_settings,
                },
                current_view: *current_view,
                currently_hovered_machine_name: currently_hovered_machine_name.clone(),
                service_to_drop: service_to_drop.clone(),
            }
            .into(),
            (
                EditorState::HasConfig {
                    editor_box,
                    config,
                    current_view,
                    currently_hovered_machine_name,
                    service_to_drop,
                    ..
                },
                EditorMessage::Error(e),
            ) => EditorState::HasConfig {
                editor_box: editor_box.clone(),
                error: Some(e),
                config: config.clone(),
                current_view: *current_view,
                currently_hovered_machine_name: currently_hovered_machine_name.clone(),
                service_to_drop: service_to_drop.clone(),
            }
            .into(),
            (
                EditorState::HasConfig {
                    editor_box,
                    error,
                    config,
                    current_view,
                    currently_hovered_machine_name,
                    service_to_drop,
                },
                EditorMessage::AddRedWhiteTeam(team),
            ) => {
                let mut cconfig = config.clone();
                cconfig.red_white_teams.push(team);
                EditorState::HasConfig {
                    editor_box: editor_box.clone(),
                    error: error.clone(),
                    config: cconfig,
                    current_view: *current_view,
                    currently_hovered_machine_name: currently_hovered_machine_name.clone(),
                    service_to_drop: service_to_drop.clone(),
                }
                .into()
            }
            (
                EditorState::HasConfig {
                    editor_box,
                    error,
                    config,
                    current_view,
                    currently_hovered_machine_name,
                    service_to_drop,
                },
                EditorMessage::AddBlueTeam(team),
            ) => {
                let mut cconfig = config.clone();
                cconfig.blue_teams.push(team);
                EditorState::HasConfig {
                    editor_box: editor_box.clone(),
                    error: error.clone(),
                    config: cconfig,
                    current_view: *current_view,
                    currently_hovered_machine_name: currently_hovered_machine_name.clone(),
                    service_to_drop: service_to_drop.clone(),
                }
                .into()
            }
            (
                EditorState::HasConfig {
                    editor_box,
                    error,
                    config,
                    current_view,
                    currently_hovered_machine_name,
                    service_to_drop,
                },
                EditorMessage::EditRedWhiteTeam(ind, team),
            ) => {
                let mut cconfig = config.clone();
                let team_mut = cconfig.red_white_teams.get_mut(ind as usize);

                if let Some(team_mut) = team_mut {
                    team_mut.name = team.name;
                    team_mut.white_team = team.white_team;
                    team_mut.users = team.users;
                }

                EditorState::HasConfig {
                    editor_box: editor_box.clone(),
                    error: error.clone(),
                    config: cconfig,
                    current_view: *current_view,
                    currently_hovered_machine_name: currently_hovered_machine_name.clone(),
                    service_to_drop: service_to_drop.clone(),
                }
                .into()
            }
            (
                EditorState::HasConfig {
                    editor_box,
                    error,
                    config,
                    current_view,
                    currently_hovered_machine_name,
                    service_to_drop,
                },
                EditorMessage::EditBlueTeam(ind, team),
            ) => {
                let mut cconfig = config.clone();
                let team_mut = cconfig.blue_teams.get_mut(ind as usize);

                if let Some(team_mut) = team_mut {
                    team_mut.id = team.id;
                    team_mut.name = team.name;
                    team_mut.users = team.users;
                }

                EditorState::HasConfig {
                    editor_box: editor_box.clone(),
                    error: error.clone(),
                    config: cconfig,
                    current_view: *current_view,
                    currently_hovered_machine_name: currently_hovered_machine_name.clone(),
                    service_to_drop: service_to_drop.clone(),
                }
                .into()
            }
            (
                EditorState::HasConfig {
                    editor_box,
                    error,
                    config,
                    current_view,
                    currently_hovered_machine_name,
                    service_to_drop,
                },
                EditorMessage::RemoveRedWhiteTeam(ind),
            ) => {
                let mut cconfig = config.clone();
                cconfig.red_white_teams.remove(ind as usize);

                EditorState::HasConfig {
                    editor_box: editor_box.clone(),
                    error: error.clone(),
                    config: cconfig,
                    current_view: *current_view,
                    currently_hovered_machine_name: currently_hovered_machine_name.clone(),
                    service_to_drop: service_to_drop.clone(),
                }
                .into()
            }
            (
                EditorState::HasConfig {
                    editor_box,
                    error,
                    config,
                    current_view,
                    currently_hovered_machine_name,
                    service_to_drop,
                },
                EditorMessage::RemoveBlueTeam(ind),
            ) => {
                let mut cconfig = config.clone();
                cconfig.blue_teams.remove(ind as usize);

                EditorState::HasConfig {
                    editor_box: editor_box.clone(),
                    error: error.clone(),
                    config: cconfig,
                    current_view: *current_view,
                    currently_hovered_machine_name: currently_hovered_machine_name.clone(),
                    service_to_drop: service_to_drop.clone(),
                }
                .into()
            }

            (EditorState::Initializing { .. }, _) => self, // misconfigured case, shouldn't happen
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
    let state = use_reducer(|| EditorState::default());

    html! {
        <ContextProvider<EditorStateContext> context={state}>
            {props.children.clone()}
        </ContextProvider<EditorStateContext>>
    }
}
