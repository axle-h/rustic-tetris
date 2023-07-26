use std::collections::HashMap;
use std::time::Duration;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use crate::particles::geometry::RectF;
use crate::particles::meta::ParticleSprite;
use crate::particles::Particles;
use crate::particles::scale::Scale;
use crate::particles::source::ParticleSource;
use strum::IntoEnumIterator;

const BASE_SCALE: f64 = 0.05;

pub struct ParticleRender<'a> {
    scale: Scale,
    sprites: Texture<'a>,
    sprite_snips: HashMap<ParticleSprite, Rect>,
    particles: Particles,
}

impl<'a> ParticleRender<'a> {
    pub fn new(particles: Particles, texture_creator: &'a TextureCreator<WindowContext>, scale: Scale) -> Result<Self, String> {
        let mut sprites = texture_creator.load_texture("resource/particle/sprites.png")?;
        sprites.set_blend_mode(BlendMode::Blend);

        let sprite_snips = ParticleSprite::iter()
            .map(|s| (s, s.snip()))
            .collect();

        Ok(Self { scale, particles, sprites, sprite_snips })
    }

    pub fn add_source(&mut self, source: Box<dyn ParticleSource>) {
        self.particles.sources.push(source);
    }

    pub fn update(&mut self, delta: Duration) {
        self.particles.update(delta)
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        for particle in self.particles.particles() {

            let (r, g, b): (u8, u8, u8) = particle.color().into();
            self.sprites.set_color_mod(r, g, b);
            if particle.alpha() < 1.0 {
                self.sprites.set_alpha_mod((255.0 * particle.alpha()).round() as u8);
            } else {
                self.sprites.set_alpha_mod(255);
            }

            let point = self.scale.point_to_render_space(particle.position());
            let snip = self.sprite_snips.get(&particle.sprite()).unwrap();
            let scale = BASE_SCALE * particle.size();
            let rect = Rect::from_center(
                point,
                (scale * snip.width() as f64).round() as u32,
                (scale * snip.height() as f64).round() as u32
            );
            canvas.copy(&self.sprites, *snip, rect)?;
        }
        Ok(())
    }
}

