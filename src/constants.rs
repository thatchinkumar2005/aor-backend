pub const MAP_SIZE: usize = 40;
pub const TOTAL_ATTACKS_PER_DAY: i64 = 50;
pub const ROAD_ID: i32 = 0;
pub const BANK_BUILDING_NAME: &str = "Bank";
pub const INITIAL_RATING: i32 = 1000;
pub const INITIAL_ARTIFACTS: i32 = 500;
pub const WIN_THRESHOLD: i32 = 50;
pub const SCALE_FACTOR: f32 = 20.0;
pub const HIGHEST_TROPHY: f32 = 2_000.0;
pub const MAX_BOMBS_PER_ATTACK: i32 = 30;
pub const ATTACK_TOKEN_AGE_IN_MINUTES: i64 = 5;
pub const GAME_AGE_IN_MINUTES: usize = 3;
pub const MATCH_MAKING_ATTEMPTS: i32 = 10;
pub const PERCENTANGE_ARTIFACTS_OBTAINABLE: f32 = 0.3;
pub const BOMB_DAMAGE_MULTIPLIER: f32 = 5.0;
pub const COMPANION_BOT_RANGE: i32 = 5;

pub struct HutLevelAttribute {
    pub defenders_limit: i32,
}

pub struct LevelAttributes {
    pub hut: HutLevelAttribute,
}

pub const LEVEL: [LevelAttributes; 3] = [
    LevelAttributes {
        hut: HutLevelAttribute { defenders_limit: 3 },
    },
    LevelAttributes {
        hut: HutLevelAttribute { defenders_limit: 4 },
    },
    LevelAttributes {
        hut: HutLevelAttribute { defenders_limit: 5 },
    },
];

pub const LIVES: i32 = 3;

pub struct CompanionPriority {
    pub defenders: i32,
    pub defender_buildings: i32,
    pub buildings: i32,
}

pub const companion_priority: CompanionPriority = CompanionPriority {
    defenders: 3,
    defender_buildings: 2,
    buildings: 1,
};
pub const DAMAGE_PER_BULLET_LEVEL_1: i32 = 5;
pub const DAMAGE_PER_BULLET_LEVEL_2: i32 = 7;
pub const DAMAGE_PER_BULLET_LEVEL_3: i32 = 10;
pub const BULLET_COLLISION_TIME: i32 = 2;
