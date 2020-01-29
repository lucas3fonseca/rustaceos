use yew::agent::Bridged;
use yew::worker::Bridge;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::ship::{ConnectionStatus, Request as ShipRequest, Response as ShipResponse, ShipWorker};

pub struct Header {
    link: ComponentLink<Self>,
    ship: Box<dyn Bridge<ShipWorker>>,
    connection: ConnectionStatus,
    block_num: u32,
}

pub enum Msg {
    ShipMsg(ShipResponse),
    ConnectShip,
}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let ship_callback = link.callback(|message| Msg::ShipMsg(message));
        let mut ship = ShipWorker::bridge(ship_callback);
        ship.send(ShipRequest::Subscribe);
        Header {
            ship,
            link,
            block_num: 0,
            connection: ConnectionStatus::Offline,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShipMsg(ship_response) => match ship_response {
                ShipResponse::Connected => self.connection = ConnectionStatus::Online,
                ShipResponse::Disconnected => self.connection = ConnectionStatus::Offline,
                ShipResponse::UpdatedHeadLib(head, _lib) => self.block_num = head,
            },
            Msg::ConnectShip => self.ship.send(ShipRequest::Connect),
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <nav class="navbar" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <a class="navbar-item" href="/">
                        <img src="/eos.gif" width="28" height="28" />
                    </a>
                </div>
                <div id="navbarBasicExample" class="navbar-menu is-active">
                    <div class="navbar-start">
                        <a class="navbar-item">
                            {"Accounts"}
                        </a>
                        <a class="navbar-item">
                            {"Contracts"}
                        </a>
                    </div>
                    <div class="navbar-end">
                        <div class="navbar-item">
                            {"Block: "}
                            <strong>{self.block_num}</strong>
                        </div>
                        {self.display_chain_status()}
                    </div>
                </div>
            </nav>
        }
    }
}

impl Header {
    fn display_chain_status(&self) -> Html {
        let (text, color) = match self.connection {
            ConnectionStatus::Online => ("Online", "has-text-success"),
            ConnectionStatus::Offline => ("Offline", "has-text-danger"),
        };

        let button = if self.connection == ConnectionStatus::Offline {
            let onclick = self.link.callback(|_| Msg::ConnectShip);
            html! {
                <div class="navbar-item">
                    <button class="button is-small" onclick=onclick>{"Connect"}</button>
                </div>
            }
        } else {
            html! {}
        };

        html! {
            <>
                <div class=("navbar-item", color)>
                    {text}
                </div>
                {button}
            </>
        }
    }
}
