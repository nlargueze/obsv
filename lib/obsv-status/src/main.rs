use yew::prelude::*;

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
        <main>
            <nav>
                {"NAV"}
            </nav>
            <section>
                {"INCIDENTS"}
            </section>
            <section>
                {"GROUP 1"}
            </section>
            <div>
                <button {onclick}>{ "+10" }</button>
                <p>{ *counter }</p>
                <div>
                    {"HEY"}
                </div>
            </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
