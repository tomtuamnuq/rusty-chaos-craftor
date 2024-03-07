pub const LABEL_BUTTON_RESET: &str = "Reset";
pub const TIP_BUTTON_RESET: &str = "Restore all app settings to their default values. Note that this will also affect any generated data, such as point series or benchmark results. The app saves the current configuration every 30 seconds.";

pub const LABEL_INIT_DATA: &str = "Init Data";
pub const TIP_INIT_DATA: &str = "Generate chaotic data with the selected distributions.";
pub const LABEL_REINIT_DATA: &str = "Reinit Data";
pub const TIP_REINIT_DATA: &str = "Data points with a feature higher than 32767 (or lower than -32768) are removed. This toggle generates new samples with the same initial distribution for data points that escaped the simulation.";
pub const LABEL_INIT_FUNCTION: &str = "Init Function";
pub const TIP_INIT_FUNCTION: &str = "Apply the parametrized chaotic function.";

pub const LABEL_NUM_PARAMS: &str = "Nr Params";
pub const TIP_NUM_PARAMS: &str = "Set the number of parameters. They are evenly spaced (Linspace).";

pub const LABEL_NUM_EXECS: &str = "Nr Executions";
pub const TIP_NUM_EXECS: &str = "Set the number of executions per frame. Defines how many times a discrete map is applied between two frames, and how many infinitesimal steps an ODE solver performs. Set to 1 and use the number of frames for visualizations.";
pub const LABEL_RUN: &str = "Run";
pub const TIP_RUN: &str = "Run or pause the execution of a chaotic function. Useful for immediately stopping a high CPU load to reconfigure.";
pub const LABEL_MAIN_MODE: &str = "Main Mode";
pub const TIP_MAIN_MODE: &str = "Show a plot or run a benchmark of chaotic data.";
pub const LABEL_INIT_MODE: &str = "Mode";
pub const TIP_INIT_MODE: &str = "Choose the general type of chaotic data to initialize. Chaotic functions are selectable based on this selection.";
pub const LABEL_NUM_SAMPLES: &str = "Nr Samples";
pub const TIP_NUM_SAMPLES: &str = "Set the number of samples. This defines how many times a distribution is sampled from. For meshes, it is the number per axis (the total number of samples is the number of chosen samples to the power of selected meshes).";
pub const TIP_BUTTON_DECREASE_NUM_STATES: &str = "Decrease the dimensionality of the data points.";
pub const TIP_BUTTON_INCREASE_NUM_STATES: &str = "Increase the dimensionality of the data points.";
pub const TIP_INIT_PANEL: &str =
    "Configure initial chaotic data. Then choose which function to apply.";
pub const TIP_DISTRIBUTIONS_ALL: &str = "Select the general class of initial distributions to choose from. Mesh (deterministic as well) creates a grid with (#samples)^(#meshes) points in total.";
pub const TIP_DISTRIBUTIONS_NO_MESH: &str =
    "Select between probabilistic and deterministic initial distributions.";
pub const TIP_DIMS: &str = "Select the dimensionality of the data points. Affects which discrete maps or systems of differential equations are selectable.";
pub const TIP_PARTICLE_MODE: &str = "Choose between a 2D or 3D particle simulation.";
pub const TIP_FRACTAL_MODE: &str =
    "Choose between 2D rings (Complex, Dual, Perplex) or 4D Quaternions.";

pub const LABEL_PLOT_BACKEND: &str = "Plot Backend";
pub const TIP_PLOT_BACKEND: &str = "Select the plotting backend to use. Egui has a 2D backend for plots. 3D plots are available through Plotters and the egui_plotters crate.";

pub const LABEL_NUM_FRAMES: &str = "Frame Rate";
pub const TIP_NUM_FRAMES: &str = "Set the number of frames per second.";
pub const LABEL_NUM_SERIES: &str = "";
pub const TIP_NUM_SERIES: &str = "Set the number of series (trajectory length) to display.";
pub const LABEL_TRAJECTORY: &str = "Trajectory";
pub const TIP_TRAJECTORY: &str = "Display a trajectory with a set length. This defines how points are colored if the point coloring mode is series-based. The size of points for states is calculated dynamically to highlight the latest series."; // TODO only Plot2D
pub const LABEL_COLORMAP: &str = "Color Map";
pub const TIP_COLORMAP: &str = "Select the color map that creates colors for the plot points.";
pub const LABEL_COLOR_PER_POINT: &str = "Coloring Mode";
pub const TIP_COLOR_PER_POINT: &str = "How to color the points: \n- A single color. \n- A color per series to follow distribution evolution. \n- A color per point to follow its trajectory. \n- Mapping of a feature to a color space (min => 0, max => 1).";
pub const LABEL_POINT_SIZE: &str = "Point Size";
pub const TIP_POINT_SIZE: &str = "Set a fixed size for shapes such as points.";

// Plot
pub const LABEL_PARAMS_SHOWN: &str = "A parameter range is always on the X-Axis.";
pub const LABEL_PARAMS_NOT_SHOWN: &str =
    "The parameter range can be visualized over the X-Axis projection or color.";
