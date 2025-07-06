#!/usr/bin/env python3
"""
Consolidate all god, ability, and item data into complete JSON files
"""

import json
import re
from pathlib import Path
from typing import Dict, List, Any

def slugify(text: str) -> str:
    """Convert text to slug format"""
    text = text.replace("'", "").replace('"', '')
    text = re.sub(r'[^\w\s-]', '', text)
    text = re.sub(r'[-\s]+', '_', text)
    return text.lower()

def load_json(filepath: Path) -> Any:
    """Load JSON file"""
    if filepath.exists():
        with open(filepath, 'r', encoding='utf-8') as f:
            return json.load(f)
    return {}

def save_json(data: Any, filepath: Path):
    """Save data as JSON"""
    with open(filepath, 'w', encoding='utf-8') as f:
        json.dump(data, f, indent=2, ensure_ascii=False)

def get_all_gods_data():
    """Define complete list of all Smite gods with their classes"""
    # This is a comprehensive list of all current Smite gods
    all_gods = {
        # Warriors
        "achilles": ("Achilles", "Warrior"),
        "amaterasu": ("Amaterasu", "Warrior"),
        "bellona": ("Bellona", "Warrior"),
        "chaac": ("Chaac", "Warrior"),
        "cu_chulainn": ("Cu Chulainn", "Warrior"),
        "erlang_shen": ("Erlang Shen", "Warrior"),
        "gilgamesh": ("Gilgamesh", "Warrior"),
        "guan_yu": ("Guan Yu", "Warrior"),
        "hercules": ("Hercules", "Warrior"),
        "horus": ("Horus", "Warrior"),
        "king_arthur": ("King Arthur", "Warrior"),
        "mulan": ("Mulan", "Warrior"),
        "nike": ("Nike", "Warrior"),
        "odin": ("Odin", "Warrior"),
        "osiris": ("Osiris", "Warrior"),
        "shiva": ("Shiva", "Warrior"),
        "sun_wukong": ("Sun Wukong", "Warrior"),
        "surtr": ("Surtr", "Warrior"),
        "tyr": ("Tyr", "Warrior"),
        "vamana": ("Vamana", "Warrior"),
        
        # Assassins
        "arachne": ("Arachne", "Assassin"),
        "awilix": ("Awilix", "Assassin"),
        "bakasura": ("Bakasura", "Assassin"),
        "bastet": ("Bastet", "Assassin"),
        "camazotz": ("Camazotz", "Assassin"),
        "cliodhna": ("Cliodhna", "Assassin"),
        "da_ji": ("Da Ji", "Assassin"),
        "fenrir": ("Fenrir", "Assassin"),
        "hun_batz": ("Hun Batz", "Assassin"),
        "kali": ("Kali", "Assassin"),
        "lancelot": ("Lancelot", "Assassin"),
        "loki": ("Loki", "Assassin"),
        "mercury": ("Mercury", "Assassin"),
        "ne_zha": ("Ne Zha", "Assassin"),
        "nemesis": ("Nemesis", "Assassin"),
        "pele": ("Pele", "Assassin"),
        "ratatoskr": ("Ratatoskr", "Assassin"),
        "ravana": ("Ravana", "Assassin"),
        "serqet": ("Serqet", "Assassin"),
        "set": ("Set", "Assassin"),
        "susano": ("Susano", "Assassin"),
        "thanatos": ("Thanatos", "Assassin"),
        "thor": ("Thor", "Assassin"),
        "tsukuyomi": ("Tsukuyomi", "Assassin"),
        
        # Hunters
        "ah_muzen_cab": ("Ah Muzen Cab", "Hunter"),
        "anhur": ("Anhur", "Hunter"),
        "apollo": ("Apollo", "Hunter"),
        "artemis": ("Artemis", "Hunter"),
        "cernunnos": ("Cernunnos", "Hunter"),
        "charybdis": ("Charybdis", "Hunter"),
        "chernobog": ("Chernobog", "Hunter"),
        "chiron": ("Chiron", "Hunter"),
        "cupid": ("Cupid", "Hunter"),
        "danzaburou": ("Danzaburou", "Hunter"),
        "hachiman": ("Hachiman", "Hunter"),
        "heimdallr": ("Heimdallr", "Hunter"),
        "hou_yi": ("Hou Yi", "Hunter"),
        "ishtar": ("Ishtar", "Hunter"),
        "izanami": ("Izanami", "Hunter"),
        "jing_wei": ("Jing Wei", "Hunter"),
        "martichoras": ("Martichoras", "Hunter"),
        "medusa": ("Medusa", "Hunter"),
        "neith": ("Neith", "Hunter"),
        "nut": ("Nut", "Hunter"),
        "rama": ("Rama", "Hunter"),
        "skadi": ("Skadi", "Hunter"),
        "ullr": ("Ullr", "Hunter"),
        "xbalanque": ("Xbalanque", "Hunter"),
        
        # Mages
        "agni": ("Agni", "Mage"),
        "ah_puch": ("Ah Puch", "Mage"),
        "anubis": ("Anubis", "Mage"),
        "ao_kuang": ("Ao Kuang", "Mage"),
        "aphrodite": ("Aphrodite", "Mage"),
        "baba_yaga": ("Baba Yaga", "Mage"),
        "baron_samedi": ("Baron Samedi", "Mage"),
        "change": ("Chang'e", "Mage"),
        "chronos": ("Chronos", "Mage"),
        "discordia": ("Discordia", "Mage"),
        "eset": ("Eset", "Mage"),
        "freya": ("Freya", "Mage"),
        "hades": ("Hades", "Mage"),
        "he_bo": ("He Bo", "Mage"),
        "hel": ("Hel", "Mage"),
        "hera": ("Hera", "Mage"),
        "ix_chel": ("Ix Chel", "Mage"),
        "janus": ("Janus", "Mage"),
        "kukulkan": ("Kukulkan", "Mage"),
        "maman_brigitte": ("Maman Brigitte", "Mage"),
        "merlin": ("Merlin", "Mage"),
        "morgan_le_fay": ("Morgan Le Fay", "Mage"),
        "nox": ("Nox", "Mage"),
        "nu_wa": ("Nu Wa", "Mage"),
        "olorun": ("Olorun", "Mage"),
        "persephone": ("Persephone", "Mage"),
        "poseidon": ("Poseidon", "Mage"),
        "ra": ("Ra", "Mage"),
        "raijin": ("Raijin", "Mage"),
        "scylla": ("Scylla", "Mage"),
        "sol": ("Sol", "Mage"),
        "the_morrigan": ("The Morrigan", "Mage"),
        "thoth": ("Thoth", "Mage"),
        "tiamat": ("Tiamat", "Mage"),
        "vulcan": ("Vulcan", "Mage"),
        "yu_huang": ("Yu Huang", "Mage"),
        "zeus": ("Zeus", "Mage"),
        "zhong_kui": ("Zhong Kui", "Mage"),
        
        # Guardians
        "ares": ("Ares", "Guardian"),
        "artio": ("Artio", "Guardian"),
        "athena": ("Athena", "Guardian"),
        "atlas": ("Atlas", "Guardian"),
        "bacchus": ("Bacchus", "Guardian"),
        "bake_kujira": ("Bake Kujira", "Guardian"),
        "cabrakan": ("Cabrakan", "Guardian"),
        "cerberus": ("Cerberus", "Guardian"),
        "charon": ("Charon", "Guardian"),
        "cthulhu": ("Cthulhu", "Guardian"),
        "fafnir": ("Fafnir", "Guardian"),
        "ganesha": ("Ganesha", "Guardian"),
        "geb": ("Geb", "Guardian"),
        "jormungandr": ("Jormungandr", "Guardian"),
        "khepri": ("Khepri", "Guardian"),
        "kumbhakarna": ("Kumbhakarna", "Guardian"),
        "kuzenbo": ("Kuzenbo", "Guardian"),
        "maui": ("Maui", "Guardian"),
        "sobek": ("Sobek", "Guardian"),
        "sylvanus": ("Sylvanus", "Guardian"),
        "terra": ("Terra", "Guardian"),
        "xing_tian": ("Xing Tian", "Guardian"),
        "yemoja": ("Yemoja", "Guardian"),
        "ymir": ("Ymir", "Guardian"),
    }
    
    return all_gods

