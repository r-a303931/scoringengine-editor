// output.rs: Shows the output configuration file
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

use web_sys::{window, Document, HtmlElement};
use yew::prelude::*;

use crate::{config::convert_editor_to_final, error::EditorError};

#[function_component]
pub fn ConfigurationOutput() -> Html {
    let editor_state = use_context::<crate::state::EditorStateContext>().unwrap();
    let (_, _, config, _, _, _) = editor_state.force_init();

    let text_display_ref = use_node_ref();

    let result = convert_editor_to_final(config)
        .map_err(EditorError::Conversion)
        .and_then(|(conf, _)| serde_yaml::to_string(&conf).map_err(EditorError::Serialize));

    let onclick = {
        let text_display_ref = text_display_ref.clone();

        Callback::from(move |_| {
            let range = Document::new().unwrap().create_range().unwrap();
            range
                .select_node(&text_display_ref.cast::<HtmlElement>().unwrap())
                .unwrap();
            let window = window().unwrap();
            let selection = window.get_selection().unwrap();

            if let Some(sel) = selection {
                sel.remove_all_ranges().unwrap();
                sel.add_range(&range).unwrap();
            }
        })
    };

    html! {
        <main id="output">
            if let Err(err) = &result {
                <div id="error">
                    {err}
                </div>
            }

            <pre ref={text_display_ref} {onclick}>
                { "---\n" }
                if let Ok(yaml) = &result {
                    {yaml}
                }
                if let Ok(yaml) = &result {
                    { "\n\nflags: []" }
                }
            </pre>
        </main>
    }
}