pub const LABEL_PLOT_STATE_N: &str =
    "A multidimensional state is visualized by the selected feature projections.";
pub const LABEL_PLOT_PAR_PARTICLE: &str = "Particles are shown with the same radius and markers for each parameter value. Useful to explore the evolution of the particle world under different settings, e.g., the long-range influence of l (gravitational constants).";
// 2D
pub const LABEL_PLOT2D_STATE_1: &str = "A single state S is plotted as S prev (X-Axis=S') against S new (Y-Axis=S). Useful for discovering fixpoints and circles in a trajectory.";
pub const LABEL_PLOT2D_PARTICLE: &str = "Particles are circles with a radius of √mass, containing markers that represent the charge and parity. Charge is visualized by triangles pointing up or down. Marker size is given by radius and not the actual value of the charge or parity.";
pub const LABEL_PLOT2D_FRACTAL: &str = "Fractals should be plotted with color representing the number of iterations. There should be a sufficiently large number of points to create vibrant fractals. The final set (reached at maximum number of iterations) is highlighted in black for the colorful maps and red for the greyish ones.";
pub const LABEL_PLOT2D_PAR_STATE_1: &str =
    "The one-dimensional state is projected on the Y-Axis to produce a bifurcation diagram.";
pub const LABEL_PLOT2D_PAR_STATE_N: &str = "The state is visualized by the selected feature projection on the Y-Axis to produce a bifurcation diagram.";
pub const LABEL_PLOT2D_PAR_FRACTAL: &str = "The color should represent the number of iterations. Interesting parameter values may be identified by colorful fractal projections. A 3D plot is better suited.";
// 3D
pub const LABEL_PLOT3D_STATE_1: &str = "A one-dimensional state S is plotted as S prev prev (X-Axis=S'') against S prev (Y-Axis=S') and S new (Z-Axis). Useful for discovering fixpoints or circles in the trajectory.";
pub const LABEL_PLOT3D_STATE_2: &str = "A two-dimensional state S is plotted by a fixed assignment of S1 to Y and S2 to Z. The X-Axis shows time t for a specified trajectory length.";
pub const LABEL_PLOT3D_PARTICLE: &str = "Particles are visualized by markers representing charge and parity. 'P' and 'N' mark positively or negatively charged ones. A cross marks anti-parity, meaning it will perform an inelastic collision with particles of a different parity (without a cross). Marker size can be modified by pointer-size for better visibility.";
pub const LABEL_PLOT3D_FRACTAL: &str = "Fractals should be plotted with color representing the number of iterations. There should be a sufficiently large number of points to create vibrant fractals. The final set (reached at maximum number of iterations) is highlighted in black for the colorful maps and red for the greyish ones. Small point sizes increase performance. Opacity and the show set option are useful to avoid overlapping. Setting the iteration count as Z-Axis allows to follow how points are marked as not being part of the set.";
pub const LABEL_PLOT3D_PAR_STATE_1: &str = "The one-dimensional state is projected on the Z-Axis to produce a bifurcation diagram. The Y-Axis shows time t for a specified trajectory length. Allows investigation of the evolution of the state with different parameter values and a final 2D bifurcation diagram.";
pub const LABEL_PLOT3D_PAR_STATE_N: &str = "The state is visualized by the selected feature projections on the Y- and Z-Axis. This may produce a multidimensional bifurcation diagram and provides insights into interesting regions of the parameter space.";

pub const LABEL_PLOT3D_PAR_FRACTAL: &str = "The color should represent the number of iterations. Interesting regions of the parameter space can be identified by a 2D fractal for each parameter value.";
pub const LABEL_POINT_OPACITY: &str = "Point Opacity";
pub const TIP_POINT_OPACITY: &str = "Adjust the opacity for coloring shapes like points, useful for visualizing overlapping shapes.";

// particles
pub const TIP_PARTICLE_RADIUS: &str = "Particle radius is √mass. Egui displays a circle with this radius, while Plotters shows a rectangle around the circle's origin with the correct radius but displays shapes with the chosen size.";
pub const TIP_PARTICLE_PARITY: &str = "Parity determines collision behavior. Same parity causes inelastic collisions, combining features. Different parity causes elastic collisions, exchanging momentum. The system's collision factor 's' influences the impulse of elastic collisions.";
pub const TIP_PARTICLE_CHARGE: &str = "Charge determines attraction (opposite charges) or repulsion (same charges) due to mid-range forces.";
pub const TIP_PARTICLE_MASS: &str =
    "Mass influences gravitational force over long ranges and defines the particle's radius.";
pub const TIP_PARTICLE_PX: &str = "Particle's position in the x-direction.";
pub const TIP_PARTICLE_PY: &str = "Particle's position in the y-direction.";
pub const TIP_PARTICLE_PZ: &str = "Particle's position in the z-direction.";
pub const TIP_PARTICLE_VX: &str = "Particle's velocity in the x-direction.";
pub const TIP_PARTICLE_VY: &str = "Particle's velocity in the y-direction.";
pub const TIP_PARTICLE_VZ: &str = "Particle's velocity in the z-direction.";

