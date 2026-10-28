//! Deterministic, coarse-grained procedural settlement road graph.
//! This creates a reproducible graph model; rendering and terrain-aware routing
//! are intentionally separate concerns. Distances are arbitrary world units.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    pub id: String,
    pub kind: SiteKind,
    pub x: f64,
    pub y: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum SiteKind { Capital, Town, Village, Port, Castle, Market, Farm, Mine, RiverCrossing }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadEdge { pub from: String, pub to: String, pub class: RoadClass, pub distance: f64 }
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum RoadClass { Royal, Regional, Local }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadNetwork { pub seed: u64, pub sites: Vec<Site>, pub roads: Vec<RoadEdge> }
#[derive(Debug, Clone, Deserialize)]
pub struct RoadNetworkRequest { pub seed: u64, pub sites: Vec<Site>, pub max_local_distance: f64 }

fn mix(mut x:u64)->u64 { x^=x>>30; x=x.wrapping_mul(0xbf58476d1ce4e5b9); x^=x>>27; x=x.wrapping_mul(0x94d049bb133111eb); x^(x>>31) }
fn distance(a:&Site,b:&Site)->f64 { ((a.x-b.x).powi(2)+(a.y-b.y).powi(2)).sqrt() }

/// Build a deterministic connected backbone and local links between supplied sites.
/// Callers should supply sites sampled from terrain, water crossings and settlement placement.
pub fn generate_road_network(seed:u64, mut sites:Vec<Site>, max_local_distance:f64)->Result<RoadNetwork,&'static str>{
    if !max_local_distance.is_finite() || max_local_distance<=0.0 { return Err("max_local_distance must be positive and finite"); }
    if sites.iter().any(|s| !s.x.is_finite() || !s.y.is_finite() || s.id.trim().is_empty()) { return Err("site ids and coordinates must be valid"); }
    sites.sort_by(|a,b|a.id.cmp(&b.id));
    for pair in sites.windows(2) { if pair[0].id==pair[1].id { return Err("site ids must be unique"); } }
    let mut roads=Vec::new();
    let mut connected=BTreeSet::new();
    if let Some(first)=sites.first(){ connected.insert(first.id.clone()); }
    while connected.len()<sites.len() {
        let mut best:Option<(usize,usize,f64,u64)>=None;
        for (i,a) in sites.iter().enumerate().filter(|(_,s)|connected.contains(&s.id)) {
            for (j,b) in sites.iter().enumerate().filter(|(_,s)|!connected.contains(&s.id)) {
                let d=distance(a,b); let tie=mix(seed ^ (i as u64).rotate_left(11) ^ (j as u64).rotate_left(29));
                if best.map(|(_,_,bd,bt)|d<bd || (d==bd && tie<bt)).unwrap_or(true){best=Some((i,j,d,tie));}
            }
        }
        if let Some((i,j,d,_))=best {
            roads.push(RoadEdge{from:sites[i].id.clone(),to:sites[j].id.clone(),class:if matches!(sites[i].kind,SiteKind::Capital)||matches!(sites[j].kind,SiteKind::Capital){RoadClass::Royal}else{RoadClass::Regional},distance:d});
            connected.insert(sites[j].id.clone());
        } else { break; }
    }
    for i in 0..sites.len() { for j in i+1..sites.len() {
        let d=distance(&sites[i],&sites[j]);
        if d<=max_local_distance && !roads.iter().any(|e|(e.from==sites[i].id&&e.to==sites[j].id)||(e.from==sites[j].id&&e.to==sites[i].id)) {
            roads.push(RoadEdge{from:sites[i].id.clone(),to:sites[j].id.clone(),class:RoadClass::Local,distance:d});
        }
    }}
    Ok(RoadNetwork{seed,sites,roads})
}

#[cfg(test)] mod tests {
 use super::*;
 fn site(id:&str,x:f64,y:f64)->Site{Site{id:id.into(),kind:SiteKind::Town,x,y}}
 #[test] fn same_input_produces_same_network(){let s=vec![site("a",0.,0.),site("b",3.,0.),site("c",8.,1.)];assert_eq!(serde_json::to_string(&generate_road_network(42,s.clone(),4.).unwrap()).unwrap(),serde_json::to_string(&generate_road_network(42,s,4.).unwrap()).unwrap());}
 #[test] fn backbone_connects_every_site(){let n=generate_road_network(1,vec![site("a",0.,0.),site("b",100.,0.),site("c",200.,0.)],2.).unwrap();assert_eq!(n.roads.len(),2);}
}
