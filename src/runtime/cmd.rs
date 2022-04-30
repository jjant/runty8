use std::marker::PhantomData;

use crate::DrawContext;

/// TODO:
///   - Make this a vector (to support Cmd::batch)
///   - Think more about `and_then` (if things don't return, it's not meaningful)
///     Why should things not return (i.e, return None), instead of just returning ()?
pub struct Cmd<T> {
    cmd: Box<dyn PureCmd<Output = T> + 'static>,
}

impl Cmd<u8> {
    pub fn get_flags(sprite: u8) -> Cmd<u8> {
        Cmd::new(GetFlags { sprite })
    }
}

impl Cmd<()> {
    pub fn toggle_flag(sprite: u8, flag: u8) -> Self {
        Cmd::new(ToggleFlag { sprite, flag })
    }
}

impl<T> PureCmd for Cmd<T> {
    type Output = T;

    fn run(&mut self, runty8: &mut DrawContext) -> Option<Self::Output> {
        self.cmd.run(runty8)
    }
}

impl<T: 'static> Cmd<T> {
    fn new(cmd: impl PureCmd<Output = T> + 'static) -> Self {
        Self { cmd: Box::new(cmd) }
    }

    pub fn map<R, F>(self, f: F) -> Cmd<R>
    where
        R: 'static,
        F: Fn(T) -> R + 'static,
    {
        Cmd::new(self.cmd.map(f))
    }

    pub fn and_then<R, F>(self, f: F) -> Cmd<R>
    where
        R: 'static,
        F: Fn(T) -> Cmd<R> + 'static,
    {
        Cmd::new(AndThen { cmd: self, then: f })
    }

    pub fn none() -> Self {
        Cmd::new(CmdNone::<T> { pd: PhantomData })
    }
}

impl<T> PureCmd for Box<dyn PureCmd<Output = T> + 'static> {
    type Output = T;

    fn run(&mut self, runty8: &mut DrawContext) -> Option<Self::Output> {
        (**self).run(runty8)
    }
}
pub trait PureCmd {
    type Output;

    fn map<R, F>(self, f: F) -> CmdMap<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> R,
    {
        CmdMap { cmd: self, f }
    }

    fn run(&mut self, runty8: &mut DrawContext) -> Option<Self::Output>;
}

pub struct CmdMap<C, F> {
    cmd: C,
    f: F,
}

impl<R, F: Fn(C::Output) -> R, C: PureCmd> PureCmd for CmdMap<C, F> {
    type Output = R;

    fn run(&mut self, runty8: &mut DrawContext) -> Option<R> {
        println!("[Cmd] CmdMap");
        let output = self.cmd.run(runty8)?;

        Some((self.f)(output))
    }
}

struct CmdNone<T> {
    pd: PhantomData<T>,
}

impl<T> PureCmd for CmdNone<T> {
    type Output = T;

    fn run(&mut self, _: &mut DrawContext) -> Option<Self::Output> {
        None
    }
}

struct SetFlag {
    sprite: u8,
    flag: u8,
    value: bool,
}

impl PureCmd for SetFlag {
    type Output = ();

    fn run(&mut self, runty8: &mut DrawContext) -> Option<Self::Output> {
        let SetFlag {
            sprite,
            flag,
            value,
        } = self;
        runty8.state.fset((*sprite).into(), (*flag).into(), *value);

        None
    }
}

struct AndThen<A, F> {
    cmd: A,
    then: F,
}

impl<A, F, B> PureCmd for AndThen<A, F>
where
    A: PureCmd,
    B: PureCmd,
    F: Fn(A::Output) -> B,
{
    type Output = B::Output;

    fn run(&mut self, runty8: &mut DrawContext) -> Option<Self::Output> {
        println!("[Cmd] AndThen");
        let a = self.cmd.run(runty8)?;

        (self.then)(a).run(runty8)
    }
}

struct GetFlags {
    sprite: u8,
}

impl PureCmd for GetFlags {
    type Output = u8;

    fn run(&mut self, runty8: &mut DrawContext) -> Option<Self::Output> {
        println!("[Cmd] GetFlags");
        let flags = runty8.fget(self.sprite.into());

        Some(flags)
    }
}

struct ToggleFlag {
    sprite: u8,
    flag: u8,
}

impl PureCmd for ToggleFlag {
    type Output = ();

    fn run(&mut self, runty8: &mut DrawContext) -> Option<Self::Output> {
        println!("[Cmd] ToggleFlag");
        let Self { sprite, flag } = *self;
        let flag_value = runty8.fget_n(sprite.into(), flag);
        runty8.state.fset(sprite.into(), flag.into(), !flag_value);

        Some(())
    }
}