def get_god_abilities():
    """Get ability mappings for all gods"""
    # Standard ability structure: passive, ability1, ability2, ability3, ultimate
    god_abilities = {
        # Warriors
        "achilles": ["gift_of_the_gods", "shield_of_achilles", "radiant_glory", "combat_dodge", "fatal_strike"],
        "amaterasu": ["illuminating_strike", "divine_presence", "heavenly_reflection", "glorious_charge", "dazzling_offensive"],
        "bellona": ["master_of_war", "shield_bash", "bludgeon", "scourge", "eagles_rally"],
        "chaac": ["overflow", "thunder_strike", "torrent", "rain_dance", "storm_call"],
        "cu_chulainn": ["berserk", "barbed_spear_ground_slam", "vent_anger_salmon_leap", "rage_furious_charge", "spear_of_mortal_pain_war_cry"],
        "erlang_shen": ["howling_celestial_dog", "spot_weakness", "pin", "nine_turns_blessing", "9_form_arcane"],
        "gilgamesh": ["epic_of_gilgamesh", "sun_forged_scimitar", "drop_kick", "heros_advance", "winds_of_shamash"],
        "guan_yu": ["painless", "conviction", "warrior_will", "cavalry_charge", "the_lost_kingdom"],
        "hercules": ["strength_from_pain", "driving_strike", "earthbreaker", "mitigate_wounds", "excavate"],
        "horus": ["resolute", "updraft", "fracture", "protectors_surge", "to_the_skies"],
        "king_arthur": ["steadfast", "overhead_slash_hamstring", "battle_stomp_uppercut", "twin_cleave_bladestorm", "excaliburs_wrath_sundering_strike"],
        "mulan": ["training_arc", "cross_strike", "spear_thrust", "grapple", "divine_spear"],
        "nike": ["to_victory", "rend", "plan_of_action", "valiant_leap", "sentinel_of_zeus"],
        "odin": ["path_to_valhalla", "lunge", "raven_shout", "gungnirs_might", "ring_of_spears"],
        "osiris": ["fragmented", "sickle_strike", "spirit_flail", "judgement_tether", "lord_of_the_afterlife"],
        "shiva": ["tandava_karma", "bladed_arrow", "pillar_of_dawn", "dash", "shivas_destruction"],
        "sun_wukong": ["undefeated_body", "magic_cudgel", "master_will", "72_transformations", "somersault_cloud"],
        "surtr": ["obsidian_flesh", "flames_of_muspell", "magma_wall", "blazing_trail", "giant_inferno"],
        "tyr": ["unyielding", "fearless_power_cleave", "change_stance", "lawbringer_guard_stance", "lawbringer"],
        "vamana": ["sleeping_giant", "clear_the_path", "armored_umbrella", "umbrellarang", "colossal_fury"],
        
        # Assassins
        "arachne": ["predator", "venomous_bite", "cocoon", "web", "night_crawler"],
        "awilix": ["initiative", "summon_suku", "feather_step", "moonlight_charge", "gravity_surge"],
        "bakasura": ["insatiable_hunger", "take_down", "eat_minion", "butcher_blades", "regurgitate"],
        "bastet": ["nightstalker", "pounce", "razor_whip", "declaw", "huntress_of_bast"],
        "camazotz": ["essence_drinker", "screech", "vampire_bats", "devour", "bat_out_of_hell"],
        "cliodhna": ["lurking_in_the_shadows", "banshees_wail", "deafening_whispers", "ghostly_presence", "rushing_terror"],
        "da_ji": ["torture_blades", "horrible_burns", "one_thousand_cuts", "trickster_spirit", "pao_lao"],
        "fenrir": ["unbound_runes", "unchained", "seething_howl", "brutalize", "ragnarok"],
        "hun_batz": ["infused_strikes", "somersault", "overhand_smash", "sacred_monkey", "fear_no_evil"],
        "kali": ["marked_for_death", "nimble_strike", "lash", "incense", "destruction"],
        "lancelot": ["knights_virtue", "piercing_thrust", "skilled_strikes", "mount_up", "the_grand_joust"],
        "loki": ["behind_you", "vanish", "decoy", "aimed_strike", "assassinate"],
        "mercury": ["fastest_god_alive", "made_you_look", "maximum_velocity", "special_delivery", "sonic_boom"],
        "ne_zha": ["righteous_spirit", "universe_ring_toss", "flaming_spear", "armillary_sash", "wind_fire_wheels"],
        "nemesis": ["scales_of_fate", "swift_vengeance", "slice_and_dice", "retribution", "divine_judgement"],
        "pele": ["everlasting_flame", "pyroclast", "eruption", "magma_rush", "volcanic_lightning"],
        "ratatoskr": ["acorn_of_yggdrasil", "dart", "flurry", "acorn_blast", "through_the_cosmos"],
        "ravana": ["chain_of_blows", "prana_onslaught", "overhead_kick", "10_hand_shadow_fist", "mystic_rush"],
        "serqet": ["catalyst", "deathbane", "cobras_kiss", "ambush", "last_breath"],
        "set": ["relentless", "skewer", "spawn_of_set", "sandstorm", "kingslayer"],
        "susano": ["swift_as_the_summer_storm", "storm_kata", "wind_siphon", "jet_stream", "typhoon"],
        "thanatos": ["harvester_of_souls", "death_scythe", "scent_of_death", "soul_reap", "hovering_death"],
        "thor": ["warriors_madness", "mjolnirs_attunement", "tectonic_rift", "berserker_barrage", "anvil_of_dawn"],
        "tsukuyomi": ["shingetsu_aura", "dark_moon_shuriken", "kusarigama", "silver_moon_caltrops", "piercing_moonlight"],
        
        # Hunters
        "ah_muzen_cab": ["bees", "hive", "swarm", "honey", "stinger"],
        "anhur": ["enfeeble", "shifting_sands", "impale", "disperse", "desert_fury"],
        "apollo": ["audacity", "so_beautiful", "serenade", "the_moves", "across_the_sky"],
        "artemis": ["still_target", "transgessors_fate", "seek_and_destroy", "suppress_the_insolent", "calydonian_boar"],
        "cernunnos": ["heavy_glaive", "shifter_of_seasons", "bramble_blast", "horn_charge", "the_wild_hunt"],
        "charybdis": ["raging_tides", "spike_shot", "capsize", "whirlpool_form", "the_maw_hungers"],
        "chernobog": ["heart_of_cold", "crystallized_curses", "vicious_barrage", "into_darkness", "living_nightmare"],
        "chiron": ["herbal_medicine", "centaurus", "masterful_shot", "giddyup", "hierophant"],
        "cupid": ["lovestruck", "heart_bomb", "share_the_love", "flutter", "fields_of_love"],
        "danzaburou": ["dubious_savings", "fools_gold", "alluring_spirits", "tanuki_trickery", "uproarious_rocket"],
        "hachiman": ["master_of_arms", "eagle_eye", "heavenly_banner", "iaijutsu", "mounted_archery"],
        "heimdallr": ["the_vigilant", "piercing_sight", "gjallarhorn", "the_bifrost", "through_the_realms"],
        "hou_yi": ["golden_crow", "ricochet", "mark_of_the_golden_crow", "dive_bomb", "sunbreaker"],
        "ishtar": ["mark_of_vengeance", "imbue_arrows", "rolling_thunder", "jolt", "blades_of_retribution"],
        "izanami": ["of_death_and_darkness", "sickle_storm", "spectral_projection", "fade_away", "dark_portal"],
        "jing_wei": ["rapid_reincarnation", "persistent_gust", "explosive_bolts", "agility", "air_strike"],
        "martichoras": ["barbed_spines", "stinger_shot", "acid_spray", "trail_blazer", "death_from_above"],
        "medusa": ["sidewinder", "viper_shot", "acid_spray", "lacerate", "petrify"],
        "neith": ["broken_weave", "spirit_arrow", "unravel", "back_flip", "world_weaver"],
        "nut": ["broken_barrier", "convergence", "skyfall", "gravity_well", "across_the_cosmos"],
        "rama": ["astral_quiver", "astral_strike", "pick_me_up", "rolling_assault", "astral_barrage"],
        "skadi": ["kaldr_the_winter_wolf", "piercing_cold", "rune_of_the_hunt", "permafrost", "winters_grasp"],
        "ullr": ["weapon_master", "bladed_arrow_thrown_axe", "expose_weakness_invigorate", "hail_of_arrows_glory_bound", "wield_axes_wield_bow"],
        "xbalanque": ["dead_of_night", "branching_bola", "poison_darts", "rising_jaguar", "darkest_of_nights"],
        
        # Mages
        "agni": ["combustion", "noxious_fumes", "flame_wave", "path_of_flames", "rain_fire"],
        "ah_puch": ["hollow_ground", "undead_surge", "corpse_explosion", "fleeting_breath", "empty_the_crypts"],
        "anubis": ["sorrow", "plague_of_locusts", "mummify", "grasping_hands", "death_gaze"],
        "ao_kuang": ["dragon_kings_sword", "water_illusion", "dragon_call", "wild_storm", "king_of_the_eastern_seas"],
        "aphrodite": ["loving_embrace", "kiss", "back_off", "love_birds", "undying_love"],
        "baba_yaga": ["creeping_cabin", "baba_brew", "baba_blast", "home_run", "home_sweet_home"],
        "baron_samedi": ["hysteria", "vivid_gaze", "consign_spirits", "wrap_it_up", "life_of_the_party"],
        "change": ["jade_rabbit", "crescent_moon_dance", "moonlit_waltz", "moonflower_dance", "waxing_moon"],
        "chronos": ["the_wheel_of_time", "time_rift", "accelerate", "stop_time", "rewind"],
        "discordia": ["contest_of_gods", "unruly_magic", "strife", "erratic_behavior", "golden_apple_of_discord"],
        "eset": ["funeral_rites", "wing_gust", "spirit_ball", "dispel_magic", "circle_of_protection"],
        "freya": ["brisingamens_blessing", "irradiate", "pulse", "banish", "valkyries_discretion"],
        "hades": ["blight", "death_from_below", "shroud_of_darkness", "devour_souls", "pillar_of_agony"],
        "he_bo": ["steady_flow", "water_cannon", "atlas_of_the_yellow_river", "waterspout", "crushing_wave"],
        "hel": ["stance_attunement", "decay_restoration", "hinder_cleanse", "repulse_inspire", "switch_stances"],
        "hera": ["commanding_presence", "royal_assault", "polymorph", "divine_shroud", "argus_the_defender"],
        "ix_chel": ["rainbow_weaver", "gleaming_blast", "strands_of_moonlight", "lunar_invocation", "great_deluge"],
        "janus": ["passages", "portal", "unstable_vortex", "threshold", "through_space_and_time"],
        "kukulkan": ["power_of_the_wind_jewel", "zephyr", "slipstream", "whirlwind", "spirit_of_the_nine_winds"],
        "maman_brigitte": ["consort_of_baron", "madame_fangs", "soul_spikes", "spiritual_seance", "party_trick"],
        "merlin": ["overload", "eclipse_radiate", "frostbolt_dragonfire", "blizzard_inferno", "elemental_mastery"],
        "morgan_le_fay": ["empowered_blade", "sigil_mastery", "shroud_of_mist", "banish", "consuming_power"],
        "nox": ["flame_of_the_night", "shadow_lock", "siphon_darkness", "shadow_step", "night_terror"],
        "nu_wa": ["strength_of_wood", "mysterious_fog", "clay_soldiers", "shining_metal", "fire_shards"],
        "olorun": ["touch_of_fate", "focused_light", "overflowing_divinity", "consecration", "sanctified_field"],
        "persephone": ["pomegranate_seeds", "bone_rush", "flourish", "grasp_of_death", "life_death_rebirth"],
        "poseidon": ["changing_tides", "tidal_surge", "trident", "whirlpool", "release_the_kraken"],
        "ra": ["speed_of_light", "celestial_beam", "divine_light", "solar_blessing", "searing_pain"],
        "raijin": ["charged_tempo", "percussive_storm", "thunder_crash", "raiju", "taiko_drums"],
        "scylla": ["quick_learner", "sic_em", "crush", "sentinel", "im_a_monster"],
        "sol": ["unstable_manifestation", "radiance", "stellar_burst", "disapparate", "supernova"],
        "the_morrigan": ["doomsayer", "deadly_aspects", "dark_omen", "confusion", "changeling"],
        "thoth": ["dead_reckoning", "hieroglyphic_assault", "evade_and_punish", "glyph_of_pain", "final_judgement"],
        "tiamat": ["primordial_onslaught", "primordial_onslaught", "consume", "grounding_dive_ruination", "children_of_creation_summon_serpents"],
        "vulcan": ["master_craftsman", "backfire", "inferno_cannon", "magma_bomb", "earthshaker"],
        "yu_huang": ["master_of_the_dao", "flames_of_the_phoenix", "dao_cultivation", "celestial_flight", "dueling_dragons"],
        "zeus": ["overcharge", "chain_lightning", "aegis_assault", "detonate_charge", "lightning_storm"],
        "zhong_kui": ["demon_bag", "expose_evil", "exorcism", "book_of_demons", "recall_demons"],
        
        # Guardians
        "ares": ["blessed_presence", "shackles", "bolster_defenses", "searing_flesh", "no_escape"],
        "artio": ["decompose", "energy_surge_maul", "ferocious_roar_entangling_vines", "heavy_charge_life_tap", "shapeshift"],
        "athena": ["reach", "preemptive_strike", "confound", "shield_wall", "defender_of_olympus"],
        "atlas": ["the_astrolabe", "unburden", "gravity_pull", "kinetic_charge", "gamma_ray_burst"],
        "bacchus": ["drunk_o_meter", "chug", "belly_flop", "belch_of_the_gods", "intoxicate"],
        "bake_kujira": ["ghostly_crew", "leviathan_charge", "whales_bellow", "brackish_waters", "wrath_of_the_deep"],
        "cabrakan": ["shadow_zone", "seismic_crush", "refraction_shield", "tremors", "tectonic_shift"],
        "cerberus": ["spirit_of_death", "paralyzing_spit", "ghastly_breath", "soul_expulsion", "stygian_torment"],
        "charon": ["stygian_judgment", "spectral_surge", "tormented_souls", "swift_death", "summon_stygian_claw"],
        "cthulhu": ["prey_on_fear", "sanity_break", "the_mire", "rushing_terror", "descend_into_madness"],
        "fafnir": ["endless_greed", "cursed_strength", "coerce", "underhanded_tactics", "dragonic_corruption"],
        "ganesha": ["good_fortune", "turn_of_fate", "ohm", "remove_obstacles", "dharmic_pillars"],
        "geb": ["hard_as_rock", "roll_out", "shock_wave", "stone_shield", "cataclysm"],
        "jormungandr": ["immovable", "venomous_haze", "consuming_bellow", "submerge", "the_world_serpent"],
        "khepri": ["fortitude", "abduct", "rising_dawn", "solar_flare", "scarabs_blessing"],
        "kumbhakarna": ["sleepy", "throw_back", "mighty_yawn", "groggy_strike", "epic_uppercut"],
        "kuzenbo": ["water_bowl", "nene_kappa", "shell_spikes", "sumo_slam", "watery_grave"],
        "maui": ["heros_entrance", "master_fisherman", "mystical_ulua", "solar_swing", "landfall"],
        "sobek": ["blessing_of_the_nile", "charge_prey", "tail_whip", "sickening_strike", "lurking_in_the_waters"],
        "sylvanus": ["natures_protection", "verdant_growth", "wisps", "nature_grasp", "wrath_of_terra"],
        "terra": ["standing_stones", "force_of_nature", "crushing_earth", "monolith", "terra_blessing"],
        "xing_tian": ["smoldering_rage", "furious_roar", "hook_slam", "sky_cutting_axe", "whirlwind_of_rage_and_steel"],
        "yemoja": ["omi", "mending_waters", "bouncing_bubble", "riptide", "river_of_sorrow"],
        "ymir": ["frostbite", "ice_wall", "glacial_strike", "frost_breath", "shards_of_ice"],
    }
    
    return god_abilities

