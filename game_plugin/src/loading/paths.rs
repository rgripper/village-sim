pub struct AssetPaths {
    pub fira_sans: &'static str,
    pub audio_birds: &'static str,
    pub texture_tree: &'static str,
    pub texture_man: &'static str,
    pub texture_grad_shadow: &'static str,
}

pub const PATHS: AssetPaths = AssetPaths {
    fira_sans: "fonts/FiraSans-Bold.ttf",
    audio_birds: "audio/birds-1.ogg",
    texture_tree: "textures/tree.png",
    texture_man: "textures/man.png",
    texture_grad_shadow: "textures/grad_shadow.png",
};
