/**
 * World Map Generator OS — deterministic turn/tick simulation core.
 * Build ID: WMGO-WORLD-SIM-0.2.0
 * Game simulation only; not a scientific weather or population forecast.
 */

export const SIMULATION_VERSION = "0.2.0";
const clamp = (n, min, max) => Math.max(min, Math.min(max, n));
const finite = (n, fallback = 0) => Number.isFinite(Number(n)) ? Number(n) : fallback;

export function seededRandom(seed = 1) {
  let state = (Number(seed) >>> 0) || 1;
  return () => {
    state = (state + 0x6D2B79F5) >>> 0;
    let t = state;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

export function createWorldState(input = {}) {
  const regions = (input.regions ?? []).map((region, index) => ({
    id: String(region.id ?? ("REG-" + (index + 1))),
    name: String(region.name ?? ("Region " + (index + 1))),
    continent: String(region.continent ?? "Unknown"),
    ownerId: region.ownerId == null ? null : String(region.ownerId),
    terrain: ["plains", "forest", "hills", "mountains", "desert", "wetland"].includes(region.terrain) ? region.terrain : "plains",
    population: Math.max(0, Math.floor(finite(region.population, 10000))),
    food: Math.max(0, finite(region.food, 100)),
    foodCapacity: Math.max(1, finite(region.foodCapacity, 500)),
    unrest: clamp(finite(region.unrest, 10), 0, 100),
    fortification: clamp(finite(region.fortification, 0), 0, 5),
    roads: Array.isArray(region.roads) ? region.roads.map(String) : [],
    weather: { kind: "clear", temperatureC: 15, rain: 0, severity: 0 },
    army: Math.max(0, Math.floor(finite(region.army, 0)))
  }));
  return {
    version: SIMULATION_VERSION, tick: 0, seed: Number(input.seed ?? 1) >>> 0, season: "spring",
    factions: (input.factions ?? []).map((f, index) => ({
      id: String(f.id ?? ("FAC-" + (index + 1))), name: String(f.name ?? ("Faction " + (index + 1))),
      gold: Math.max(0, finite(f.gold, 1000)), food: Math.max(0, finite(f.food, 500)),
      policy: ["balanced", "expansionist", "defensive"].includes(f.policy) ? f.policy : "balanced",
      relations: { ...(f.relations ?? {}) }
    })),
    regions, events: [{ tick: 0, type: "world-created", message: "Simulation initialized." }]
  };
}

export function resolveRiskBattle({ attackers = 1, defenders = 1, attackDice = 3, defendDice = 2, random = Math.random } = {}) {
  const a = Math.max(1, Math.floor(finite(attackers, 1)));
  const d = Math.max(1, Math.floor(finite(defenders, 1)));
  const aDice = clamp(Math.floor(finite(attackDice, 3)), 1, Math.min(3, a > 1 ? a - 1 : 1));
  const dDice = clamp(Math.floor(finite(defendDice, 2)), 1, Math.min(2, d));
  const rollsA = Array.from({ length: aDice }, () => 1 + Math.floor(random() * 6)).sort((x, y) => y - x);
  const rollsD = Array.from({ length: dDice }, () => 1 + Math.floor(random() * 6)).sort((x, y) => y - x);
  let attackerLosses = 0, defenderLosses = 0;
  for (let i = 0; i < Math.min(rollsA.length, rollsD.length); i++) {
    if (rollsA[i] > rollsD[i]) defenderLosses++;
    else attackerLosses++; // defender wins ties
  }
  return { rollsA, rollsD, attackerLosses, defenderLosses, attackersRemaining: a - attackerLosses, defendersRemaining: d - defenderLosses };
}

export function tickWorld(state, { ticks = 1 } = {}) {
  if (!state || !Array.isArray(state.regions) || !Array.isArray(state.factions)) {
    throw new TypeError("tickWorld requires state from createWorldState()");
  }
  const count = clamp(Math.floor(finite(ticks, 1)), 1, 1000);
  const next = structuredClone(state);
  const seasons = ["spring", "summer", "autumn", "winter"];
  const events = [];
  for (let step = 0; step < count; step++) {
    next.tick++;
    next.season = seasons[Math.floor(((next.tick - 1) % 48) / 12)];
    const random = seededRandom((next.seed ^ (next.tick * 2654435761)) >>> 0);
    for (const region of next.regions) {
      // Abstract seasonal weather: deterministic game conditions, not a real forecast.
      const seasonalTemp = { spring: 12, summer: 24, autumn: 10, winter: -2 }[next.season];
      const terrainTemp = region.terrain === "mountains" ? -7 : region.terrain === "desert" ? 7 : 0;
      const temperatureC = Math.round(seasonalTemp + terrainTemp + (random() - 0.5) * 12);
      const rain = Math.round(random() * 100);
      const kind = temperatureC <= 0 && rain > 55 ? "snow" : rain > 78 ? "storm" : rain > 48 ? "rain" : "clear";
      const severity = kind === "storm" ? 1 + Math.floor(random() * 3) : kind === "snow" ? 1 + Math.floor(random() * 2) : 0;
      region.weather = { kind, temperatureC, rain, severity };
      const terrainYield = { plains: 1.2, forest: 0.9, hills: 0.8, mountains: 0.45, desert: 0.25, wetland: 1.0 }[region.terrain];
      const weatherYield = kind === "storm" || kind === "snow" ? 0.55 : kind === "rain" ? 1.05 : 1;
      const production = Math.max(0, Math.floor((region.population / 1000) * terrainYield * weatherYield));
      const consumption = Math.ceil(region.population / 1200) + Math.ceil(region.army / 100);
      region.food = clamp(region.food + production - consumption, 0, region.foodCapacity);
      const shortage = region.food === 0 && consumption > production;
      const growthRate = shortage ? -0.004 : region.food > region.foodCapacity * 0.2 ? 0.0015 : 0.0002;
      region.population = Math.max(0, Math.floor(region.population * (1 + growthRate)));
      region.unrest = clamp(region.unrest + (shortage ? 3 : -0.4) + (region.ownerId ? 0 : 0.1), 0, 100);
      if (shortage) events.push({ tick: next.tick, type: "famine-risk", regionId: region.id, message: "Food shortage in " + region.name + "." });
      if (kind === "storm" && region.roads.length) events.push({ tick: next.tick, type: "weather", regionId: region.id, message: "Storm conditions disrupt travel in " + region.name + "." });
    }
    for (const faction of next.factions) {
      const controlled = next.regions.filter(region => region.ownerId === faction.id);
      const income = controlled.reduce((sum, region) => sum + Math.max(1, Math.floor(region.population / 10000)), 0);
      const upkeep = controlled.reduce((sum, region) => sum + Math.ceil(region.army / 50), 0);
      faction.gold = Math.max(0, faction.gold + income - upkeep);
    }
    if (next.tick % 12 === 0) events.push({ tick: next.tick, type: "season", message: "Season changed to " + next.season + "." });
  }
  next.events = [...next.events, ...events].slice(-500);
  return next;
}

export function getWorldSummary(state) {
  return {
    tick: state.tick, season: state.season, regionCount: state.regions.length,
    population: state.regions.reduce((sum, region) => sum + region.population, 0),
    food: state.regions.reduce((sum, region) => sum + region.food, 0),
    unrestAverage: state.regions.length ? Math.round(state.regions.reduce((sum, region) => sum + region.unrest, 0) / state.regions.length) : 0,
    latestEvents: state.events.slice(-10)
  };
}
