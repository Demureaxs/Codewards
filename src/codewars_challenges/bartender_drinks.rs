pub fn run_bartender_drinks() {
    get_drink_by_profession("jabrOni");
    get_drink_by_profession("scHOOl counselor");
    get_drink_by_profession("prOgramMer");
    get_drink_by_profession("bike ganG member");
    get_drink_by_profession("poLiTiCian");
    get_drink_by_profession("rapper");
    get_drink_by_profession("pundit");
    get_drink_by_profession("Pug");
}

fn get_drink_by_profession(param: &str) -> &'static str {
    // using match as an expression to return the static string
    // based on matching the lowercase input
    match param.to_lowercase().as_str() {
        "jabroni" => "Patron Tequila",
        "school counselor" => "Anything with Alcohol",
        "programmer" => "Hipster Craft Beer",
        "bike gang member" => "Moonshine",
        "politician" => "Your tax dollars",
        "rapper" => "Cristal",
        _ => "Beer",
    }
}