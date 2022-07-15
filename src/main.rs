use yew::prelude::*;
use configurable_styling::ConfigurableStylingComponent;
use chart_component::ChartComponent;

mod data_source;
mod configurable_styling;
mod bindings;
mod chart_component;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let data = data_source::get_data();
        let cur_data_html = data.iter().map(|data_point| {
            html! {
                <tr class={match data_point.value {
                    x if x >= 100.0 => "success".to_string(),
                    x if x == 0.0 => "warning".to_string(),
                    x if x < 0.0 => "danger".to_string(),
                    _ => "".to_string()
                }}>
                    <td>{data_point.item_name.clone()}</td>
                    <td>{data_point.quantity}</td>
                    <td>{data_point.value}</td>
                </tr>
            }
        });

        html! {
            <div class="section">
                <div>
                    <div class="container">
                        <h1 class="title">{"Main page"}</h1>
                        <h2 class="subtitle">{bindings::get_now_date()}</h2>
                        <div >
                            <table class="table is-hoverable is-striped">
                                <thead>
                                    <tr>
                                        <th>{"Item"}</th>
                                        <th>{"Quantity"}</th>
                                        <th>{"Value"}</th>
                                    </tr>
                                </thead>
                                <tbody>
                                {for cur_data_html}
                                </tbody>
                            </table>
                        </div>
                        <ConfigurableStylingComponent message="element 1" is_dark_mode={true} has_shadow={true} is_rounded={true} />
                        <ConfigurableStylingComponent message="element 2" is_dark_mode={false} has_shadow={true} is_rounded={true} />
                        <ConfigurableStylingComponent message="element 3" is_dark_mode={false} has_shadow={true} is_rounded={false} />
                    </div>
                </div>
                <div>
                    <ChartComponent />
                </div>
            </div>
        }   
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
