use std::rc::Rc;
use yew::prelude::*;

/*
#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick} class="theme_dark">{"Dank mode"}</button>
            <div>{"Hej ling"}</div>
            <p>{ *counter }</p>
        </div>
    }
}
*/

#[derive(Clone, Copy, Debug, PartialEq)]
struct Theme {
    gamerness: Gamerness,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Gamerness {
    DarkMode,
    NoobMode,
}

#[function_component]
fn NavButton() -> Html {
    let theme = use_context::<Rc<Theme>>().expect("Failed to get context");

    let color = match theme.gamerness {
        Gamerness::DarkMode => "blue",
        Gamerness::NoobMode => "yellow",
    }
    .to_string();

    //let onclick = { color };

    html! {
        <div style={format!("background-color: {}", color)}>{"hejsan svejsan"}</div>
        /*<div>
            <button {onclick} class="theme_dark">{"Dank mode"}</button>
            <div>{"Hej ling"}</div>
            <p>{ color }</p>
        </div>*/
    }
}

#[function_component]
fn App() -> Html {
    let theme = use_state(|| Theme {
        gamerness: Gamerness::NoobMode,
    });

    html! {
        <ContextProvider<Rc<Theme>> context={Rc::new(*theme)}>
            <NavButton />
        </ContextProvider<Rc<Theme>>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
