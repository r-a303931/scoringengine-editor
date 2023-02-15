// machines.rs: Machine configuration menu
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

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{config::MachineEditor, state};

macro_rules! count_properties {
    () => (0usize);
    ($p:ident,$($p2:ident,)*) => (1usize + count_properties!($($p2,)*));
}

macro_rules! define_service_account_editor {
    ($props:expr, None) => {};
    ($props:expr, Some(vec![])) => {};
}

macro_rules! define_service_environment_editor {
    (None, $props:expr, ) => {};
    (vec![], $props:expr, $($property:ident => $property_name:expr),*) => {};
}

macro_rules! setup_service {
    (
        ($name:ident, $pretty_name:expr),
        ServiceEditor {
            name => $new_name:expr,
            port => $new_port:expr,
            points => $new_points:expr,
            accounts => $new_accounts:expr,
            definition => $new_service:expr
        },
        ($($property:ident => $prop_pretty_name:expr),*)
    ) => {
        setup_service!{
            ($name, $pretty_name),
            ServiceEditor {
                name => $new_name,
                port => $new_port,
                points => $new_points,
                accounts => $new_accounts,
                definition => $new_service, vec![]
            },
            ($($property => $prop_pretty_name),*)
        }
    };
    (
        ($name:ident, $pretty_name:expr),
        ServiceEditor {
            name => $new_name:expr,
            port => $new_port:expr,
            points => $new_points:expr,
            accounts => $new_accounts:expr,
            definition => $new_service:expr, $new_service_params:expr
        },
        ($($property:ident => $prop_pretty_name:expr),*)
    ) => {
        mod $name {
            use crate::config::{self, ServiceEditor};
            use yew::prelude::*;

            #[derive(Properties, PartialEq)]
            pub struct NewServiceComponentProps {
                pub handle_pickup: Callback<config::ServiceEditor>,
                pub handle_dragend: Callback<()>,
            }

            #[function_component]
            pub fn NewServiceComponent(props: &NewServiceComponentProps) -> Html {
                let ondragstart = {
                    let handle_pickup = props.handle_pickup.clone();

                    Callback::from(move |_| {
                        handle_pickup.emit(ServiceEditor {
                            name: $new_name.to_string(),
                            port: $new_port,
                            points: $new_points,
                            accounts: $new_accounts,
                            definition: $new_service($new_service_params),
                        });
                    })
                };

                let ondragend = {
                    let handle_dragend = props.handle_dragend.clone();

                    Callback::from(move |_| {
                        handle_dragend.emit(());
                    })
                };

                html! {
                    <div {ondragstart} draggable={"true"} class="new-service" {ondragend}>
                        <h3>
                            { $pretty_name }
                        </h3>

                        <div class="service-details">
                            if $new_port != 0 {
                                <div class="service-detail">
                                    <span>{ "Default port: " }</span>
                                    { ($new_port).to_string() }
                                </div>
                            }

                            <div class="service-detail">
                                <span>{ "Default points: " }</span>
                                { ($new_points).to_string() }
                            </div>

                            <div class="service-detail">
                                <span>{ "Accounts: " }</span>
                                { if {
                                    let new_accounts: Option<Vec<config::User>> = $new_accounts;
                                    new_accounts.is_some()
                                } {
                                    "Yes"
                                } else {
                                    "No"
                                } }
                            </div>
                        </div>

                        if count_properties!($($property,)*) != 0 {
                            <div class="service-environment">
                                <h4>
                                    { "Service properties:" }
                                </h4>

                                $(
                                    <div class="service-property">
                                        { $prop_pretty_name }
                                    </div>
                                )*
                            </div>
                        }
                    </div>
                }
            }
        }
    };
}

setup_service! {
    (dns, "DNS"),
    ServiceEditor {
        name => "DNS",
        port => 53,
        points => 100,
        accounts => None,
        definition => config::ServiceDefinition::Dns
    },
    (
        qtype => "Query type",
        domain => "Domain"
    )
}
setup_service! {
    (icmp, "ICMP Ping"),
    ServiceEditor {
        name => "ICMP",
        port => 0,
        points => 25,
        accounts => None,
        definition => config::ServiceDefinition::Icmp, None
    },
    ()
}
setup_service! {
    (ssh, "SSH"),
    ServiceEditor {
        name => "SSH",
        port => 0,
        points => 25,
        accounts => Some(vec![]),
        definition => config::ServiceDefinition::Ssh
    },
    (
        commands => "Commands"
    )
}

#[derive(Properties, PartialEq)]
struct MachineEditorProps {
    i: u8,
    machine: MachineEditor,
}

