/// The `clean_search_vec` function takes a vector of strings, cleans and filters the strings, and
/// returns a new vector. This is used to clean up search terms before they are used to search for
/// raws.
///
/// Arguments:
///
/// * `vec`: A vector of strings representing search terms.
///
/// Returns:
///
/// The function `clean_search_vec` returns a `Vec<String>`.
pub fn clean_search_vec(vec: &[String]) -> Vec<String> {
    let mut vec: Vec<String> = vec.join(" ").split_whitespace().map(String::from).collect();

    // Lowercase everything
    vec = vec.iter().map(|x| x.to_lowercase()).collect();

    // Remove any periods, commas, etc.
    vec = vec
        .iter()
        .map(|x| x.replace('.', ""))
        .map(|x| x.replace(',', ""))
        .map(|x| x.replace('(', ""))
        .map(|x| x.replace(')', ""))
        .map(|x| x.replace(';', ""))
        // ! This is dangerous, because it can obscure bad tag parsing.
        .map(|x| x.replace(':', " "))
        .collect();

    // Uniq the vec
    vec.sort();
    vec.dedup();

    // Remove some generic words
    vec.retain(|x| !x.eq_ignore_ascii_case("creature"));
    vec.retain(|x| !x.eq_ignore_ascii_case("all"));
    vec.retain(|x| !x.eq_ignore_ascii_case("the"));
    vec.retain(|x| !x.eq_ignore_ascii_case("of"));
    vec.retain(|x| !x.eq_ignore_ascii_case("in"));
    vec.retain(|x| !x.eq_ignore_ascii_case("and"));
    vec.retain(|x| !x.eq_ignore_ascii_case("a"));
    vec.retain(|x| !x.eq_ignore_ascii_case("an"));
    vec.retain(|x| !x.eq_ignore_ascii_case("with"));

    vec
}
