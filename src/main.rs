use geng::prelude::*;

mod game;

type Texture = Rc<ugli::Texture>;

pub struct Sprite {
    texture: Texture,
    flipped: bool,
}

impl Sprite {
    fn new(texture: &Texture) -> Self {
        Self {
            texture: texture.clone(),
            flipped: false,
        }
    }
}

impl From<&Texture> for Sprite {
    fn from(texture: &Texture) -> Self {
        Self::new(texture)
    }
}

#[derive(geng::Assets)]
struct Assets {
    #[asset(path = "fonts/NF_pixels/fonts/ttf/NFPixels-Regular.ttf")]
    font: Font,
    sprites: Sprites,
}

#[derive(Deref)]
pub struct Font {
    #[deref]
    inner: Rc<geng::Font>,
}

impl geng::LoadAsset for Font {
    fn load(geng: &Geng, path: &str) -> geng::AssetFuture<Self> {
        let geng = geng.clone();
        <Vec<u8> as geng::LoadAsset>::load(&geng, path)
            .map(move |data| {
                Ok(Font {
                    inner: Rc::new(geng::Font::new(&geng, data?)?),
                })
            })
            .boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = Some("ttf");
}

#[derive(geng::Assets)]
struct Sprites {
    skeleton: Texture,
    necromancer: Texture,
    knight: Texture,
    castle: Texture,
    #[asset(path = "grave/*.png", range = "1..=3")]
    graves: Vec<Texture>,
}

macro_rules! sprites_init {
    ($($texture:expr),*) => {
        $(
            Rc::get_mut(&mut $texture).unwrap().set_filter(ugli::Filter::Nearest);
        )*
    };
}

impl Sprites {
    fn init(&mut self) {
        sprites_init!(self.skeleton, self.necromancer, self.knight, self.castle);
        for grave in &mut self.graves {
            Rc::get_mut(grave)
                .unwrap()
                .set_filter(ugli::Filter::Nearest);
        }
    }
}

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    // Setup working directory
    if let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        std::env::set_current_dir(std::path::Path::new(&dir).join("static")).unwrap();
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = std::env::current_exe().unwrap().parent() {
                std::env::set_current_dir(path).unwrap();
            }
        }
    }

    // Intialize geng
    let geng = Geng::new("Skelemancer");
    let assets = <Assets as geng::LoadAsset>::load(&geng, ".");

    // Run
    geng::run(
        &geng,
        geng::LoadingScreen::new(&geng, geng::EmptyLoadingScreen, assets, {
            let geng = geng.clone();
            move |assets| {
                let mut assets = assets.unwrap();
                assets.sprites.init();

                game::GameState::new(&geng, &Rc::new(assets))
            }
        }),
    );
}
