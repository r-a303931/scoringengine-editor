use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
pub fn InitEditor() -> Html {
    let editor_state = use_context::<crate::state::EditorStateContext>().unwrap();

    let input_node_ref = use_node_ref();

    let (editor_box, error) = match &*editor_state {
        crate::state::EditorState::HasConfig {
            ref editor_box,
            ref error,
            ..
        } => (editor_box, error),
        crate::state::EditorState::Initializing {
            ref editor_box,
            ref error,
        } => (editor_box, error),
    };

    let onchange = {
        let input_node_ref = input_node_ref.clone();
        let editor_state = editor_state.clone();

        Callback::from(move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                let value = input.value();

                editor_state.dispatch(crate::state::EditorMessage::UpdateInitialEditor(value));
            }
        })
    };

    let onfinish = {
        let editor_state = editor_state.clone();

        Callback::from(move |_| {
            editor_state.dispatch(crate::state::EditorMessage::FinishInit);
        })
    };

    let oncreatenew = {
        let editor_state = editor_state.clone();

        Callback::from(move |_| {
            editor_state.dispatch(crate::state::EditorMessage::CreateNew);
        })
    };

    html! {
        <main id="input">
            if let Some(msg) = &error {
                <div id="error">{ "Error! " } { msg }</div>
            }

            <h3>{ "Enter the configuration file to edit" }</h3>

            <p>{ "This must be a file that was generated by this program previously" }</p>

            <textarea value={editor_box.clone()} onchange={onchange} ref={input_node_ref} />

            <a href="#" onclick={onfinish}>{ "Load configuration" }</a>
            <a href="#" onclick={oncreatenew}>{ "Create new configuration" }</a>
        </main>
    }
}
