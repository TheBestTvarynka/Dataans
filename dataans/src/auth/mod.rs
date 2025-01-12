use leptos::ev::SubmitEvent;
use leptos::{
    component, create_signal, html, view, Action, IntoView, NodeRef, Show, SignalGet, SignalSet, WriteSignal,
};

use crate::backend::auth::{sign_in, sign_up};
use crate::notes::md_node::InlineCode;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
enum WindowMode {
    SignIn,
    #[default]
    SignUp,
}

#[component]
fn SignIn(set_mode: WriteSignal<WindowMode>) -> impl IntoView {
    let toaster = leptoaster::expect_toaster();

    let (is_success, set_is_success) = create_signal(false);

    let secret_key_input: NodeRef<html::Input> = NodeRef::new();
    let username_input: NodeRef<html::Input> = NodeRef::new();
    let password_input: NodeRef<html::Input> = NodeRef::new();

    let t = toaster.clone();
    let sign_in = Action::new(move |data: &(Option<Vec<u8>>, String, String)| {
        let t = t.clone();

        let (secret_key, username, password) = data.clone();

        async move {
            try_exec!(sign_in(secret_key, username, password).await, "Failed to sign in", t);
            set_is_success.set(true);
        }
    });

    let sign_in = move |ev: SubmitEvent| {
        ev.prevent_default();

        if is_success.get() {
            set_is_success.set(false);
        }

        let secret_key = secret_key_input.get().expect("<input> should be mounted").value();
        let secret_key = if !secret_key.is_empty() {
            Some(try_exec!(
                hex::decode(secret_key),
                "Invitation token should be hex encoded",
                toaster
            ))
        } else {
            None
        };
        let username = username_input.get().expect("<input> should be mounted").value();
        let password = password_input.get().expect("<input> should be mounted").value();

        sign_in.dispatch((secret_key, username, password));
    };

    view! {
        <form class="auth-form" on:submit=sign_in>
            <span class="auth-form-label">"Secret key:"</span>
            <span
                class="auth-form-label"
                style="font-size: 0.8em; width: 100%"
            >
                "(leave empty to load the existing secret key file)"
            </span>
            <input class="auth-form-input input" placeholder="invitation token" type="text" node_ref=secret_key_input />
            <span class="auth-form-label">"Username:"</span>
            <input class="auth-form-input input" placeholder="username" type="text" node_ref=username_input />
            <span class="auth-form-label">"Password:"</span>
            <input class="auth-form-input input" placeholder="password" type="password" node_ref=password_input />
            <button
                class="auth-form-button button_ok"
                type="submit"
            >
                "Sign in"
            </button>
            <button
                class="auth-form-button button_cancel"
                on:click=move |_| set_mode.set(WindowMode::SignUp)
            >
                "Don't have an account? Sign up."
            </button>
            <Show when=move || is_success.get()>
                {move || { view! { <span class="auth-form-label-success">"Sign in successful!"</span> } }}
            </Show>
        </form>
    }
}

#[component]
fn SignUp(set_mode: WriteSignal<WindowMode>) -> impl IntoView {
    let toaster = leptoaster::expect_toaster();

    let (user_id, set_user_id) = create_signal(None);

    let invitation_token_input: NodeRef<html::Input> = NodeRef::new();
    let username_input: NodeRef<html::Input> = NodeRef::new();
    let password_input: NodeRef<html::Input> = NodeRef::new();

    let t = toaster.clone();
    let sign_up = Action::new(move |data: &(Vec<u8>, String, String)| {
        let t = t.clone();

        let (invitation_token, username, password) = data.clone();

        async move {
            set_user_id.set(Some(try_exec!(
                sign_up(invitation_token, username, password).await,
                "Failed to sign up",
                t
            )));
        }
    });

    let sign_up = move |ev: SubmitEvent| {
        ev.prevent_default();

        if user_id.get().is_some() {
            set_user_id.set(None);
        }

        let invitation_token = invitation_token_input.get().expect("<input> should be mounted").value();
        let invitation_token = try_exec!(
            hex::decode(invitation_token),
            "Invitation token should be hex encoded",
            toaster
        );
        let username = username_input.get().expect("<input> should be mounted").value();
        let password = password_input.get().expect("<input> should be mounted").value();

        sign_up.dispatch((invitation_token, username, password));
    };

    view! {
        <form class="auth-form" on:submit=sign_up>
            <span class="auth-form-label">"Invitation token:"</span>
            <input class="auth-form-input input" placeholder="invitation token" type="text" node_ref=invitation_token_input />
            <span class="auth-form-label">"Username:"</span>
            <input class="auth-form-input input" placeholder="username" type="text" node_ref=username_input />
            <span class="auth-form-label">"Password:"</span>
            <input class="auth-form-input input" placeholder="password" type="password" node_ref=password_input />
            <button
                class="auth-form-button button_ok"
                type="submit"
            >
                "Sign up"
            </button>
            <button
                class="auth-form-button button_cancel"
                on:click=move |_| set_mode.set(WindowMode::SignIn)
            >
                "Already have an account? Sign in."
            </button>
            <Show when=move || user_id.get().is_some()>
                {move || {
                    let user_id = user_id.get().unwrap();
                    view! {
                        <span class="auth-form-label-success">
                            "Sign up successful! Here is your user ID: "
                            <InlineCode code={user_id.to_string()} />
                            " (you don't need to remember it)."
                        </span>
                    }
                }}
            </Show>
        </form>
    }
}

#[component]
pub fn AuthWindow() -> impl IntoView {
    let (mode, set_mode) = create_signal(WindowMode::default());

    view! {
        <div class="auth-window">
            {move || match mode.get() {
                WindowMode::SignIn => view! { <SignIn set_mode /> },
                WindowMode::SignUp => view! { <SignUp set_mode /> },
            }}
        </div>
    }
}
