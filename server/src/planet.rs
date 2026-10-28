use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetProfile {
    pub id: String,
    pub name: String,
    pub class: PlanetClass,
    pub fictional_class_code: FictionalPlanetClass,
    pub sub_class: String,
    pub seed: u64,
    pub radius_km: f64,
    pub surface_gravity_m_s2: f64,
    pub mean_temperature_c: f64,
    pub atmosphere: AtmosphereProfile,
    pub hydrology: HydrologyProfile,
    pub biology: BiologyProfile,
    pub continents: Vec<Landmass>,
    pub biomes: Vec<BiomeRegion>,
    pub provenance: String,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum PlanetClass { Terrestrial, OceanWorld, Desert, IceWorld, Volcanic, GasGiant, IceGiant, DwarfPlanet, BarrenRock, ArtificialHabitat, Unknown }
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FictionalPlanetClass { D, H, J, K, L, M, N, P, T, Y, R, W, Unknown }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphereProfile { pub composition: Vec<AtmosphereGas>, pub surface_pressure_kpa: f64, pub breathable_for_humans: bool }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphereGas { pub gas: String, pub percentage: f64 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydrologyProfile { pub surface_liquid_water_percent: f64, pub ocean_coverage_percent: f64, pub has_rivers: bool, pub has_lakes: bool, pub has_subsurface_ocean: bool }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologyProfile { pub biosphere: BiosphereClass, pub estimated_species_count: u64, pub complexity_index: f64, pub dominant_domains: Vec<String>, pub notes: String }
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum BiosphereClass { Sterile, PrebioticChemistry, Microbial, SimpleMulticellular, ComplexMulticellular, Technological, Unknown }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Landmass { pub id: String, pub name: String, pub area_percent: f64, pub center_latitude: f64, pub center_longitude: f64, pub elevation_mean_m: f64, pub mountain_fraction: f64 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeRegion { pub name: String, pub area_percent: f64, pub mean_temperature_c: f64, pub precipitation_mm_year: f64, pub elevation_band_m: [i32;2] }

/// Deterministic prototype profile generator. Values are game-world estimates, not measured astronomy.
pub fn generate_planet(seed:u64,id:String,name:String,class:PlanetClass)->PlanetProfile {
 let a=mix(seed); let b=mix(seed^0x9e3779b97f4a7c15); let c=mix(seed^0xd1b54a32d192ed03);
 let unit=|v:u64| (v as f64)/(u64::MAX as f64);
 let gas=matches!(class,PlanetClass::GasGiant|PlanetClass::IceGiant);
 let ocean=matches!(class,PlanetClass::OceanWorld);
 let ice=matches!(class,PlanetClass::IceWorld);
 let volcanic=matches!(class,PlanetClass::Volcanic);
 let barren=matches!(class,PlanetClass::BarrenRock|PlanetClass::DwarfPlanet);
 let fictional_class_code=match class { PlanetClass::Terrestrial=>FictionalPlanetClass::M, PlanetClass::OceanWorld=>FictionalPlanetClass::M, PlanetClass::Desert=>FictionalPlanetClass::H, PlanetClass::IceWorld=>FictionalPlanetClass::P, PlanetClass::Volcanic=>FictionalPlanetClass::N, PlanetClass::GasGiant=>FictionalPlanetClass::J, PlanetClass::IceGiant=>FictionalPlanetClass::T, PlanetClass::DwarfPlanet=>FictionalPlanetClass::D, PlanetClass::BarrenRock=>FictionalPlanetClass::K, PlanetClass::ArtificialHabitat=>FictionalPlanetClass::R, PlanetClass::Unknown=>FictionalPlanetClass::Unknown };
 let radius=if gas {30_000.0+unit(a)*45_000.0} else {2_000.0+unit(a)*9_500.0};
 let gravity=if gas {8.0+unit(b)*20.0} else {1.0+unit(b)*20.0};
 let temp=if ice {-180.0+unit(c)*100.0} else if volcanic {80.0+unit(c)*500.0} else {-70.0+unit(c)*110.0};
 let mut continents=Vec::new();
 if !gas {
   let count=1+(mix(seed^7)%7) as usize;
   let weights:Vec<f64>=(0..count).map(|i|1.0+unit(mix(seed^(i as u64+1)*97))*4.0).collect();
   let total: f64=weights.iter().sum();
   for i in 0..count {
    let lat=unit(mix(seed^(i as u64+11)*31))*140.0-70.0;
    let lon=unit(mix(seed^(i as u64+13)*53))*360.0-180.0;
    continents.push(Landmass{id:format!("{}-land-{}",id,i+1),name:format!("{} {}",["Aurelia","Borealis","Cyr","Demeris","Eld","Faron","Galen"][i%7],["Continent","Major Land","Shield"][i%3]),area_percent:weights[i]/total*(if ocean {100.0-unit(c)*20.0}else{45.0+unit(c)*40.0}),center_latitude:lat,center_longitude:lon,elevation_mean_m:(unit(mix(seed^i as u64*71))*1800.0) as f64,mountain_fraction:unit(mix(seed^i as u64*89))*0.55});
   }
 }
 let biosphere=if gas||barren||volcanic||ice {BiosphereClass::Unknown}else{match mix(seed^55)%4 {0=>BiosphereClass::Sterile,1=>BiosphereClass::PrebioticChemistry,2=>BiosphereClass::Microbial,_=>BiosphereClass::SimpleMulticellular}};
 let species=if matches!(biosphere,BiosphereClass::Microbial|BiosphereClass::SimpleMulticellular){mix(seed^91)%8_000_000+1_000}else{0};
 let bios=if gas {vec![]} else {vec![
  BiomeRegion{name:"Polar ice".into(),area_percent:10.0+unit(a)*20.0,mean_temperature_c:temp-35.0,precipitation_mm_year:unit(b)*300.0,elevation_band_m:[0,1500]},
  BiomeRegion{name:(if ocean{"Pelagic ocean"}else if volcanic{"Volcanic badlands"}else if ice{"Glacier fields"}else{"Temperate woodland"}).into(),area_percent:30.0+unit(b)*30.0,mean_temperature_c:temp,precipitation_mm_year:100.0+unit(c)*1600.0,elevation_band_m:[-5000,2500]},
  BiomeRegion{name:"Arid highlands".into(),area_percent:10.0+unit(c)*20.0,mean_temperature_c:temp+15.0,precipitation_mm_year:unit(a)*250.0,elevation_band_m:[500,5000]}]};
 PlanetProfile{id,name,class,fictional_class_code,sub_class:if gas{"gas envelope".into()}else if ocean{"deep-ocean / archipelago".into()}else if ice{"cryosphere-dominated".into()}else if volcanic{"high volcanic activity".into()}else{"mixed crustal terrain".into()},seed,radius_km:radius,surface_gravity_m_s2:gravity,mean_temperature_c:temp,atmosphere:AtmosphereProfile{composition:if gas{vec![AtmosphereGas{gas:"hydrogen".into(),percentage:82.0},AtmosphereGas{gas:"helium".into(),percentage:17.0},AtmosphereGas{gas:"other gases".into(),percentage:1.0}]}else{vec![AtmosphereGas{gas:"nitrogen".into(),percentage:78.0},AtmosphereGas{gas:"oxygen".into(),percentage:21.0},AtmosphereGas{gas:"trace gases".into(),percentage:1.0}]},surface_pressure_kpa:if gas{0.0}else{45.0+unit(b)*110.0},breathable_for_humans:false},hydrology:HydrologyProfile{surface_liquid_water_percent:if ocean{70.0+unit(a)*30.0}else{unit(b)*20.0},ocean_coverage_percent:if ocean{70.0+unit(c)*30.0}else{unit(a)*40.0},has_rivers:!gas&&!ice&&unit(a)>0.3,has_lakes:!gas&&unit(b)>0.2,has_subsurface_ocean:ice&&unit(c)>0.3},biology:BiologyProfile{biosphere,estimated_species_count:species,complexity_index:unit(mix(seed^101)),dominant_domains:if species>0{vec!["microbial analogues".into()]}else{vec![]},notes:"Procedurally generated fictional biosphere estimate; not a scientific detection.".into()},continents,biomes:bios,provenance:"procedural_estimate".into()}
}
fn mix(mut x:u64)->u64{x^=x>>30;x=x.wrapping_mul(0xbf58476d1ce4e5b9);x^=x>>27;x=x.wrapping_mul(0x94d049bb133111eb);x^(x>>31)}
