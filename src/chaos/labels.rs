use crate::chaos::data::*;

pub trait ChaosDescription {
    fn description(&self) -> String;
    fn reference(&self) -> &'static str;
}

pub trait ChaosFormula {
    fn formula(&self) -> &[&'static str];
}

pub const LABEL_PARTICLE_PX: &str = "Position X";
pub const LABEL_PARTICLE_PY: &str = "Position Y";
pub const LABEL_PARTICLE_PZ: &str = "Position Z";
pub const LABEL_PARTICLE_VX: &str = "Velocity X";
pub const LABEL_PARTICLE_VY: &str = "Velocity Y";
pub const LABEL_PARTICLE_VZ: &str = "Velocity Z";
pub const LABEL_PARTICLE_PARITY: &str = "Parity s";
pub const LABEL_PARTICLE_CHARGE: &str = "Charge m";
pub const LABEL_PARTICLE_MASS: &str = "Mass l";
pub const LABEL_PARTICLE_RADIUS: &str = "Radius r";
pub const LABEL_PARTICLE_DESCRIPTION: &str = "This particle simulation employs Newtonian mechanics to model the dynamics of particles. It calculates the cumulative forces between pairs to determine their velocity and positional changes. When particles collide, indicated by their radii overlapping, a short-range force is applied. The nature of the collision—inelastic or elastic—depends on the parity of the particles' signs. The 's' parameter inversely controls collision behavior and the effective mass during the impulse exchange of an elastic collision. Setting 's' to zero turns off collision interactions. The simulation also includes a mid-range force analogous to the Coulomb force, governed by the 'm' parameter. Positive 'm' values result in attraction among particles with opposite charges, while negative values cause repulsion. With 'm' set to zero, this force is neutralized. Lastly, a long-range gravitational force, modulated by the 'l' parameter, affects the particles. Negative 'l' values can be used to simulate anti-gravity effects.";
pub const LINK_PARTICLE: &str = "https://wikipedia.org/wiki/Classical_mechanics";
pub const FORMULA_PARTICLE: [&str; 5] = [
    "radius= √mass",
    "force_i= ∑_j short(i,j) + mid(i,j) + long(i,j)",
    "acceleration = force / mass",
    "d position = velocity",
    "d velocity = acceleration",
];
pub const LABELS_PARTICLE_2D: [&str; NUM_DIMS_PARTICLEXY] = [
    LABEL_PARTICLE_PX,
    LABEL_PARTICLE_PY,
    LABEL_PARTICLE_VX,
    LABEL_PARTICLE_VY,
    LABEL_PARTICLE_PARITY,
    LABEL_PARTICLE_CHARGE,
    LABEL_PARTICLE_MASS,
    LABEL_PARTICLE_RADIUS,
];
pub const LABELS_PARTICLE_3D: [&str; NUM_DIMS_PARTICLEXYZ] = [
    LABEL_PARTICLE_PX,
    LABEL_PARTICLE_PY,
    LABEL_PARTICLE_PZ,
    LABEL_PARTICLE_VX,
    LABEL_PARTICLE_VY,
    LABEL_PARTICLE_VZ,
    LABEL_PARTICLE_PARITY,
    LABEL_PARTICLE_CHARGE,
    LABEL_PARTICLE_MASS,
    LABEL_PARTICLE_RADIUS,
];
pub const LABELS_SHORT_PARTICLE_2D: [&str; NUM_DIMS_PARTICLEXY] =
    ["X", "Y", "vx", "vy", "s", "m", "l", "r"];
pub const LABELS_SHORT_PARTICLE_3D: [&str; NUM_DIMS_PARTICLEXYZ] =
    ["X", "Y", "Z", "vx", "vy", "vz", "s", "m", "l", "r"];

pub const LABELS_COMPLEX: [&str; NUM_DIMS_FRACTALCOMPLEX] = [
    "c real",
    "c imaginary",
    "fractal iteration",
    "z real",
    "z imaginary",
];
pub const LABELS_DUAL: [&str; NUM_DIMS_FRACTALDUAL] =
    ["c real", "c ε", "fractal iteration", "z real", "z ε"];
pub const LABELS_PERPLEX: [&str; NUM_DIMS_FRACTALPERPLEX] = [
    "c time",
    "c space",
    "fractal iteration",
    "z time",
    "z space",
];
pub const LABELS_QUATERNION: [&str; NUM_DIMS_FRACTALQUATERNION] = [
    "c w",
    "c i",
    "fractal iteration",
    "c j",
    "c k",
    "z w",
    "z i",
    "z j",
    "z k",
];
pub const LABELS_SHORT_COMPLEX: [&str; NUM_DIMS_FRACTALCOMPLEX] =
    ["c re", "c im", "iter", "z re", "z im"];
pub const LABELS_SHORT_DUAL: [&str; NUM_DIMS_FRACTALDUAL] = ["c re", "c ε", "iter", "z re", "z ε"];
pub const LABELS_SHORT_PERPLEX: [&str; NUM_DIMS_FRACTALPERPLEX] =
    ["c t", "c x", "iter", "z t", "z x"];
pub const LABELS_SHORT_QUATERNION: [&str; NUM_DIMS_FRACTALQUATERNION] = [
    "c w", "c i", "iter", "c j", "c k", "z w", "z i", "z j", "z k",
];
