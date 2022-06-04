use yew::prelude::*;
use log;

use crate::components::header::Header;

pub struct App{

}

impl Component for App{
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self> ) -> Self{
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }
    
    fn view(&self) -> Html{
        log::info!("Mammamia");
        html!{
            <>
            <Header />
            </>
        }       
    }
}