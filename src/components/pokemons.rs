use serde_derive::Deserialize;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    ConsoleService,
};

pub enum Msg {
    GetPokemons,
    Resp(Result<Vec<Pokemon>, anyhow::Error>),
}

struct Pokemons {
    link: ComponentLink<Self>,
    pokemons: Option<Vec<Pokemon>>,
    fetch_pokemons: Option<FetchTask>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pokemon {
    pub name: String,
    pub url: String,
}

impl Component for Pokemons {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetPokemons);
        Self {
            link,
            pokemons: None,
            fetch_pokemons: None,
        }
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::GetPokemons => {
                self.pokemons = None;
                let req = Request::get("https://pokeapi.co/api/v2/pokemon?limit=1&offset=0")
                    .body(Nothing)
                    .expect("can maque req to pokeapi");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<Pokemon>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create pokemons");
                self.fetch_pokemons = Some(task);
                ()
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }
}
