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
        let three_quotes = "\"\"\"";
        let content =
            if self.fixture.json.contains(three_quotes) || self.fixture.source.contains(three_quotes) {
                toml::to_string_pretty(&self.fixture).unwrap()
            } else {
                format!(
                    r##"source = """
{}
"""

json = """
{}
"""
"##,
                    &self.fixture.source.trim_end(), &self.fixture.json
                )
            };
        std::fs::write(&self.file, content)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fixture {
    pub source: String,
    pub json: String,
}

impl Fixture {
    fn from(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }
}
