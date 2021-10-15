use geng::prelude::*;

mod game;

#[derive(geng::Assets)]
pub struct Assets {
    sprites: Sprites,
}

#[derive(geng::Assets)]
struct Sprites {
    skeleton: ugli::Texture,
}

impl Sprites {
    fn init(&mut self) {
        self.skeleton.set_filter(ugli::Filter::Nearest);
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
    let geng = Geng::new("Unstable Asteroids");
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
