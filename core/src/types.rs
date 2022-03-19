use bevy::ecs::schedule::SystemDescriptor;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use std::any::Any;

pub trait ComponentProps: 'static + Clone + Send + Sync {
    fn typed<T>(self) -> T
    where
        T: ComponentProps,
    {
        let untyped: Box<dyn Any> = Box::new(self);
        let typed = match untyped.downcast_ref::<T>() {
            Some(p) => p,
            None => panic!("Failed to downcast ComponentProps"),
        };
        typed.clone()
    }
}

pub trait EntitySpawner {
    fn spawn<'w, 's, 'a>(
        commands: &'a mut Commands<'w, 's>,
        props: impl ComponentProps,
    ) -> EntityCommands<'w, 's, 'a>;
}

pub trait MainSystemDescriptorFactory {
    fn system_descriptor(&self) -> SystemDescriptor;
}
