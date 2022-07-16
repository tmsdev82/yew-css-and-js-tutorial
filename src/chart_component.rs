use gloo::timers::callback::Timeout;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::bindings::MyChart;


pub enum Msg {
    Draw,
    AddValue(i32),
    DoNothing
}

fn parse_input_val(input_ref: NodeRef) -> Msg {
    let num_input_element = input_ref.cast::<HtmlInputElement>().unwrap();
    let new_value = num_input_element.value();
    num_input_element.set_value("");
    match new_value.parse() {
        Ok(val) => Msg::AddValue(val),
        Err(_) => {
            log::error!("Error occurred parsing '{}'", new_value);
            Msg::DoNothing
        }
    }
}

pub struct ChartComponent {
    pub chart: MyChart,
    pub input_ref: NodeRef,
    pub draw_timer: Timeout
}


impl Component for ChartComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link();
        let stand_alone_timer = {
            let link = link.clone();
            Timeout::new(10, move|| {
                link.send_message(Msg::Draw)
            })
        };
        Self {
            chart: MyChart::new(),
            draw_timer: stand_alone_timer,
            input_ref: NodeRef::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Draw => {
                self.chart.draw("myChart");
                true
            },
            Msg::AddValue(val) => {
                log::debug!("Adding value: {}", val);
                self.chart.update(val);
                true
            },
            Msg::DoNothing => {
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_key_press = {
            let cur_input_ref = self.input_ref.clone();
            link.callback(move| e: KeyboardEvent| {
                match e.key().as_str() {
                    "Enter" => {
                        parse_input_val(cur_input_ref.clone())
                    },
                    _ => {
                        Msg::DoNothing
                    }
                }
            })
        };

        let on_click = {
            let cur_input_ref = self.input_ref.clone();
            link.callback(move |_e: MouseEvent| {
                parse_input_val(cur_input_ref.clone())
            })
        };

        html! {
            <section class="section">
            <div class="container">
                <input onkeypress={on_key_press} ref={self.input_ref.clone()} class="input is-primary" type="number" placeholder="input value for chart"/>
                <button class="button" onclick={on_click}>{ "Add value" }</button>
                <canvas id="myChart" width="600" height="500"></canvas>
            </div>
            </section>
        }
    }
}