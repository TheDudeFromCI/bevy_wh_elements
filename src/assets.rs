use bevy::prelude::*;

pub struct AssetReference<'a> {
    server: &'a AssetServer,
    assets: Vec<UntypedHandle>,
}

impl<'a> AssetReference<'a> {
    pub fn new(server: &'a AssetServer) -> Self {
        Self {
            server,
            assets: Vec::new(),
        }
    }

    pub fn load<A: Asset>(&mut self, path: String) -> Handle<A> {
        let handle = self.server.load(path);
        self.assets.push(handle.clone().untyped());
        handle
    }

    pub fn get_handles(&self) -> &[UntypedHandle] {
        &self.assets
    }
}
