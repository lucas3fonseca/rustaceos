use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::components::Header;
use crate::components::Test;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header />
                <section class="section">
                    <div class="container">
                        <Test />
                    </div>
                </section>
            </>
        }
    }
}
