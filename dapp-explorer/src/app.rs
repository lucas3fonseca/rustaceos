use yew::{html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};

use crate::chain::ChainStatus;
use crate::components::Header;

pub struct App {
    chain: ChainStatus,
    clicked: bool,
    onclick: Callback<ClickEvent>,
}

pub enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            chain: ChainStatus::default(),
            clicked: false,
            onclick: link.callback(|_| Msg::Click),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = !self.clicked;
                true // re-render
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked { "Clicked" } else { "Click me" };

        html! {
            <>
                <Header chain=&self.chain />
                <section class="section">
                    <div class="container">
                        <button class="button" onclick=&self.onclick>{ button_text }</button>
                    </div>
                </section>
            </>
        }
    }
}
