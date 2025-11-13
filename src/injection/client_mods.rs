use crate::injection::inject::Injectable;

pub struct ClientMod {
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

    pub fn into_injectables(self) -> Vec<Injectable> {
        self.mods
            .into_iter()
            .map(|m| Injectable::new(m.script, m.styles))
            .collect()
    }
}
