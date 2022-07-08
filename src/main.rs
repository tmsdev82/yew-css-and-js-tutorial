use yew::prelude::*;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="section">
                <div class="container">
                    <h1 class="title">{"Main page"}</h1>
                </div>
            </div>
        }   
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
