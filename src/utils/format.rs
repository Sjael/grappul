/// Convert god name from code format to image filename format
pub fn format_god_image_name(god_name: &str) -> String {
    match god_name {
        // Special cases that need manual mapping
        "chang_e" => "chang_e",
        "cu_chulainn" => "cu_chulainn",
        "he_bo" => "he_bo",
        "hou_yi" => "hou_yi",
        "hun_batz" => "hun_batz",
        "ne_zha" => "ne_zha",
        "nu_wa" => "nu_wa",
        "sun_wukong" => "sun_wukong",
        "xing_tian" => "xing_tian",
        "zhong_kui" => "zhong_kui",
        "ah_muzen_cab" => "ah_muzen_cab",
        "ah_puch" => "ah_puch",
        "ao_kuang" => "ao_kuang",
        "baba_yaga" => "baba_yaga",
        "baron_samedi" => "baron_samedi",
        "da_ji" => "da_ji",
        "erlang_shen" => "erlang_shen",
        "guan_yu" => "guan_yu",
        "king_arthur" => "king_arthur",
        "morgan_le_fay" => "morgan_le_fay",
        "pele" => "pele",
        "the_morrigan" => "the_morrigan",
        "yu_huang" => "yu_huang",
        _ => god_name
    }.to_string()
}