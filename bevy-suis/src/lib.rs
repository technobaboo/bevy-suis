use bevy::{
    app::{Plugin, PreUpdate},
    ecs::{
        component::Component,
        entity::{self, EntityHashMap},
        system::Query,
    },
    prelude::{App, Entity, IntoSystemConfigs},
};
use std::{hash::Hash, marker::PhantomData};
pub mod xr;

pub struct SUISPlugin<T: InputDataTrait>(PhantomData<T>);
impl<T: InputDataTrait> Plugin for SUISPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (clear_handler_old_input::<T>, setup_input::<T>).chain(),
        );
    }
}

pub trait InputDataTrait: Clone + Eq + PartialEq + Send + Sync + 'static {}
impl<T: Clone + Eq + PartialEq + Send + Sync + 'static> InputDataTrait for T {}

#[derive(Debug, Hash)]
pub struct InputData<T: InputDataTrait>(T);

#[derive(Component, Debug)]
pub struct InputMethod<T: InputDataTrait> {
    pub input_data: T,
    pub handler_order: Vec<Entity>,
    pub captured_by: Option<Entity>,
}
#[derive(Component, Debug)]
pub struct InputHandler<T: InputDataTrait> {
    pub previous_frame_data: EntityHashMap<T>,
    pub current_data: EntityHashMap<T>,
    pub capture_condition: fn(&InputData<T>) -> bool,
}

fn clear_handler_old_input<T: InputDataTrait>(mut handlers: Query<&mut InputHandler<T>>) {
    for mut handler in handlers.iter_mut() {
        handler.previous_frame_data = std::mem::take(&mut handler.current_data);
    }
}

fn setup_input<T: InputDataTrait>(
    methods: Query<&InputMethod<T>>,
    mut handlers: Query<&mut InputHandler<T>>,
) {
    for method in methods.iter() {
        for handler_entity in method.handler_order.iter().copied() {
            let Ok(mut handler) = handlers.get_mut(handler_entity) else {
                continue;
            };
            handler
                .current_data
                .insert(handler_entity, method.input_data.clone());
        }
    }
}