// fractal
pub const TIP_FRACTAL_SET: &str =
    "Show the points that are in the set. Useful to avoid overlapping.";
pub const LINK_COMPLEX: &str = "https://wikipedia.org/wiki/Fractal";
pub const TIP_COMPLEX: &str = "Complex numbers, solutions to equations like x²=-1, are used to generate fractals such as the Mandelbrot and Julia sets.";
pub const LABEL_BASIS_COMPLEX: &str = "Complex Number: a + b i";
pub const TIP_FRACTAL_COMPLEX_RE: &str = "Real part 'a' of the complex number c = a + b i, determining the x-value of the pixel and the real part of z0.";
pub const TIP_FRACTAL_COMPLEX_IM: &str = "Imaginary part 'b' of the complex number c = a + b i, determining the y-value of the pixel and the imaginary part of z0.";

pub const LINK_DUAL: &str = "http://dx.doi.org/10.1080/10236190412331334482";
pub const TIP_DUAL: &str = "Dual numbers, also known as parabolic numbers, are akin to complex numbers but use the element ε (ε≠0, ε²=0) instead of the imaginary unit i. They're applied in automatic differentiation and geometrical transformations. For more on quadratic dynamics in binary systems, see 'Quadratic dynamics in binary number systems' by Fishback. Dual numbers are also used with other algebraic structures to create high-dimensional fractal structures, as seen in 'Dual-Quaternion Julia Fractals' by Ben Kenwright.";
pub const LABEL_BASIS_DUAL: &str = "Dual Number: a + b ε";
pub const TIP_FRACTAL_DUAL_RE: &str = "The real part 'a' of the dual number c = a + b ε, defining the x-value of the pixel and the real part of z0.";
pub const TIP_FRACTAL_DUAL_IM: &str = "The dual part 'b' of the dual number c = a + b ε, defining the y-value of the pixel and the dual part of z0.";

pub const LINK_PERPLEX: &str = "https://doi.org/10.3390/fractalfract3010006";
pub const TIP_PERPLEX: &str = "Perplex numbers, also known as split-complex, double, or hyperbolic numbers, extend real numbers by introducing a new element h (h≠±1, h²=1). Julia and Mandelbrot sets over hyperbolic numbers are simpler than those over complex numbers. For details, see 'Julia and Mandelbrot Sets for Dynamics over the Hyperbolic Numbers'.";
pub const LABEL_BASIS_PERPLEX: &str = "Perplex Number: t + x h";
pub const TIP_FRACTAL_PERPLEX_RE: &str = "The time component 't' of the perplex number c = t + x h, defining the x-value of the pixel and the time component of z0.";
pub const TIP_FRACTAL_PERPLEX_IM: &str = "The space component 'x' of the perplex number c = t + x h, defining the y-value of the pixel and the space component of z0.";

pub const LINK_QUATERNION: &str = "https://doi.org/10.1007/s11071-023-08785-0";
pub const TIP_QUATERNION: &str = "Quaternions extend complex numbers with two additional units j and k, alongside the imaginary unit i, to generate 4D fractal structures. Mandelbrot and Julia sets are visualized by selecting a 3D subspace, revealing intricate structures. See 'On the quaternion Julia sets via Picard–Mann iteration' for more information.";
pub const LABEL_BASIS_QUATERNION: &str = "Quaternion: a + b i + c j + d k";
pub const TIP_FRACTAL_QUATERNION_RE: &str = "The real part 'a' of the quaternion z = a + b i + c j + d k, defining the x-value of the pixel and the real part of z0.";
pub const TIP_FRACTAL_QUATERNION_I: &str = "The i-component 'b' of the quaternion z = a + b i + c j + d k, defining the y-value of the pixel and the i-component of z0.";
pub const TIP_FRACTAL_QUATERNION_J: &str = "The j-component 'c' of the quaternion z = a + b i + c j + d k, defining the z-value of the pixel and the j-component of z0.";
pub const TIP_FRACTAL_QUATERNION_K: &str = "The k-component 'd' of the quaternion z = a + b i + c j + d k, usually fixed to visualize the dimensions a, b, and c.";
// benchmark
pub const LABEL_WARMUP: &str = "Warm-Up";
pub const TIP_WARMUP: &str = "Choose whether to run the chaotic function several times before measurement for benchmarking purposes.";
pub const LABEL_NUM_WARMUPS: &str = "Number of Warm-Ups";
pub const TIP_NUM_WARMUPS: &str =
    "Specify the number of warm-up executions before starting the actual benchmark.";
pub const LABEL_NUM_ITERATIONS: &str = "Number of Iterations";
pub const TIP_NUM_ITERATIONS: &str = "Determine the number of iterations for each benchmark run.";

pub const LABEL_BENCHMARK: &str = "Benchmark Execution";
pub const TIP_BENCHMARK: &str = "Execute the benchmark with the current settings.";
