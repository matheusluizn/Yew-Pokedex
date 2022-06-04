use yew::prelude::*;

pub struct Header{

}

impl Component for Header{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self{
        Self {}
    }
    
    fn view(&self, _ctx: &Context<Self>) -> Html{
        log::info!("Mammamia");
        html!{
            <header>
                <img src={"./assets/pokemon-logo.png"} />
            </header>
        }       
    }
}