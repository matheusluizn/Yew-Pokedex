use yew::prelude::*;
use log;

use crate::components::header::Header;

pub struct App{

}

impl Component for App{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self{
        Self {}
    }
    
    fn view(&self, _ctx: &Context<Self>) -> Html{
        log::info!("Mammamia");
        html!{
            <>
            <Header />
            </>
        }       
    }
}