#[derive(Debug, Clone)]
pub struct Base64String(String);

impl PartialEq for Base64String {
    fn eq(&self, other: &Self) -> bool {
        self.0.chars().filter(|&c| c != '=').collect::<String>()
            == other.0.chars().filter(|&c| c != '=').collect::<String>()
    }
}

impl core::fmt::Display for Base64String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
