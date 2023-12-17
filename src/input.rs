// input.rs: Configuration file input menu
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

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::state::{EditingState, EditorMessage, EditorStateContext};

#[function_component]
pub fn InitEditor() -> Html {
    let editor_state = use_context::<EditorStateContext>().unwrap();

    let error = &editor_state.error;

    let new_config_name = use_state(String::default);
    let edited_config_name = use_state(Option::<u8>::default);

    let config_name_editor = use_node_ref();

    let set_new_name = {
        let config_name_editor = config_name_editor.clone();
        let new_config_name = new_config_name.clone();

        Callback::from(move |_| {
            let Some(input) = config_name_editor.cast::<HtmlInputElement>() else {
                return;
            };
            new_config_name.set(input.value().into());
        })
    };

    let oncreatenew = {
        let editor_state = editor_state.clone();
        let new_config_name = new_config_name.clone();

        Callback::from(move |_| {
            editor_state.dispatch(EditorMessage::CreateNew(new_config_name.to_string()));
            new_config_name.set(String::default());
        })
    };

    let config_len = editor_state.configs.len();
    let selected_config = match &editor_state.state {
        EditingState::Initializing => None,
        EditingState::HasConfig { config, .. } => Some(*config),
    };

    let configs = editor_state.configs.iter().enumerate().map(|(i, config)| {
        let edit = {
            let editor_state = editor_state.clone();
            Callback::from(move |_| {
                editor_state.dispatch(EditorMessage::FinishInit(i as u8));
            })
        };

        let copy = {
            let editor_state = editor_state.clone();
            let new_config_name = new_config_name.clone();

            Callback::from(move |_| {
                if new_config_name.is_empty() {
                    return;
                }
                editor_state.dispatch(EditorMessage::Copy(new_config_name.to_string(), i as u8));
                new_config_name.set("".to_owned());
            })
        };

        let delete = {
            let editor_state = editor_state.clone();
            Callback::from(move |_| {
                editor_state.dispatch(EditorMessage::DeleteConfig(i as u8));
            })
        };

        let stop_editing = {
            let edited_config_name = edited_config_name.clone();

            Callback::from(move |_| {
                edited_config_name.set(None);
            })
        };

        let start_edit_name = {
            let edited_config_name = edited_config_name.clone();

            Callback::from(move |_| {
                edited_config_name.set(Some(i as u8));
            })
        };

        let edit_name = {
            let editor_state = editor_state.clone();

            Callback::from(move |e: Event| {
                let name_el = e
                    .target()
                    .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                let name = name_el
                    .as_ref()
                    .map(HtmlInputElement::value);
                let Some(name) = name else {
                    return;
                };
                editor_state.dispatch(EditorMessage::EditConfigName(name, i as u8));
            })
        };

        let editing_name = Some(i as u8) == *edited_config_name;

        let service_count: usize = config.config.machines.iter().map(|machine| machine.services.len()).sum();

        html! {
            <div class={classes!(
                "config-row",
                (Some(i as u8) == selected_config).then(|| Some("selected"))
            )}>
                <div class="config-name">
                    if editing_name {
                        <input
                            onchange={edit_name}
                            onblur={stop_editing}
                            value={config.name.clone()}
                        />
                    } else {
                        <span onclick={start_edit_name}>{config.name.clone()}</span>
                    }
                </div>

                <div class="config-details">
                    <div class="config-numbers">
                        { format!(
                            "{} users * {} machine templates * {} services = {} total services across {} machines",
                            config.config.blue_teams.len(),
                            config.config.machines.len(),
                            service_count,
                            config.config.blue_teams.len() * config.config.machines.len() * service_count,
                            config.config.blue_teams.len() * config.config.machines.len()
                        ) }
                    </div>

                    <div class="config-buttons">
                        <a href="#" onclick={edit} class="button">{ "Edit" }</a>
                        <a href="#" onclick={copy} class={classes!(
                            "button",
                            new_config_name.is_empty().then(|| Some("disabled"))
                        )}>{ "Copy" }</a>
                        <a href="#" onclick={delete} class="button">{ "Delete" }</a>
                    </div>
                </div>
            </div>
        }
    });

    html! {
        <main id="input">
            if let Some(msg) = &error {
                <div id="error">{ "Error! " } { msg }</div>
            }

            <h3>{ "Select a configuration file to edit" }</h3>

            <div class="new-config-row">
                <a class={classes!(
                    "button",
                    new_config_name.is_empty().then(|| Some("disabled"))
                )} href="#" onclick={oncreatenew}>
                    { "Or, create a new one:" }
                </a>

                <input
                    ref={config_name_editor}
                    value={(*new_config_name).clone()}
                    oninput={set_new_name}
                    placeholder={"New configuration name"}
                />
            </div>

            <div class="configs">
                { for configs }

                if config_len == 0 {
                    <i>{ "No configurations yet" }</i>
                }
            </div>
        </main>
    }
}
