use serde_derive::Deserialize;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Pokemon{
    name: String,
    url: String
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct PokemonResult{
    count: i32,
    next: String,
    previous: Option<i32>,
    results: Option<Vec<Pokemon>>
}

pub enum Msg{
    GetPokemon(),
    Resp(Result<PokemonResult, anyhow::Error>)
}

pub struct Pokemons{
    link: ComponentLink<Self>,
    pokemons: Option<PokemonResult>,
    fetch_pokemon: Option<FetchTask>
}

impl Pokemons {
    fn render_pokemons(&self, pokemon_result: &Option<PokemonResult>) -> Html{
        match pokemon_result {
            Some(p) => {
                let pokemons = &p.results;
                html! {
                    <div>
                        { pokemons.iter().map(|pokemons| self.view_pokemons(pokemons)).collect::<Html>() }
                    </div>
                }
            }
            None => {
                html! {
                    <div>{"loading..."}</div>
                }
            }
        }
    }

    fn view_pokemons(&self, pokemons : &Vec<Pokemon>) -> Html{
        yew::services::ConsoleService::info(&format!("{:?}", pokemons));
        html! {
            <div>
                { pokemons.iter().map(|pokemon| self.view_pokemon(pokemon)).collect::<Html>() }
            </div>
        }
    }

    fn view_pokemon(&self, pokemon: &Pokemon) -> Html{
        html! {
            <div>
                {&pokemon.name}
            </div>
        }
    }
}

impl Component for Pokemons{
    type Properties = ();
    type Message = Msg;


    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetPokemon());
        Self {
            link,
            pokemons: None,
            fetch_pokemon: None,
        }
    }

    fn view(&self) -> Html {    
        yew::services::ConsoleService::info(&format!("{:?}", &self.pokemons));
        html! {
            <div>
                { self.render_pokemons(&self.pokemons)}
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message ) -> ShouldRender {
        match msg{
            Msg::GetPokemon() => {
                let req = Request::get("https://pokeapi.co/api/v2/pokemon?limit=10&offset=0")
                .body(Nothing)
                .expect("Can make req to poke api");

                let cb = self.link.callback(|response: Response<Json<Result<PokemonResult, anyhow::Error>>>| {
                    let Json(data) = response.into_body();
                    Msg::Resp(data)
                });

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_pokemon = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.pokemons = Some(data);
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}