def create_ability_details(ability_name: str, display_name: str) -> Dict[str, Any]:
    """Create basic ability details structure"""
    return {
        "display_name": display_name,
        "description": f"{display_name} ability",  # Basic placeholder
        "details": {}
    }

def consolidate_data():
    """Main consolidation function"""
    # Paths
    data_dir = Path("/home/jakeo/proj/grappul/src/data/json")
    
    # Load existing data
    existing_gods = load_json(data_dir / "gods.json")
    existing_abilities = load_json(data_dir / "abilities.json")
    existing_items = load_json(data_dir / "items.json")
    
    # Get all gods and their abilities
    all_gods_info = get_all_gods_data()
    god_abilities_mapping = get_god_abilities()
    
    # Create complete gods data
    consolidated_gods = {}
    for god_key, (display_name, class_name) in all_gods_info.items():
        # Get existing data if available
        existing_god = existing_gods.get(god_key, {})
        
        # Get abilities for this god
        abilities = god_abilities_mapping.get(god_key, [])
        
        consolidated_gods[god_key] = {
            "display_name": display_name,
            "class": class_name,
            "abilities": abilities
        }
    
    # Create complete abilities data
    consolidated_abilities = {}
    
    # First, include all existing abilities
    for ability_key, ability_data in existing_abilities.items():
        consolidated_abilities[ability_key] = ability_data
    
    # Then, add any missing abilities from god data
    for god_key, abilities in god_abilities_mapping.items():
        for ability_name in abilities:
            if ability_name and ability_name not in consolidated_abilities:
                # Create a basic ability entry
                display_name = ability_name.replace('_', ' ').title()
                consolidated_abilities[ability_name] = create_ability_details(
                    ability_name, display_name
                )
    
    # Items are already consolidated, just ensure they have proper structure
    consolidated_items = {}
    for item_key, item_data in existing_items.items():
        # Ensure all fields exist
        consolidated_items[item_key] = {
            "display_name": item_data.get("display_name", item_key.replace('_', ' ').title()),
            "price": item_data.get("price", 0),
            "stats": item_data.get("stats", []),
            "effects": item_data.get("effects", [])
        }
    
    # Print statistics
    print(f"Total gods: {len(consolidated_gods)}")
    print(f"Gods with abilities: {sum(1 for g in consolidated_gods.values() if g['abilities'])}")
    print(f"Total abilities: {len(consolidated_abilities)}")
    print(f"Total items: {len(consolidated_items)}")
    
    # Count missing data
    gods_without_abilities = [g for g, data in consolidated_gods.items() if not data['abilities']]
    if gods_without_abilities:
        print(f"\nGods without abilities ({len(gods_without_abilities)}):")
        for god in gods_without_abilities[:10]:
            print(f"  - {consolidated_gods[god]['display_name']}")
        if len(gods_without_abilities) > 10:
            print(f"  ... and {len(gods_without_abilities) - 10} more")
    
    # Save consolidated data
    save_json(consolidated_gods, data_dir / "gods_consolidated.json")
    save_json(consolidated_abilities, data_dir / "abilities_consolidated.json") 
    save_json(consolidated_items, data_dir / "items_consolidated.json")
    
    print("\nConsolidated data saved to:")
    print("  - gods_consolidated.json")
    print("  - abilities_consolidated.json")
    print("  - items_consolidated.json")

if __name__ == "__main__":
    consolidate_data()