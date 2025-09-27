pub struct ClientMod {
    pub name: String,
    pub script: String,
    pub styles: Option<String>,
}

pub struct ClientMods {
    pub mods: Vec<ClientMod>,
}

impl ClientMods {
    pub fn new() -> Self {
        ClientMods { mods: Vec::new() }
    }

    pub fn add_mod(&mut self, client_mod: ClientMod) {
        self.mods.push(client_mod);
    }

    pub fn into_injectables(self) -> Vec<crate::injection::inject::Injectable> {
        self.mods
            .into_iter()
            .map(|m| crate::injection::inject::Injectable::new(m.script, m.styles))
            .collect()
    }
}
