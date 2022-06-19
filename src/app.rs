use yew::prelude::*;

use crate::components::header::Header;
use crate::components::pokemons::Pokemons;

pub struct App{

}

impl Component for App{
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self> ) -> Self{
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
    
    fn view(&self) -> Html{
        html!{
            <main>
            <Header />
            <Pokemons />
            </main>
        }       
    }
}