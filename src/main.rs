mod app;
mod components;

fn main(){
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
}