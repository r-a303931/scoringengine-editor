use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{config::IpGeneratorScheme, state};

#[function_component]
pub fn IpSettingsEditor() -> Html {
    let editor_state = use_context::<crate::state::EditorStateContext>().unwrap();
    let editor_state_c = editor_state.force_init();
    let machine_count = editor_state_c.2.machines.len();
    let error = editor_state.error();

    let offsetreplace_state = use_state(|| "".to_string());

    let input_node_ref = use_node_ref();

    let manual_class = Some("selected")
        .filter(|_| matches!(editor_state_c.2.ip_generator, IpGeneratorScheme::OneTeam));
    let dumbreplace_class = Some("selected").filter(|_| {
        matches!(
            editor_state_c.2.ip_generator,
            IpGeneratorScheme::ReplaceXWithId
        )
    });

    let set_manual = {
        let editor_state = editor_state.clone();

        Callback::from(move |_| {
            editor_state.dispatch(state::EditorMessage::UpdateIpSettings(
                IpGeneratorScheme::OneTeam,
            ));
        })
    };

    let set_dumb_replace = {
        let editor_state = editor_state.clone();

        Callback::from(move |_| {
            editor_state.dispatch(state::EditorMessage::UpdateIpSettings(
                IpGeneratorScheme::ReplaceXWithId,
            ));
        })
    };

    let update_offsetreplace_state = {
        let input_node_ref = input_node_ref.clone();
        let offsetreplace_state = offsetreplace_state.clone();

        Callback::from(move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                let value = input.value();

                offsetreplace_state.set(value);
            }
        })
    };

    let set_multiplier = {
        let input_node_ref = input_node_ref.clone();
        let editor_state = editor_state.clone();

        Callback::from(move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                let value = input.value();

                match value.parse::<u8>() {
                    Ok(mult) if (mult as usize) < machine_count => {
                        editor_state.dispatch(state::EditorMessage::Error(format!(
                            "Multiplier ({mult}) must be higher than the current machine count ({machine_count})"
                        )))
                    },
                    Ok(mult) => editor_state.dispatch(state::EditorMessage::UpdateIpSettings(
                        IpGeneratorScheme::ReplaceXWithIdTimesMultiplierPlusOffset(mult),
                    )),
                    Err(e) => editor_state.dispatch(state::EditorMessage::Error(format!(
                        "Unable to parse input: {:?}",
                        e
                    ))),
                }
            }
        })
    };

    html! {
        <main id="ipsettings">
            <div class={classes!("ipoption", "manual", manual_class)}>
                <div class="settingheader">
                    <h3>{ "Manual configuration" }</h3>

                    <div class="button-box">
                        <a href="#" onclick={set_manual}>
                            { "Select" }
                        </a>
                    </div>
                </div>

                <div class="description">
                    <p>{ "Simplest option. Works only with one team. Requires providing all the IP addresses for all machines" }</p>
                </div>
            </div>

            <div class={classes!("ipoption", "dumbreplace", dumbreplace_class)}>
                <div class="settingheader">
                    <h3>{ "Simple ID substitution" }</h3>

                    <div class="button-box">
                        <a href="#" onclick={set_dumb_replace}>{ "Select" }</a>
                    </div>
                </div>

                <div class="description">
                    <p>
                        { "This method takes the ID of a team and a template IP address specified by the machine, and replaces all occurrences of the letter X with the ID of the team." }
                    </p>
                </div>
            </div>

            if let Some(msg) = error {
                <div id="error">{ "Error! " } { msg }</div>
            }

            <div class={classes!("ipoption", "offsetreplace")}>
                <div class="settingheader">
                    <h3>{ "ID Offset Multiplier" }</h3>

                    <div class="button-box">
                        <a href="#" onclick={set_multiplier}>{ "Select" }</a>
                    </div>
                </div>


                <div class="description">
                    <p>
                        { "This method takes the ID of a team, multiplies it by the multiplier, and adds an offset (specified by the machines). Given this number, X, it then takes the template IP address (also specified by machine) and replaces all occurrences of the letter X with this new number." }
                    </p>

                    <p>
                        { "When would you want to use this? Say you have 2 teams or divisions, with 12 boxes each. If the multiplier is 15, then given a template like 192.168.1.X it is possible for team 1 to get IPs from 192.168.1.15-192.168.1.29, preventing duplicates with addresses such as 11" }
                    </p>
                </div>


                <div class="form">
                    <label>{ "Current multiplier" }</label>

                    <div>
                        { match editor_state_c.2.ip_generator {
                            IpGeneratorScheme::ReplaceXWithIdTimesMultiplierPlusOffset(x) => x.to_string(),
                            _ => "(none)".to_string(),
                        } }
                    </div>


                    <label>{ "Multiplier" }</label>

                    <div>
                        <input ref={input_node_ref} value={(*offsetreplace_state).clone()} onchange={update_offsetreplace_state} />
                    </div>
                </div>
            </div>
        </main>
    }
}
