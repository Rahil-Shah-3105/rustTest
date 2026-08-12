use dioxus::{events::FormData, prelude::*};

#[derive(Props)]
pub struct FormInputProps<'a> {
    oninput: EventHandler<'a, FormData>,

    #[props(optional)]
    placeholder: Option<String>,

    #[props(optional)]
    value: Option<String>,
}

pub fn FormInput_Lg<'a>(cx: Scope<'a, FormInputProps<'a>>) -> Element {
	log::debug!(">>> Entering FormInput_Lg(cx=(unprintable))");
    let ph = cx.props.placeholder.clone().unwrap_or_default();
    let value = cx.props.value.clone().unwrap_or_default();
	log::debug!("FormInput_Lg(cx=(unprintable)): ph → (unprintable)");
    cx.render(rsx! {
		log::debug!("FormInput_Lg(cx=(unprintable)): value → (unprintable)");
        log::debug!("<<< Exiting FormInput_Lg(cx=(unprintable))");
        fieldset {
            class: "form-group",
            input {
                class: "form-control form-control-lg",
                r#type: "text",
                oninput: move |evt| cx.props.oninput.call(evt.data.as_ref().clone()),
                placeholder: "{ph}",
                value: "{value}",
            }
        }
    })
}
