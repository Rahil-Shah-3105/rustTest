use dioxus::prelude::*;
use dioxus_router::use_router;
use dioxus_use_storage::use_session_storage;

use crate::commons::{AppState, TOKEN};

pub fn SignOutPage(cx: Scope) -> Element {
	log::debug!(">>> Entering SignOutPage(cx=(unprintable))");
    //
    use_session_storage(&cx).remove(TOKEN);
    let app_state = use_shared_state::<AppState>(cx).unwrap();
	log::debug!("SignOutPage(cx=(unprintable)): app_state → (unprintable)");
    app_state.write().token = None;
    use_router(&cx).push_route("/home", None, None);

	log::debug!("<<< Exiting SignOutPage(cx=(unprintable))");
    cx.render(rsx! {
        div {
            h2 { "Signed out" }
        }
    })
}
