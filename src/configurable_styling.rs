use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ConfigurableStylingProps {
    pub message: &'static str,
    pub has_shadow: bool,
    pub is_dark_mode: bool,
    pub is_rounded: bool
}

#[function_component(ConfigurableStylingComponent)]
pub fn configurable_styling(props: &ConfigurableStylingProps) -> Html {
    let mut styles = vec!["test-element"];

    if props.has_shadow {
        styles.push("shadow");
    }

    if props.is_dark_mode {
        styles.push("dark");
    }

    if props.is_rounded {
        styles.push("rounded");
    }

    html! {
        <div class={styles}>
        {props.message}
        </div>
    }
}