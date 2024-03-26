use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct FixtureFile {
    pub file: PathBuf,
    pub fixture: Fixture,
}

impl FixtureFile {
    pub fn from(
        file: &PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let content = std::fs::read_to_string(file)?;
        let fixture = Fixture::from(&content)?;
        Ok(Self {
            file: file.clone(),
            fixture,
        })
    }

    pub fn update(
        &mut self,
        fixture: &Fixture,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.fixture = fixture.clone();
        self.save()?;
        Ok(())
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        let content = self.fixture.to_toml_string_pretty();
        std::fs::write(&self.file, content)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fixture {
    source: String,
    json: String,
}

impl Fixture {
    pub fn new(source: &str, json: &str) -> Self {
        Self {
            source: source.trim().to_string(),
            json: json.trim().to_string(),
        }
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn json(&self) -> &str {
        &self.json
    }

    fn from(content: &str) -> Result<Self, toml::de::Error> {
        let ret: Self = toml::from_str(content)?;
        Ok(Self::new(&ret.source, &ret.json))
    }

    fn to_toml_string_pretty(&self) -> String {
        let three_quotes = "\"\"\"";
        if self.source.contains(three_quotes) || self.json.contains(three_quotes) {
            toml::to_string_pretty(&self).unwrap()
        } else {
            format!(
                r##"
source = """
{}
"""

json = """
{}
"""
"##,
                &self.source, &self.json
            )
            .trim_start()
            .to_string()
        }
    }
}
