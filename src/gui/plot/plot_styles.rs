use crate::chaos::data::*;
use crate::chaos::fractal::FractalData;
use crate::chaos::Particle;

pub const DEFAULT_RADIUS: ChaosFloat = 2.0;
#[derive(Debug, PartialEq, Default)]
pub struct Markers {
    pub positive: bool,
    pub negative: bool,
    pub special: bool,
}

pub trait ColoredStyle<C> {
    fn colored_style(&self, color: C) -> Style<C> {
        Style {
            color,
            radius: DEFAULT_RADIUS,
            markers: Default::default(),
        }
    }
}
impl<C> ColoredStyle<C> for State1 {}
impl<C> ColoredStyle<C> for State2 {}
impl<C> ColoredStyle<C> for State3 {}
impl<C> ColoredStyle<C> for State4 {}
impl<C> ColoredStyle<C> for State5 {}
impl<C> ColoredStyle<C> for State6 {}
#[derive(Debug, PartialEq)]
pub struct Style<C> {
    pub color: C,
    pub radius: ChaosFloat,
    pub markers: Markers,
}
impl<V, F, C> ColoredStyle<C> for Particle<V, F> {
    fn colored_style(&self, color: C) -> Style<C> {
        let positive = self.mid > 0.0;
        let negative = self.mid < 0.0;
        let special = self.short < 0.0;
        let radius = self.radius;
        Style {
            color,
            radius,
            markers: Markers {
                positive,
                negative,
                special,
            },
        }
    }
}
impl<E, C> ColoredStyle<C> for FractalData<E> {
    fn colored_style(&self, color: C) -> Style<C> {
        Style {
            color,
            radius: DEFAULT_RADIUS,
            markers: Markers {
                positive: self.last(),
                negative: self.first(),
                special: self.biomorph(),
            },
        }
    }
}
