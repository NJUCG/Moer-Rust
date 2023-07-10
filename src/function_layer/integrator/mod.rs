pub mod integrator;
mod direct_integrator;
mod normal_integrator;
mod whitted_integrator;
mod path_integrator;
mod volpath;

pub use integrator::Integrator;
