use app::{MyApp, SiteName};
use bevy::prelude::*;
use async_bevy_web::prelude::ABWConfigPlugin;
use async_bevy_web::prelude::LeptosAppPlugin;

//this main implementation for the states needs to be cleaned up in main file 
mod states;
use states::StatePlugin;

fn main(){
    App::new()
        .insert_resource(SiteName("Bevy + Leptos".to_owned()))
        .add_systems(Startup, print_running)
        .add_plugins(ABWConfigPlugin::new(60.0))
        .add_plugins(LeptosAppPlugin::new(MyApp))
        .add_plugins(StatePlugin)
        .run();
}

fn print_running(){
    println!("Running!")
}
