// users.rs: The user/team editor
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

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    config::{BlueTeamEditor, RedWhiteTeamEditor, User},
    state::{self, EditorMessage},
};

#[derive(Clone, PartialEq, Debug)]
struct UserProps {
    username: AttrValue,
    password: AttrValue,
}

#[derive(Clone, Properties, PartialEq)]
struct UserEditorProps {
    username: AttrValue,
    password: AttrValue,
    update_user: Callback<(String, String)>,
    delete_user: Callback<()>,
}

#[function_component]
fn UserEditorComponent(props: &UserEditorProps) -> Html {
    let username_ref = use_node_ref();
    let password_ref = use_node_ref();

    let update_username = {
        let username_ref = username_ref.clone();
        let update_user = props.update_user.clone();
        let password = props.password.clone();

        Callback::from(move |_| {
            if let Some(input) = username_ref.cast::<HtmlInputElement>() {
                let value = input.value();

                update_user.emit((value, password.to_string()));
            }
        })
    };

    let update_password = {
        let password_ref = password_ref.clone();
        let update_user = props.update_user.clone();
        let username = props.username.clone();

        Callback::from(move |_| {
            if let Some(input) = password_ref.cast::<HtmlInputElement>() {
                let value = input.value();

                update_user.emit((username.to_string(), value));
            }
        })
    };

    let delete_user_onclick = {
        let delete_user = props.delete_user.clone();

        Callback::from(move |_| delete_user.emit(()))
    };

    html! {
        <div class="user-editor">
            <div class="form-row">
                <div class="form-block">
                    { "Username" }
                </div>

                <div class="form-block">
                    <input
                        type="text"
                        value={props.username.clone()}
                        ref={username_ref}
                        onchange={update_username}
                    />
                </div>
            </div>

            <div class="form-row">
                <div class="form-block">
                    { "Password" }
                </div>

                <div class="form-block">
                    <input
                        type="text"
                        value={props.password.clone()}
                        ref={password_ref}
                        onchange={update_password}
                    />
                </div>
            </div>

            <div class="form-row">
                <div class="form-block">
                    { "Delete user" }
                </div>

                <div class="form-block">
                    <a href="#" onclick={delete_user_onclick}>
                        { "Delete user" }
                    </a>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct UserListEditorProps {
    users: Rc<Vec<UserProps>>,
    update_users: Callback<Vec<UserProps>>,
}

#[function_component]
fn UserListEditor(props: &UserListEditorProps) -> Html {
    let user_list = props.users.iter().enumerate().map(|(i, user)| {
        let user = user.clone();

        let update_user = {
            let users = props.users.clone();
            let overall_callback = props.update_users.clone();

            Callback::from(move |(username, password): (String, String)| {
                let mut new_users = (*users).clone();

                new_users[i] = UserProps {
                    username: username.into(),
                    password: password.into(),
                };

                overall_callback.emit(new_users);
            })
        };

        let delete_user = {
            let users = props.users.clone();
            let overall_callback = props.update_users.clone();

            Callback::from(move |_| {
                let mut new_users = (*users).clone();
                log::info!("Deleting user {i}: {users:?}, {users:?}");
                new_users.remove(i);
                overall_callback.emit(new_users);
            })
        };

        html! {
            <UserEditorComponent
                username={user.username.clone()}
                password={user.password.clone()}
                {update_user}
                {delete_user}
            />
        }
    });

    html! {
        <div class="user-editor-list">
            { for user_list }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct RedWhiteTeamEditorProps {
    name: AttrValue,
    users: Rc<Vec<UserProps>>,
    white_team: bool,
    modify_red_white_team: Callback<(AttrValue, Rc<Vec<UserProps>>, bool)>,
    delete_team: Callback<()>,
}

#[function_component]
fn RedWhiteTeamEditorComponent(props: &RedWhiteTeamEditorProps) -> Html {
    let name_ref = use_node_ref();
    let type_ref = use_node_ref();

    let set_name = {
        let update_team = props.modify_red_white_team.clone();
        let users = props.users.clone();
        let name_ref = name_ref.clone();
        let white_team = props.white_team;

        Callback::from(move |_| {
            let Some(input) = name_ref.cast::<HtmlInputElement>() else { return; };
            let value = input.value();

            update_team.emit((value.into(), users.clone().into(), white_team));
        })
    };

    let change_team_type = {
        let update_team = props.modify_red_white_team.clone();
        let users = props.users.clone();
        let name = props.name.clone();
        let type_ref = type_ref.clone();

        Callback::from(move |_| {
            let Some(input) = type_ref.cast::<HtmlInputElement>() else { return; };
            let value = input.value();

            let white_team = value == "white";

            update_team.emit((name.clone().into(), users.clone().into(), white_team));
        })
    };

    let update_users = {
        let update_team = props.modify_red_white_team.clone();
        let name = props.name.clone();
        let white_team = props.white_team;

        Callback::from(move |users| update_team.emit((name.clone(), Rc::new(users), white_team)))
    };

    let add_user = {
        let name = props.name.clone();
        let users = props.users.clone();
        let white_team = props.white_team;
        let update_team = props.modify_red_white_team.clone();

        Callback::from(move |_| {
            let mut users = (*users).clone();
            users.push(UserProps {
                username: "".into(),
                password: "".into(),
            });
            update_team.emit((name.clone(), users.into(), white_team));
        })
    };

    let delete_team = {
        let delete_team = props.delete_team.clone();

        Callback::from(move |_| delete_team.emit(()))
    };

    html! {
        <div class="team-editor red-team-editor">
            <div class="form-row">
                <div class="form-block">
                    { "Team name" }
                </div>

                <div class="form-block">
                    <input
                        ref={name_ref}
                        type="text"
                        value={props.name.clone()}
                        onchange={set_name}
                    />
                </div>
            </div>

            <div class="form-row">
                <div class="form-block">
                    { "White team or red team" }
                </div>

                <div class="form-block">
                    <select
                        value={if props.white_team { "white" } else { "red" }}
                        onchange={change_team_type}
                        ref={type_ref}
                    >
                        <option value="red">{ "Red team" }</option>
                        <option value="white">{ "White team" }</option>
                    </select>
                </div>
            </div>

            <div class="form-row">
                <div class="form-block">
                    { "Users" }
                </div>

                <div class="form-block">
                    <a href="#" onclick={add_user}>
                        { "Add user" }
                    </a>
                </div>
            </div>

            <UserListEditor
                users={props.users.clone()}
                {update_users}
            />

            <div class="form-row">
                <div class="form-block">
                </div>

                <div class="form-block">
                    <a href="#" onclick={delete_team}>
                        { "Delete team" }
                    </a>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct BlueTeamEditorProps {
    id: u8,
    name: AttrValue,
    users: Rc<Vec<UserProps>>,
    modify_blue_team: Callback<(AttrValue, Rc<Vec<UserProps>>, u8)>,
    delete_team: Callback<()>,
}

#[function_component]
fn BlueTeamEditorComponent(props: &BlueTeamEditorProps) -> Html {
    let name_ref = use_node_ref();
    let id_ref = use_node_ref();

    let id_input_state = use_state(|| (AttrValue::from(props.id.to_string()), None::<String>));

    let set_name = {
        let update_team = props.modify_blue_team.clone();
        let users = props.users.clone();
        let name_ref = name_ref.clone();
        let id = props.id;

        Callback::from(move |_| {
            let Some(input) = name_ref.cast::<HtmlInputElement>() else { return; };
            let value = input.value();

            update_team.emit((value.into(), users.clone().into(), id));
        })
    };

    let set_id = {
        let update_team = props.modify_blue_team.clone();
        let users = props.users.clone();
        let name = props.name.clone();
        let id_ref = id_ref.clone();
        let id_input_state = id_input_state.clone();

        Callback::from(move |_| {
            let Some(input) = id_ref.cast::<HtmlInputElement>() else { return; };
            let value = input.value();

            match value.parse::<u8>() {
                Ok(id) => {
                    id_input_state.set((id.to_string().into(), None));

                    update_team.emit((name.clone().into(), users.clone().into(), id));
                }
                Err(e) => id_input_state.set((value.into(), Some(format!("Parse error: {e:?}")))),
            }
        })
    };

    let update_users = {
        let update_team = props.modify_blue_team.clone();
        let name = props.name.clone();
        let id = props.id;

        Callback::from(move |users| update_team.emit((name.clone(), Rc::new(users), id)))
    };

    let add_user = {
        let name = props.name.clone();
        let users = props.users.clone();
        let id = props.id;
        let update_team = props.modify_blue_team.clone();

        Callback::from(move |_| {
            let mut users = (*users).clone();
            users.push(UserProps {
                username: "".into(),
                password: "".into(),
            });
            update_team.emit((name.clone(), users.into(), id));
        })
    };

    let delete_team = {
        let delete_team = props.delete_team.clone();

        Callback::from(move |_| delete_team.emit(()))
    };

    html! {
        <div class="team-editor red-team-editor">
            <div class="form-row">
                <div class="form-block">
                    { "Team name" }
                </div>

                <div class="form-block">
                    <input
                        ref={name_ref}
                        type="text"
                        value={props.name.clone()}
                        onchange={set_name}
                    />
                </div>
            </div>

            <div class="form-row">
                <div class="form-block">
                    { "Team ID" }
                </div>

                <div class="form-block">
                    <input
                        ref={id_ref}
                        type="text"
                        value={id_input_state.0.clone()}
                        onchange={set_id}
                    />
                </div>
            </div>

            <div class="form-row">
                <div class="form-block">
                    { "Users" }
                </div>

                <div class="form-block">
                    <a href="#" onclick={add_user}>
                        { "Add user" }
                    </a>
                </div>
            </div>

            <UserListEditor
                users={props.users.clone()}
                {update_users}
            />

            <div class="form-row">
                <div class="form-block">
                </div>

                <div class="form-block">
                    <a href="#" onclick={delete_team}>
                        { "Delete team" }
                    </a>
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn TeamsEditor() -> Html {
    let editor_state = use_context::<crate::state::EditorStateContext>().unwrap();
    let (_, _, config, _, _, _) = editor_state.force_init();
    let red_white_teams = config.red_white_teams.clone();
    let blue_teams = config.blue_teams.clone();

    let new_team_id = blue_teams.iter().map(|team| team.id).max().unwrap_or(0);

    let add_new_red_white_team = {
        let editor_state = editor_state.clone();

        Callback::from(move |_| {
            editor_state.dispatch(state::EditorMessage::AddRedWhiteTeam(RedWhiteTeamEditor {
                name: "".into(),
                users: vec![],
                white_team: true,
            }));
        })
    };

    let add_new_blue_team = {
        let editor_state = editor_state.clone();

        Callback::from(move |_| {
            editor_state.dispatch(state::EditorMessage::AddBlueTeam(BlueTeamEditor {
                id: new_team_id + 1,
                name: "".into(),
                users: vec![],
            }));
        })
    };

    let red_team_editors = red_white_teams.iter().enumerate().map(|(i, team)| {
        let modify_red_white_team = {
            let editor_state = editor_state.clone();

            Callback::from(
                move |(name, users, white_team): (AttrValue, Rc<Vec<UserProps>>, bool)| {
                    editor_state.dispatch(EditorMessage::EditRedWhiteTeam(
                        i.try_into().unwrap(),
                        RedWhiteTeamEditor {
                            name: name.to_string(),
                            users: users
                                .iter()
                                .map(|user| User {
                                    username: user.username.to_string(),
                                    password: user.password.to_string(),
                                })
                                .collect(),
                            white_team,
                        },
                    ))
                },
            )
        };

        let delete_team = {
            let editor_state = editor_state.clone();

            Callback::from(move |_| {
                editor_state.dispatch(EditorMessage::RemoveRedWhiteTeam(i.try_into().unwrap()))
            })
        };

        let users: Rc<Vec<UserProps>> = team
            .users
            .iter()
            .map(|user| UserProps {
                username: user.username.clone().into(),
                password: user.password.clone().into(),
            })
            .collect::<Vec<_>>()
            .into();

        let name: AttrValue = team.name.clone().into();

        html! {
            <li>
                <RedWhiteTeamEditorComponent
                    {name}
                    {users}
                    white_team={team.white_team}
                    {modify_red_white_team}
                    {delete_team}
                />
            </li>
        }
    });

    let blue_team_editors = blue_teams.iter().enumerate().map(|(i, team)| {
        let modify_blue_team = {
            let editor_state = editor_state.clone();

            Callback::from(
                move |(name, users, id): (AttrValue, Rc<Vec<UserProps>>, u8)| {
                    editor_state.dispatch(EditorMessage::EditBlueTeam(
                        i.try_into().unwrap(),
                        BlueTeamEditor {
                            name: name.to_string(),
                            users: users
                                .iter()
                                .map(|user| User {
                                    username: user.username.to_string(),
                                    password: user.password.to_string(),
                                })
                                .collect(),
                            id,
                        },
                    ))
                },
            )
        };

        let delete_team = {
            let editor_state = editor_state.clone();

            Callback::from(move |_| {
                editor_state.dispatch(EditorMessage::RemoveBlueTeam(i.try_into().unwrap()))
            })
        };

        let users: Rc<Vec<UserProps>> = team
            .users
            .iter()
            .map(|user| UserProps {
                username: user.username.clone().into(),
                password: user.password.clone().into(),
            })
            .collect::<Vec<_>>()
            .into();

        let name: AttrValue = team.name.clone().into();

        html! {
            <li>
                <BlueTeamEditorComponent
                    {name}
                    {users}
                    id={team.id}
                    {modify_blue_team}
                    {delete_team}
                />
            </li>
        }
    });

    let debug_click = {
        let blue_teams = blue_teams.clone();
        let red_white_teams = red_white_teams.clone();

        Callback::from(move |_| {
            log::info!("Red teams: {red_white_teams:?}");
            log::info!("Blue teams: {blue_teams:?}");
        })
    };

    html! {
        <main id="teams">
            <div class="red-white-team-list">
                <h3>{ "Red and white teams" }</h3>

                <div>
                    <h4>{ "Add new red or white team" }</h4>

                    <div class="form-submit">
                        <div class="form-submit-button">
                            <a href="#" onclick={add_new_red_white_team}>
                                { "Add new team" }
                            </a>
                        </div>
                    </div>
                </div>

                <ul>
                    { for red_team_editors }
                </ul>
            </div>

            <div class="blue-team-list">
                <h3>{ "Blue teams" }</h3>

                <div>
                    <h4>{ "Add new blue team" }</h4>

                    <div class="form-submit">
                        <div class="form-submit-button">
                            <a href="#" onclick={add_new_blue_team}>
                                { "Add new team" }
                            </a>
                        </div>
                    </div>
                </div>

                <ul>
                    { for blue_team_editors }
                </ul>
            </div>

            <a href="#" onclick={debug_click}>
                { "Test" }
            </a>
        </main>
    }
}
