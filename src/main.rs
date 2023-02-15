// main.rs: Main page and layout and instantiator of state
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

use state::EditorState;
use yew::prelude::*;

mod config;
mod error;
mod state;

mod input;
mod ipsettings;
mod machines;
mod output;
mod users;

#[function_component]
fn NavBar() -> Html {
    let editor_state = use_context::<state::EditorStateContext>().unwrap();

    let (allow_others, current_view) = match &*editor_state {
        EditorState::HasConfig { current_view, .. } => (true, *current_view),
        _ => (false, state::CurrentView::Input),
    };

    macro_rules! define_view_change_callback {
        ($event:expr) => {{
            let editor_state_clone = editor_state.clone();
            Callback::from(move |_: MouseEvent| {
                editor_state_clone.dispatch(state::EditorMessage::ChangeToView($event))
            })
        }};
    }

    macro_rules! class_currently_selected {
        ($view:expr) => {{
            if current_view == $view {
                classes!(Some("selected"))
            } else if !allow_others {
                classes!(Some("inactive"))
            } else {
                classes!()
            }
        }};
    }

    let input_class = classes!(if current_view == state::CurrentView::Input {
        Some("selected")
    } else {
        None
    });

    let error_message = if !allow_others {
        "Please input a configuration file to edit"
    } else {
        ""
    };

    html! {
        <nav>
            <ul>
                <li class={input_class}>
                    <a href="#" onclick={define_view_change_callback!(state::CurrentView::Input)}>
                        { "Input config" }
                    </a>
                </li>
                <li class={class_currently_selected!(state::CurrentView::Teams)} title={error_message}>
                    <a href="#" onclick={define_view_change_callback!(state::CurrentView::Teams)}>
                        { "Teams" }
                    </a>
                </li>
                <li class={class_currently_selected!(state::CurrentView::Machines)} title={error_message}>
                    <a href="#" onclick={define_view_change_callback!(state::CurrentView::Machines)}>
                        { "Machines" }
                    </a>
                </li>
                <li class={class_currently_selected!(state::CurrentView::IpSettings)} title={error_message}>
                    <a href="#" onclick={define_view_change_callback!(state::CurrentView::IpSettings)}>
                        { "IP Settings" }
                    </a>
                </li>
                <li class={class_currently_selected!(state::CurrentView::Output)} title={error_message}>
                    <a href="#" onclick={define_view_change_callback!(state::CurrentView::Output)}>
                        { "Generated config" }
                    </a>
                </li>
            </ul>
        </nav>
    }
}

#[function_component]
fn MainContent() -> Html {
    let editor_state = use_context::<state::EditorStateContext>().unwrap();

    use state::CurrentView::*;

    match &*editor_state {
        state::EditorState::Initializing { .. } => html! {
            <input::InitEditor />
        },
        state::EditorState::HasConfig { current_view, .. } => match current_view {
            Input => html! {
                <input::InitEditor />
            },
            IpSettings => html! {
                <ipsettings::IpSettingsEditor />
            },
            Teams => html! {
                <users::TeamsEditor />
            },
            Output => html! {
                <output::ConfigurationOutput />
            },
            Machines => html! {
                <machines::MachineConfiguration />
            },
        },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <state::EditorStateProvider>
            <header>
                <h2>{ "Scoring Engine Configuration Editor" }</h2>
            </header>

            <NavBar />

            <MainContent />
        </state::EditorStateProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
