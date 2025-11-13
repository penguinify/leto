use crate::window::App;

use crate::injection::shared::{self, InjectionError};

pub struct Injectable {
    script: String,         // link or path
    styles: Option<String>, // link or path
}

impl Injectable {
    pub fn new(script: String, styles: Option<String>) -> Self {
        Injectable { script, styles }
    }

    pub fn get_script(&self) -> &str {
        &self.script
    }

    pub fn get_styles(&self) -> &str {
        &self.styles.as_deref().unwrap_or("") // returns empty string if not defined
    }
}

pub fn wrap_css(styles: &str) -> String {
    format!(
        r#"
    (function() {{
        let style = document.createElement('style');
        style.type = 'text/css';
        style.innerHTML = `{}`;
        document.head.appendChild(style);
    }})();
    "#,
        styles
    )
}

pub fn inject_injectable(app: &mut App, injectable: Injectable) -> Result<(), InjectionError> {
    let script = if injectable.get_script().starts_with("http") {
        shared::load_url_into_string(injectable.get_script()).map_err(|e| {
            tracing::error!("Failed to load URL: {}", injectable.get_script());
            return e;
        })
    } else {
        shared::load_script_into_string(injectable.get_script()).map_err(|e| {
            tracing::error!("Failed to load script: {}", injectable.get_script());
            return e;
        })
    };

    let styles = if !injectable.get_styles().is_empty() {
        if injectable.get_styles().starts_with("http") {
            shared::load_url_into_string(injectable.get_styles()).map_err(|e| {
                tracing::error!("Failed to load URL: {}", injectable.get_styles());
                return e;
            })
        } else {
            shared::load_script_into_string(injectable.get_styles()).map_err(|e| {
                tracing::error!("Failed to load styles: {}", injectable.get_styles());
                return e;
            })
        }
    } else {
        Ok(String::new())
    };

    match (script, styles) {
        (Ok(script_content), Ok(styles_content)) => {
            let wrapped_styles = if !styles_content.is_empty() {
                wrap_css(&styles_content)
            } else {
                String::new()
            };

            let final_script = format!("{};\n{}", wrapped_styles, script_content);

            app.evaluate_script(&final_script).map_err(|e| {
                tracing::error!("Failed to inject script: {}", e);
                InjectionError::ScriptInjectionError
            })?;

            tracing::info!("Successfully injected script and styles.");
            Ok(())
        }
        (Err(e), _) => Err(e),
        (_, Err(e)) => Err(e),
    }
}
