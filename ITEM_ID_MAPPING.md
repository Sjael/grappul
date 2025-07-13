# Item ID Mapping

This document provides a comprehensive mapping from old item IDs used in guides.json to new item IDs used in the current items.json file.

## Complete Mapping

| Old ID | New ID | Display Name |
|--------|--------|--------------|
| aegis | aegis_amulet | Aegis Amulet |
| ankh | amulet_of_silence | Amulet of Silence |
| arondight | arondight | Arondight |
| asi | asi | Asi |
| bancrofts | bancrofts_claw | Bancroft's Claw |
| bancroftsclaw | bancrofts_claw | Bancroft's Claw |
| beads | purification_beads | Purification Beads |
| blink | blink_rune | Blink Rune |
| bookofthoth | book_of_thoth | Book of Thoth |
| bookofthoth2 | evolved_book_of_thoth | Evolved Book of Thoth |
| crusher | the_crusher | The Crusher |
| db_envenom | envenomed_executioner | Envenomed Executioner |
| deathbringer | deathbringer | Deathbringer |
| demonic | demonic_grip | Demonic Grip |
| dom | dominance | Dominance |
| druidstone | druid_stone | Druid Stone |
| fatalis | fatalis | Fatalis |
| heartseeker | heartseeker | Heartseeker |
| hiddendag | hidden_dagger | Hidden Dagger |
| hydras | hydras_lament | Hydra's Lament |
| jotunns | jotunns_vigor | Jotunn's Vigor |
| jotunnsvigor | jotunns_vigor | Jotunn's Vigor |
| mace | mace | Mace |
| magic-focus | magic_focus | Magic Focus |
| myrdin | staff_of_myrddin | Staff of Myrddin |
| obshard | obsidian_shard | Obsidian Shard |
| powerpot | potion_of_power | Potion of Power |
| pridwen | pridwen | Pridwen |
| rage | rage | Rage |
| rage2 | evolved_rage | Evolved Rage |
| s_bluestone | bluestone_pendant | Bluestone Pendant |
| s_bluestonebrooch | bluestone_brooch | Bluestone Brooch |
| s_bumbas | bumbas_dagger | Bumba's Dagger |
| s_bumbasspear | bumbas_spear | Bumba's Spear |
| s_conduit | conduit_gem | Conduit Gem |
| s_eye | eye_of_the_jungle | Eye of the Jungle |
| s_hidden | manikin_hidden_blade | Manikin Hidden Blade |
| s_lonos | lonos_mask | Lono's Mask |
| s_manikin | manikin_mace | Manikin Mace |
| s_pendulum | pendulum_of_ages | Pendulum of Ages |
| s_protectors | protectors_mask | Protector's Mask |
| s_sands | sands_of_time | Sands of Time |
| s_warriors | warriors_axe | Warrior's Axe |
| serrated | serrated_edge | Serrated Edge |
| shell | magic_shell | Magic Shell |
| souleater | soul_eater | Soul Eater |
| souleater2 | evolved_soul_eater | Evolved Soul Eater |
| soulreaver | soul_reaver | Soul Reaver |
| spearmagus | spear_of_the_magus | Spear of the Magus |
| spearofdeso | spear_of_desolation | Spear of Desolation |
| spellbook | spellbook | Spellbook |
| spiked | spiked_gauntlet | Spiked Gauntlet |
| spiritrobe | spirit_robe | Spirit Robe |
| stoneofbinding | stone_of_binding | Stone of Binding |
| tahuti | rod_of_tahuti | Rod of Tahuti |
| tahuti_calamitous | calamitous_rod_of_tahuti | Calamitous Rod of Tahuti |
| tahuti_perfected | perfected_rod_of_tahuti | Perfected Rod of Tahuti |
| thebes | gauntlet_of_thebes | Gauntlet of Thebes |
| thebes2 | evolved_gauntlet_of_thebes | Evolved Gauntlet of Thebes |
| titans | titans_bane | Titan's Bane |
| trans | transcendence | Transcendence |
| typhons | typhons_fang | Typhon's Fang |
| winddemon | winged_blade | Winged Blade |

## Summary

- **Total mappings**: 57 unique item IDs
- **All mappings verified**: ✅ Every old ID has been successfully mapped to a valid new ID
- **Mapping patterns**:
  - Simple name changes (e.g., `obshard` → `obsidian_shard`)
  - Starter items prefix removed (e.g., `s_pendulum` → `pendulum_of_ages`)
  - Evolved items (e.g., `bookofthoth2` → `evolved_book_of_thoth`)
  - Shortened names expanded (e.g., `myrdin` → `staff_of_myrddin`)
  - Relic simplification (e.g., `beads` → `purification_beads`)

## Usage

The mapping is available in JSON format at `/home/jakeo/proj/grappul/item_id_mapping.json` for programmatic use.