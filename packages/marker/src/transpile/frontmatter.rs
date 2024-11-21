
pub fn frontmatter_to_kv(frontmatter: String, delim: String) -> Vec<(String, String)> {

    let mut inside = false;

    let mut entries= Vec::new();

    for line in frontmatter.clone().lines() {
        if line.trim() == delim {
            inside = !inside;
            continue;
        }
        if !inside {
            continue;
        }
        if !line.trim().is_empty() && let Some((k, v)) =line
            .split_once(":")
            .map(|(k, v)| (k.trim(), v.trim()))
        {
            entries.push((k.to_string(), v.to_string()));
        }
    }

    return entries;

}
