//! Deterministic planetary surface sampling for map tiles.
//! Generated values are fictional simulation terrain, not remote-sensing observations.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass { Terrestrial, OceanWorld, Desert, IceWorld, Volcanic, BarrenRock }

#[derive(Debug, Deserialize)]
pub struct SurfaceTileRequest {
    pub seed: u64,
    pub width: usize,
    pub height: usize,
    pub class: SurfaceClass,
}

#[derive(Debug, Serialize)]
pub struct SurfaceSample {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation_m: f32,
    pub temperature_c: f32,
    pub precipitation_mm_year: f32,
    pub biome: &'static str,
}

#[derive(Debug, Serialize)]
pub struct SurfaceTile {
    pub schema_version: &'static str,
    pub provenance: &'static str,
    pub seed: u64,
    pub width: usize,
    pub height: usize,
    pub samples: Vec<SurfaceSample>,
}

fn mix(mut x: u64) -> u64 {
    x ^= x >> 30; x = x.wrapping_mul(0xbf58476d1ce4e5b9);
    x ^= x >> 27; x = x.wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
fn hash(seed: u64, x: i64, y: i64, octave: u64) -> f64 {
    let v = mix(seed ^ (x as u64).wrapping_mul(0x9e3779b97f4a7c15)
        ^ (y as u64).wrapping_mul(0xd1b54a32d192ed03) ^ octave.wrapping_mul(0x94d049bb133111eb));
    (v as f64 / u64::MAX as f64) * 2.0 - 1.0
}
fn smooth(t: f64) -> f64 { t * t * (3.0 - 2.0 * t) }
fn noise(seed: u64, x: f64, y: f64, octave: u64) -> f64 {
    let ix=x.floor() as i64; let iy=y.floor() as i64;
    let fx=smooth(x-x.floor()); let fy=smooth(y-y.floor());
    let a=hash(seed,ix,iy,octave); let b=hash(seed,ix+1,iy,octave);
    let c=hash(seed,ix,iy+1,octave); let d=hash(seed,ix+1,iy+1,octave);
    let top=a+(b-a)*fx; let bottom=c+(d-c)*fx;
    top+(bottom-top)*fy
}
fn fractal(seed:u64,x:f64,y:f64)->f64 {
    let mut sum=0.0; let mut amplitude=0.55; let mut norm=0.0;
    for octave in 0..6u64 {
        let scale=2f64.powi(octave as i32);
        sum+=noise(seed,x*scale,y*scale,octave)*amplitude;
        norm+=amplitude; amplitude*=0.5;
    }
    sum/norm
}
pub fn generate_surface_tile(r: SurfaceTileRequest) -> Result<SurfaceTile, &'static str> {
    if r.width==0 || r.height==0 || r.width>256 || r.height>128 {
        return Err("width must be 1..256 and height must be 1..128");
    }
    let mut samples=Vec::with_capacity(r.width*r.height);
    for row in 0..r.height {
        let lat=90.0-(row as f64/(r.height.saturating_sub(1).max(1)) as f64)*180.0;
        for col in 0..r.width {
            let lon=col as f64/(r.width.saturating_sub(1).max(1)) as f64*360.0-180.0;
            let nx=lon/360.0; let ny=lat/180.0;
            let continental=fractal(r.seed^0x434f4e54494e454e,nx*5.0+12.0,ny*3.0+8.0);
            let detail=fractal(r.seed^0x5445525241494e,nx*20.0+3.0,ny*12.0+4.0);
            let polar=(lat.abs()/90.0).powf(1.7);
            let mut elevation=(continental*0.78+detail*0.22)*2400.0;
            let ocean_threshold=match r.class { SurfaceClass::OceanWorld=>0.28, SurfaceClass::Terrestrial=>0.02, SurfaceClass::Desert=>0.12, SurfaceClass::IceWorld=>0.05, SurfaceClass::Volcanic=>0.0, SurfaceClass::BarrenRock=>0.15 };
            if matches!(r.class,SurfaceClass::Volcanic) { elevation += detail.abs()*2500.0; }
            if continental < ocean_threshold { elevation=-250.0-detail.abs()*4800.0; }
            let base_temp=match r.class { SurfaceClass::IceWorld=>-65.0, SurfaceClass::Volcanic=>180.0, SurfaceClass::Desert=>31.0, SurfaceClass::BarrenRock=>-15.0, _=>14.0 };
            let temperature=base_temp-48.0*polar-(elevation.max(0.0)/1000.0*6.0)+detail as f64*7.0;
            let precipitation=(850.0*(1.0-polar)*((detail+1.0)/2.0).max(0.0)).max(0.0);
            let biome=if elevation < 0.0 { "ocean" } else if matches!(r.class,SurfaceClass::Volcanic) { "volcanic_terrain" } else if matches!(r.class,SurfaceClass::IceWorld)||temperature < -12.0 { "ice_tundra" } else if temperature > 28.0 && precipitation < 350.0 { "arid_desert" } else if precipitation > 700.0 && temperature > 8.0 { "temperate_forest" } else if polar > 0.75 { "polar_barren" } else { "grassland_steppe" };
            samples.push(SurfaceSample { latitude:lat, longitude:lon, elevation_m:elevation as f32, temperature_c:temperature as f32, precipitation_mm_year:precipitation as f32, biome });
        }
    }
    Ok(SurfaceTile { schema_version:"planet-surface-tile.v1", provenance:"procedural_estimate", seed:r.seed, width:r.width, height:r.height, samples })
}

#[cfg(test)]
mod tests {
 use super::*;
 #[test] fn deterministic_surface() {
   let a=generate_surface_tile(SurfaceTileRequest{seed:17,width:8,height:4,class:SurfaceClass::Terrestrial}).unwrap();
   let b=generate_surface_tile(SurfaceTileRequest{seed:17,width:8,height:4,class:SurfaceClass::Terrestrial}).unwrap();
   assert_eq!(serde_json::to_string(&a).unwrap(),serde_json::to_string(&b).unwrap());
 }
 #[test] fn bounds_are_checked() {
   assert!(generate_surface_tile(SurfaceTileRequest{seed:1,width:257,height:1,class:SurfaceClass::Terrestrial}).is_err());
 }
}
