use serde_derive::Deserialize;
use serde_json::json;
use yew::format::{Json};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};



#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct PokemonResult{
    data : PokemonV2Pokemon      
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
struct PokemonV2Pokemon{
    pokemon_v2_pokemon: Option<Vec<Pokemon>>
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
struct Pokemon {
    id: i32,
    name: String,
    pokemon_v2_pokemontypes: Vec<PokemonTypes>
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
struct PokemonTypes {
    pokemon_v2_type : PokemonTypeNames
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
struct PokemonTypeNames{
    name: String
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
                let pokemons = &p.data.pokemon_v2_pokemon;
                html! {
                    <div>
                        { pokemons.iter().map(|pokemons| self.view_pokemons(pokemons)).collect::<Html>() }
                    </div>
                }
            }
            None => {
                html! {
                    <div class={"loading"}>
                        <img src={String::from("./assets/pokeball-icon.png")}/>
                        <h1>{"Carregando Pokemons..."}</h1>
                    </div>
                }
            }
        }
    }

    fn view_pokemons(&self, pokemons : &Vec<Pokemon>) -> Html{
        html! {
            <div class={"pokemon"}>
                { pokemons.iter().map(|pokemon| self.view_pokemon(pokemon)).collect::<Html>() }
            </div>
        }
    }

    fn view_pokemon(&self, pokemon: &Pokemon) -> Html{

        let pokemon_name = &pokemon.name.split("-").collect::<Vec<&str>>()[0];
        yew::services::ConsoleService::info("Mammamia");

        html! {
            <div class={"pokemon-card"}>
                <object data={String::from(&format!("https://img.pokemondb.net/sprites/black-white/anim/normal/{}.gif", &pokemon.name))} loading={"lazy"} type="image/png">
                    <img src={String::from(&format!("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png", &pokemon.id))} />
                </object>
                
                <h4>{"Nº "} {&pokemon.id}</h4>
                <h3>
                    {pokemon_name}
                </h3>
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
        html! {
            <div>
                { self.render_pokemons(&self.pokemons)}
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message ) -> ShouldRender {
        match msg{
            Msg::GetPokemon() => {

                let data = &json!({
                    "operationName":"samplePokeAPIquery",
                    "query":"query samplePokeAPIquery {\n  pokemon_v2_pokemon {\n    id\n    name\n    pokemon_v2_pokemontypes {\n      pokemon_v2_type {\n        name\n      }\n    }\n  }\n}\n"
                });

                let req = Request::post("https://beta.pokeapi.co/graphql/v1beta")
                .body(Json(data))
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