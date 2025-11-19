pub use crate::prelude::*;
use bevy::ecs::{
    lifecycle::HookContext,
    query::{QueryData, ReadOnlyQueryData, ReleaseStateQueryData},
    world::DeferredWorld,
};
use std::marker::PhantomData;

#[derive(Component)]
pub struct Other(u32);

pub struct SomethingSpecial;

pub fn start(mut commands: Commands) {
    let entity = commands.spawn((Transform::default(), Other(1))).id();
    commands.spawn(EntityWith::<Transform, Other>::new(entity));

    let entity = commands.spawn((Transform::default(), Other(2))).id();
    commands.spawn(<EntityWith<Other> as Marked>::MarkedBy::<SomethingSpecial>::new(entity));
}

pub fn update(
    entity_with_transform_and_other: Query<&EntityWith<Transform, Other>>,
    // entity_with_other_and_marked_by_something_special: Query<
    //     &<EntityWith<Other> as Marked>::MarkedBy<SomethingSpecial>,
    // >,
    transform_and_other: Query<(&Transform, &Other)>,
    //other: Query<&Other>,
) {
    for entity_with in entity_with_transform_and_other {
        let (transform, other) = entity_with.get(&transform_and_other);
        info!("{}", other.0);
        info!("{}", transform.translation);
    }

    // for entity_with in entity_with_other_and_marked_by_something_special {
    //     let other = entity_with.get(&other);
    //     assert_eq!(other.0, 2);
    // }
}

pub trait Blah {
    type ReadOnlyQueryData: ReadOnlyQueryData + ReleaseStateQueryData;
    type Bundle: Bundle;
}

impl Blah for Holder {
    type ReadOnlyQueryData = ();
    type Bundle = ();
}
impl<A: Component> Blah for Holder<A> {
    type ReadOnlyQueryData = &'static A;
    type Bundle = A;
}
impl<A: Component, B: Component> Blah for Holder<A, B> {
    type ReadOnlyQueryData = (&'static A, &'static B);
    type Bundle = (A, B);
}

pub trait AnyOf<Case, A, B, C, D> {}
impl<A, B, C, D> AnyOf<(), A, B, C, D> for A {}
impl<A, B, C, D> AnyOf<i32, A, B, C, D> for B {}
impl<A, B, C, D> AnyOf<u32, A, B, C, D> for C {}
impl<A, B, C, D> AnyOf<bool, A, B, C, D> for D {}

pub trait OldValidQueryFor<Bundle, Case> {}

impl<A: Component> OldValidQueryFor<Holder<A>, ()> for () {}
impl<A: Component> OldValidQueryFor<Holder<A>, ()> for &A {}
impl<A: Component> OldValidQueryFor<Holder<A>, ()> for &mut A {}

//impl<H1: Component, H2: Component, M1, M2, T1: AnyOf<
impl<A: Component, B: Component> OldValidQueryFor<Holder<A, B>, ()> for () {}
impl<A: Component, B: Component> OldValidQueryFor<Holder<A, B>, ()> for &A {}
impl<A: Component, B: Component> OldValidQueryFor<Holder<A, B>, bool> for &B {}
impl<A: Component, B: Component> OldValidQueryFor<Holder<A, B>, ()> for (&A, &B) {}
impl<A: Component, B: Component> OldValidQueryFor<Holder<A, B>, bool> for (&B, &A) {}
impl<A: Component, B: Component> OldValidQueryFor<Holder<A, B>, ()> for (&mut A, &mut B) {}

pub struct Unit;
pub struct Holder<A = Unit, B = Unit>(PhantomData<(A, B)>);
pub type EntityWith<A = Unit, B = Unit> = MarkedEntityWith<Holder<A, B>, ()>;

#[derive(Component)]
#[component(on_add = Self::on_add)]
pub struct MarkedEntityWith<Bundle: Blah + 'static, Marker: 'static> {
    target: Entity,
    //observer: Entity,
    phantom: PhantomData<(Bundle, Marker)>,
}

unsafe impl<C: Blah, M> Send for MarkedEntityWith<C, M> {}
unsafe impl<C: Blah, M> Sync for MarkedEntityWith<C, M> {}

impl<Bundle: Blah, Marker> MarkedEntityWith<Bundle, Marker> {
    fn new(target: Entity) -> Self {
        Self {
            target,
            phantom: PhantomData,
        }
    }

    fn get<'a, Case, D: QueryData + ValidQueryFor<Bundle, Case>>(
        &self,
        query: &'a Query<'a, 'a, D>,
    ) -> <<D as QueryData>::ReadOnly as QueryData>::Item<'a, 'a> {
        query.get(self.target).unwrap()
    }

    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let Some(entity_with) = world.entity(context.entity).get::<Self>() else {
            error!("Impossible! The component was just added!");
            return;
        };

        let target = entity_with.target;
        let this_entity = context.entity;

        // Despawn immediately if the target doesn't have the component.
        if world
            .entity(target)
            .get_components::<Bundle::ReadOnlyQueryData>()
            .is_none()
        {
            world.commands().entity(this_entity).remove::<Self>();
            return;
        }

        // Observe when the component is removed from the target so that this component
        // can remove itself.
        world.commands().entity(target).observe(
            move |_: On<Remove, Bundle::Bundle>, mut commands: Commands| {
                commands.entity(this_entity).remove::<Self>();
            },
        );
    }
}

pub trait Marked {
    type MarkedBy<Marker: 'static>;
}
impl<Bundle: Blah> Marked for MarkedEntityWith<Bundle, ()> {
    type MarkedBy<Marker: 'static> = MarkedEntityWith<Bundle, Marker>;
}

pub struct Tuple<A = Unit, B = Unit, C = Unit, D = Unit>(PhantomData<(A, B, C, D)>);

pub trait ContainsOnly<Marker, A = Unit, B = Unit, C = Unit, D = Unit> {}
impl<
    M1,
    M2,
    M3,
    M4,
    CO1,
    CO2,
    CO3,
    CO4,
    T1: OneOf<M1, CO1, CO2, CO3, CO4>,
    T2: OneOf<M2, CO1, CO2, CO3, CO4>,
    T3: OneOf<M3, CO1, CO2, CO3, CO4>,
    T4: OneOf<M4, CO1, CO2, CO3, CO4>,
> ContainsOnly<(M1, M2, M3, M4), CO1, CO2, CO3, CO4> for Tuple<T1, T2, T3, T4>
{
}

// New stuff:
pub trait OneOf<Marker, A = Unit, B = Unit, C = Unit, D = Unit> {}

impl<A, B, C, D> OneOf<i32, A, B, C, D> for A {}
impl<A, B, C, D> OneOf<(), A, B, C, D> for B {}
impl<A, B, C, D> OneOf<bool, A, B, C, D> for C {}
impl<A, B, C, D> OneOf<f32, A, B, C, D> for D {}

pub trait ValidQueryFor<Bundle, Marker> {}

impl<H1, H2, M1, M2, A: OneOf<M1, H1, H2>, B: OneOf<M2, H1, H2>>
    ValidQueryFor<Holder<H1, H2>, (M1, M2)> for (&A, &B)
{
}
