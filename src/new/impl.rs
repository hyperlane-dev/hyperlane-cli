use crate::*;

impl NewProjectConfig {
    /// Create a new project configuration with default template
    ///
    /// # Arguments
    /// - `project_name`: Name of the project
    ///
    /// # Returns
    /// - `NewProjectConfig`: Configuration instance
    pub(crate) fn new(project_name: String) -> Self {
        Self {
            project_name,
            template_url: "https://github.com/hyperlane-dev/hyperlane-quick-start".to_string(),
        }
    }
}
