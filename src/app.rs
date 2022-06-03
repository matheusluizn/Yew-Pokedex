use yew::prelude::*;
use log;

pub struct App{}

impl Component for App{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self{
        Self {}
    }
    
    fn view(&self, _ctx: &Context<Self>) -> Html{
        log::info!("Mammamia");
        html!{
            <h1>{"Poke Yew"}</h1>
        }       
    }
}