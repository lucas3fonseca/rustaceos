use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::chain::ChainStatus;
use crate::components::Header;
use crate::components::Test;

pub struct App {
    chain: ChainStatus,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            chain: ChainStatus::default(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header chain=&self.chain />
                <section class="section">
                    <div class="container">
                        <Test />
                    </div>
                </section>
            </>
        }
    }
}
