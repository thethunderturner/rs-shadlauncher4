const CATEGORIES: &[(&str, &str)] = &[
    ("ac", "Additional Content"),
    ("bd", "Blu-ray Disc?"),
    ("gc", "Game Content(?)"),
    ("gd", "Game Digital Application"),
    ("gda", "System Application"),
    ("gdb", "Unknown"),
    ("gdc", "Non-Game Big Application"),
    ("gdd", "BG Application"),
    ("gde", "Non-Game Mini App / Video Service Native App"),
    ("gdg", "CommonDialog"),
    ("gdk", "Video Service Web App"),
    ("gdl", "PS Cloud Beta App"),
    ("gdO", "PS2 Classic"),
    ("gp", "Game Application Patch"),
    ("gpc", "Non-Game Big App Patch"),
    ("gpd", "BG Application patch"),
    ("gpe", "Non-Game Mini App Patch / Video Service Native App Patch"),
    ("gpk", "Video Service Web App Patch"),
    ("gpl", "PS Cloud Beta App Patch"),
    ("sd", "Save Data"),
    ("la", "License Area (Vita)?"),
    ("wda", "Unknown"),
];

// https://www.psdevwiki.com/ps4/Param.sfo#CATEGORY
pub fn description(value: &str) -> Vec<String> {
    CATEGORIES
        .iter()
        .filter(|(code, _)| *code == value)
        .map(|(_, text)| (*text).to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::description;

    #[test]
    fn category_codes_are_matched_exactly() {
        assert_eq!(description("gd"), vec!["Game Digital Application"]);
        assert_eq!(description("gda"), vec!["System Application"]);
        assert_eq!(description("gdO"), vec!["PS2 Classic"]);
        assert!(description("gdo").is_empty());
        assert!(description("unknown").is_empty());
    }
}
