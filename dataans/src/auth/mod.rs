use leptos::{component, create_signal, view, IntoView, SignalGet, SignalSet};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
enum WindowMode {
    SignIn,
    #[default]
    SignUp,
}

#[component]
pub fn AuthWindow() -> impl IntoView {
    let (mode, set_mode) = create_signal(WindowMode::default());

    view! {
        <div class="auth-window">
            <form class="auth-form">
                {move || match mode.get() {
                    WindowMode::SignIn => view! {
                        <span class="auth-form-label">"Secret key:"</span>
                        <span
                            class="auth-form-label"
                            style="font-size: 0.8em; width: 100%"
                        >
                            "(leave empty to load the existing secret key file)"
                        </span>
                        <input class="auth-form-input input" placeholder="invitation token" type="text" />
                    },
                    WindowMode::SignUp => view! {
                        <span class="auth-form-label">"Invitation token:"</span>
                        <input class="auth-form-input input" placeholder="invitation token" type="text" />
                    },
                }}
                <span class="auth-form-label">"Username:"</span>
                <input class="auth-form-input input" placeholder="username" type="text" />
                <span class="auth-form-label">"Password:"</span>
                <input class="auth-form-input input" placeholder="password" type="password" />
                {move || match mode.get() {
                    WindowMode::SignIn => view! {
                        <button
                            class="auth-form-button button_ok"
                        >
                            "Sign in"
                        </button>
                        <button
                            class="auth-form-button button_cancel"
                            on:click=move |_| set_mode.set(WindowMode::SignUp)
                        >
                            "Don't have an account? Sign up."
                        </button>
                    },
                    WindowMode::SignUp => view! {
                        <button
                            class="auth-form-button button_ok"
                        >
                            "Sign up"
                        </button>
                        <button
                            class="auth-form-button button_cancel"
                            on:click=move |_| set_mode.set(WindowMode::SignIn)
                        >
                            "Already have an account? Sign in."
                        </button>
                    },
                }}
            </form>
        </div>
    }
}
