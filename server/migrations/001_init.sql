CREATE TABLE IF NOT EXISTS players (
  id CHAR(36) PRIMARY KEY,
  username VARCHAR(32) NOT NULL UNIQUE,
  password_hash VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS universes (
  id CHAR(36) PRIMARY KEY,
  name VARCHAR(96) NOT NULL,
  seed VARCHAR(128) NOT NULL,
  generation_version VARCHAR(32) NOT NULL DEFAULT '0.1.0',
  created_by CHAR(36) NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_universe_creator FOREIGN KEY (created_by) REFERENCES players(id) ON DELETE SET NULL
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS star_systems (
  id CHAR(36) PRIMARY KEY,
  universe_id CHAR(36) NOT NULL,
  sector_number SMALLINT UNSIGNED NOT NULL,
  quadrant TINYINT UNSIGNED NOT NULL,
  system_index INT UNSIGNED NOT NULL,
  name VARCHAR(96) NOT NULL,
  seed BIGINT UNSIGNED NOT NULL,
  data JSON NOT NULL,
  UNIQUE KEY uq_system_position (universe_id, quadrant, sector_number, system_index),
  KEY idx_system_universe (universe_id),
  CONSTRAINT fk_system_universe FOREIGN KEY (universe_id) REFERENCES universes(id) ON DELETE CASCADE
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS celestial_bodies (
  id CHAR(36) PRIMARY KEY,
  system_id CHAR(36) NOT NULL,
  parent_body_id CHAR(36) NULL,
  body_type ENUM('star','planet','moon','asteroid','comet') NOT NULL,
  name VARCHAR(96) NOT NULL,
  seed BIGINT UNSIGNED NOT NULL,
  orbital_radius DOUBLE NULL,
  orbital_period DOUBLE NULL,
  radius_km DOUBLE NULL,
  mass_kg DOUBLE NULL,
  axial_tilt_deg DOUBLE NULL,
  rotation_hours DOUBLE NULL,
  scientific_data JSON NOT NULL,
  generated_data JSON NOT NULL,
  KEY idx_body_system (system_id),
  CONSTRAINT fk_body_system FOREIGN KEY (system_id) REFERENCES star_systems(id) ON DELETE CASCADE,
  CONSTRAINT fk_body_parent FOREIGN KEY (parent_body_id) REFERENCES celestial_bodies(id) ON DELETE SET NULL
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS settlements (
  id CHAR(36) PRIMARY KEY,
  body_id CHAR(36) NOT NULL,
  owner_player_id CHAR(36) NULL,
  name VARCHAR(96) NOT NULL,
  x DOUBLE NOT NULL,
  y DOUBLE NOT NULL,
  population BIGINT UNSIGNED NOT NULL DEFAULT 0,
  state JSON NOT NULL,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  KEY idx_settlement_body (body_id),
  KEY idx_settlement_owner (owner_player_id),
  CONSTRAINT fk_settlement_body FOREIGN KEY (body_id) REFERENCES celestial_bodies(id) ON DELETE CASCADE,
  CONSTRAINT fk_settlement_owner FOREIGN KEY (owner_player_id) REFERENCES players(id) ON DELETE SET NULL
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS player_sessions (
  id CHAR(36) PRIMARY KEY,
  player_id CHAR(36) NOT NULL,
  token_hash CHAR(64) NOT NULL UNIQUE,
  expires_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  KEY idx_session_player (player_id),
  CONSTRAINT fk_session_player FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE
) ENGINE=InnoDB;

CREATE TABLE IF NOT EXISTS universe_events (
  id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
  universe_id CHAR(36) NOT NULL,
  player_id CHAR(36) NULL,
  event_type VARCHAR(48) NOT NULL,
  event_data JSON NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  KEY idx_events_universe_time (universe_id, created_at),
  CONSTRAINT fk_event_universe FOREIGN KEY (universe_id) REFERENCES universes(id) ON DELETE CASCADE,
  CONSTRAINT fk_event_player FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE SET NULL
) ENGINE=InnoDB;
