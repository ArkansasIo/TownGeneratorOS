use serde::{Deserialize, Serialize};

pub const G: f64 = 6.67430e-11;
pub const AU_METERS: f64 = 149_597_870_700.0;
pub const SOLAR_MASS_KG: f64 = 1.98847e30;
pub const EARTH_MASS_KG: f64 = 5.9722e24;

#[derive(Debug, Clone, Deserialize)]
pub struct OrbitRequest {
    pub central_mass_solar: f64,
    pub semi_major_axis_au: f64,
    #[serde(default)]
    pub eccentricity: f64,
}
#[derive(Debug, Serialize)]
pub struct OrbitResult {
    pub gravitational_parameter_m3_s2: f64,
    pub orbital_period_seconds: f64,
    pub orbital_period_days: f64,
    pub circular_speed_m_s: f64,
    pub periapsis_au: f64,
    pub apoapsis_au: f64,
}
pub fn calculate_orbit(r: &OrbitRequest) -> Result<OrbitResult, &'static str> {
    if !r.central_mass_solar.is_finite() || r.central_mass_solar <= 0.0 { return Err("central_mass_solar must be finite and positive"); }
    if !r.semi_major_axis_au.is_finite() || r.semi_major_axis_au <= 0.0 { return Err("semi_major_axis_au must be finite and positive"); }
    if !r.eccentricity.is_finite() || !(0.0..1.0).contains(&r.eccentricity) { return Err("eccentricity must be in [0, 1) for a bound elliptical orbit"); }
    let mass = r.central_mass_solar * SOLAR_MASS_KG;
    let a = r.semi_major_axis_au * AU_METERS;
    let mu = G * mass;
    let period = 2.0 * std::f64::consts::PI * (a.powi(3) / mu).sqrt();
    Ok(OrbitResult {
        gravitational_parameter_m3_s2: mu,
        orbital_period_seconds: period,
        orbital_period_days: period / 86_400.0,
        circular_speed_m_s: (mu / a).sqrt(),
        periapsis_au: r.semi_major_axis_au * (1.0 - r.eccentricity),
        apoapsis_au: r.semi_major_axis_au * (1.0 + r.eccentricity),
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct SurfaceGravityRequest { pub mass_earth: f64, pub radius_earth: f64 }
#[derive(Debug, Serialize)]
pub struct SurfaceGravityResult { pub gravity_m_s2: f64, pub earth_g: f64, pub escape_velocity_m_s: f64 }
pub fn surface_gravity(r: &SurfaceGravityRequest) -> Result<SurfaceGravityResult, &'static str> {
    if !r.mass_earth.is_finite() || r.mass_earth <= 0.0 { return Err("mass_earth must be finite and positive"); }
    if !r.radius_earth.is_finite() || r.radius_earth <= 0.0 { return Err("radius_earth must be finite and positive"); }
    let mass = r.mass_earth * EARTH_MASS_KG;
    let radius = r.radius_earth * 6_371_000.0;
    let g = G * mass / radius.powi(2);
    Ok(SurfaceGravityResult { gravity_m_s2: g, earth_g: g / 9.80665, escape_velocity_m_s: (2.0 * G * mass / radius).sqrt() })
}

#[cfg(test)]
mod tests {
 use super::*;
 #[test] fn earth_orbit_is_about_one_year() {
  let r=calculate_orbit(&OrbitRequest{central_mass_solar:1.0,semi_major_axis_au:1.0,eccentricity:0.0167}).unwrap();
  assert!((r.orbital_period_days-365.256).abs()<0.2);
 }
 #[test] fn earth_gravity_is_about_one_g() {
  let r=surface_gravity(&SurfaceGravityRequest{mass_earth:1.0,radius_earth:1.0}).unwrap();
  assert!((r.earth_g-1.0).abs()<0.002);
 }
}
