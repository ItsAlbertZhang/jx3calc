use super::*;

enumxlua! { ABSORB_ATTRIBUTE_SHIELD_TYPE {
    GLOBAL,
    PHYSICS,
    SOLAR,
    NEUTRAL,
    LUNAR,
    POISON,
}}

enumxlua! { ATTRIBUTE_EFFECT_MODE {
    EFFECT_TO_SELF_NOT_ROLLBACK,
    EFFECT_TO_SELF_AND_ROLLBACK,
    EFFECT_TO_DEST_NOT_ROLLBACK,
    EFFECT_TO_DEST_AND_ROLLBACK,
}}

enumxlua! { BUFF_COMPARE_FLAG {
    EQUAL,
    GREATER,
    GREATER_EQUAL,
}}

enumxlua! { SKILL_COMPARE_FLAG {
    EQUAL,
    GREATER,
    GREATER_EQUAL,
}}

enumxlua! { GLOBAL {
    GAME_FPS = 16,
}}