#[function_component]
fn MachineEditorComponent(props: &MachineEditorProps) -> Html {
    let editor_state = use_context::<crate::state::EditorStateContext>().unwrap();

    let editing_name = use_state(bool::default);
    let editing_name_ref = use_node_ref();

    let start_editing_name = {
        let editing_name = editing_name.clone();
        Callback::from(move |_| editing_name.set(true))
    };

    let stop_editing_name = {
        let editing_name = editing_name.clone();
        Callback::from(move |_| editing_name.set(false))
    };

    {
        let editing_name_ref = editing_name_ref.clone();
        let editing_name_ref2 = editing_name_ref.clone();
        use_effect_with_deps(
            move |_| {
                let Some(input) = editing_name_ref.cast::<HtmlInputElement>() else { return; };
                input.focus().unwrap();
            },
            editing_name_ref2.get(),
        );
    }

    let update_name = {
        let editor_state = editor_state.clone();
        let editing_name = editing_name.clone();
        let editing_name_ref = editing_name_ref.clone();
        let machine = props.machine.clone();
        let i = props.i;

        Callback::from(move |_| {
            let Some(input) = editing_name_ref.cast::<HtmlInputElement>() else { return; };
            let mut new_machine = machine.clone();
            new_machine.name = input.value().clone();
            editor_state.dispatch(state::EditorMessage::UpdateMachine(i, new_machine));
            editing_name.set(false);
        })
    };

    let delete_machine = {
        let editor_state = editor_state.clone();
        let i = props.i;

        Callback::from(move |_| {
            editor_state.dispatch(state::EditorMessage::RemoveMachine(i));
        })
    };

    let ondragover = {
        let editor_state = editor_state.clone();
        let machine_name = props.machine.name.clone();
        let i = props.i;
        let is_editing = *editing_name;

        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            if editor_state.force_init().5.is_none() || machine_name.is_empty() || is_editing {
                return;
            }
            editor_state.dispatch(state::EditorMessage::HoverOverMachine(i));
        })
    };

    let ondragleave = {
        let editor_state = editor_state.clone();

        Callback::from(move |_| {
            editor_state.dispatch(state::EditorMessage::StopHoveringOverMachines);
        })
    };

    let ondrop = {
        let editor_state = editor_state.clone();
        let i = props.i;

        Callback::from(move |e: DragEvent| {
            e.prevent_default();

            editor_state.dispatch(state::EditorMessage::DropService(i));
        })
    };

    let hovering_class = Some("hovering").filter(|_| {
        editor_state
            .force_init()
            .4
            .map(|hovering| hovering == props.i)
            .unwrap_or(false)
    });

    html! {
        <div {ondragover} {ondragleave} {ondrop} class={classes!("machine", hovering_class)}>
            <div class="machine-header">
                <div class="machine-name">
                    if *editing_name || props.machine.name.is_empty() {
                        <input
                            ref={editing_name_ref}
                            onchange={update_name}
                            onblur={stop_editing_name}
                            value={props.machine.name.clone()}
                        />
                    } else {
                        <h3 onclick={start_editing_name}>
                            { props.machine.name.clone() }
                        </h3>
                    }
                </div>

                <a href="#" onclick={delete_machine}>
                    { "Delete machine" }
                </a>
            </div>

            <div class="machine-body">
                <div class="machine-properties">
                    <div class="">

                    </div>
                </div>

                <div class="machine-services">

                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn MachineConfiguration() -> Html {
    let editor_state = use_context::<crate::state::EditorStateContext>().unwrap();
    let config = editor_state.force_init().2;

    let handle_pickup = {
        let editor_state = editor_state.clone();

        Callback::from(move |new_service| {
            editor_state.dispatch(state::EditorMessage::PickupService(new_service));
        })
    };

    let handle_dragend = {
        let editor_state = editor_state.clone();

        Callback::from(move |()| {
            editor_state.dispatch(state::EditorMessage::StopHoveringOverMachines);
        })
    };

    let add_machine = {
        let editor_state = editor_state.clone();

        Callback::from(move |_| {
            editor_state.dispatch(state::EditorMessage::AddMachine(MachineEditor {
                name: "".to_owned(),
                ip_offset: None,
                ip_template: "".to_owned(),
                services: vec![],
            }));
        })
    };

    let machine_list = config.machines.iter().enumerate().map(|(i, machine)| {
        let i: u8 = i.try_into().unwrap();

        html! {
            <MachineEditorComponent
                key={i}
                {i}
                machine={machine.clone()}
            />
        }
    });

    html! {
        <main id="machines">
            <div class="service-list">
                <dns::NewServiceComponent handle_pickup={handle_pickup.clone()} handle_dragend={handle_dragend.clone()} />
                <icmp::NewServiceComponent handle_pickup={handle_pickup.clone()} handle_dragend={handle_dragend.clone()} />
                <ssh::NewServiceComponent handle_pickup={handle_pickup.clone()} handle_dragend={handle_dragend.clone()} />
            </div>

            <div class="machine-list-header">
                <a href="#" onclick={add_machine}>
                    { "Add machine" }
                </a>
            </div>

            <div class="machine-list">
                { for machine_list }
            </div>
        </main>
    }
}
