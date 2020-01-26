use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::chain::{ChainStatus, ConnectionStatus};

pub struct Header {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub chain: ChainStatus,
}

pub enum Msg {}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
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
                            <strong>{self.props.chain.block_num}</strong>
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
        let (text, color) = match self.props.chain.connection {
            ConnectionStatus::Online => ("Online", "has-text-success"),
            ConnectionStatus::Offline => ("Offline", "has-text-danger"),
        };

        html! {
            <div class=("navbar-item", color)>
                {text}
            </div>
        }
    }
}
