#![allow(missing_docs)] // for downcast.

use downcast_rs::Downcast;
use generational_arena::Arena;
use crate::RealField;

use crate::object::{BodyHandle, BodySet, DefaultBodyHandle};
use crate::solver::IntegrationParameters;

/// Default force generator set based on an arena with generational indices.
pub type DefaultForceGeneratorSet<N: RealField, Handle: BodyHandle = DefaultBodyHandle> =
    Arena<Box<dyn ForceGenerator<N, Handle>>>;

/// Trait implemented by sets of force generators.
///
/// A set of bodies maps a force generator handle to a force generator instance.
pub trait ForceGeneratorSet<N: RealField, Handle: BodyHandle> {
    /// Type of a force generator stored in this set.
    type ForceGenerator: ?Sized + ForceGenerator<N, Handle>;
    /// Type of a force generator handle identifying a force generator in this set.
    type Handle: Copy;

    /// Gets a reference to the force generator identified by `handle`.
    fn get(&self, handle: Self::Handle) -> Option<&Self::ForceGenerator>;
    /// Gets a mutable reference to the force generator identified by `handle`.
    fn get_mut(&mut self, handle: Self::Handle) -> Option<&mut Self::ForceGenerator>;

    /// Check if this set contains a force generator identified by `handle`.
    fn contains(&self, handle: Self::Handle) -> bool;

    /// Iterate through all the force generators on this set, applying the closure `f` on them.
    fn foreach(&self, f: impl FnMut(Self::Handle, &Self::ForceGenerator));
    /// Mutable iterates through all the force generators on this set, applying the closure `f` on them.
    fn foreach_mut(&mut self, f: impl FnMut(Self::Handle, &mut Self::ForceGenerator));
}

impl<N: RealField, Handle: BodyHandle> ForceGeneratorSet<N, Handle>
    for DefaultForceGeneratorSet<N, Handle>
{
    type ForceGenerator = dyn ForceGenerator<N, Handle>;
    type Handle = DefaultForceGeneratorHandle;

    fn get(&self, handle: Self::Handle) -> Option<&Self::ForceGenerator> {
        self.get(handle).map(|c| &**c)
    }

    fn get_mut(&mut self, handle: Self::Handle) -> Option<&mut Self::ForceGenerator> {
        self.get_mut(handle).map(|c| &mut **c)
    }

    fn contains(&self, handle: Self::Handle) -> bool {
        self.contains(handle)
    }

    fn foreach(&self, mut f: impl FnMut(Self::Handle, &Self::ForceGenerator)) {
        for (h, b) in self.iter() {
            f(h, &**b)
        }
    }

    fn foreach_mut(&mut self, mut f: impl FnMut(Self::Handle, &mut Self::ForceGenerator)) {
        for (h, b) in self.iter_mut() {
            f(h, &mut **b)
        }
    }
}

/// The handle of a force generator.
pub type DefaultForceGeneratorHandle = generational_arena::Index;

/// A persistent force generator.
///
/// A force generator applies a force to one or several bodies at each step of the simulation.
pub trait ForceGenerator<N: RealField, Handle: BodyHandle>: Downcast + Send + Sync {
    /// Apply forces to some bodies.
    fn apply(
        &mut self,
        parameters: &IntegrationParameters<N>,
        bodies: &mut dyn BodySet<N, Handle = Handle>,
    );
}

impl_downcast!(ForceGenerator<N, Handle> where N: RealField, Handle: BodyHandle);
