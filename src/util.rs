//! procmail model — mail filtering/local delivery agent.

#[derive(Debug, Clone)]
pub struct Recipe {
    pub name: String,
    pub conditions: Vec<String>,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub recipes: Vec<Recipe>,
}

impl Default for Model {
    fn default() -> Self { Self { recipes: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_recipe(&mut self, recipe: Recipe) { self.recipes.push(recipe); }

    pub fn list(&self) -> &[Recipe] { &self.recipes }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_recipe() {
        let mut p = Model::new();
        p.add_recipe(Recipe { name: "spam".into(), conditions: vec!["^Subject:.*viagra".into()], actions: vec!["file spam".into()] });
        assert_eq!(p.list().len(), 1);
    }
}
