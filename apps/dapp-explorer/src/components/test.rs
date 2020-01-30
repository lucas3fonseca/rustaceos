use chrono::prelude::*;
use log::{debug, info};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Test {
    counter: i32,
    link: ComponentLink<Self>,
}

#[derive(Clone)]
pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

impl Component for Test {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Test { counter: 0, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.counter += 1;
                info!("incrementing counter to {}", self.counter);
                true
            }
            Msg::Decrement => {
                self.counter -= 1;
                debug!("decrementing counter to {}", self.counter);
                true
            }
            Msg::Bulk(msgs) => {
                for msg in msgs {
                    self.update(msg);
                }
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.action_button(Msg::Increment) }
                { self.action_button(Msg::Decrement) }
                { self.action_button(Msg::Bulk(vec![Msg::Increment, Msg::Increment])) }
                <p>{ self.counter }</p>
                <p>{ Local::now() }</p>
            </>
        }
    }
}

impl Test {
    fn action_button(&self, msg: Msg) -> Html {
        let (text, color) = match msg {
            Msg::Increment => ("Increment", "is-info"),
            Msg::Decrement => ("Decrement", "is-danger"),
            Msg::Bulk(_) => ("Bulk", "is-success"),
        };

        html! {
            <button class=("button", color) onclick=self.link.callback(move |_| msg.clone())>
                {text}
            </button>
        }
    }
}
