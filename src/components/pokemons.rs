use yew::prelude::*;

pub enum Msg {
    GetPokemons,
}

struct Pokemons {
    pokemons: Option<Vec<Pokemon>>,
}

pub struct Pokemon {
    pub name: String,
    pub url: String,
}

impl Component for Pokemons {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { pokemons: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetPokemons => {}
        }
        true
    }
}
