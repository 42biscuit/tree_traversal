mod node;
mod test;
mod tree;

use ggez::{
    conf,
    event::{self, KeyCode},
    graphics, input, mint, Context, ContextBuilder, GameError, GameResult,
};
use test::{System, FORCE_APPLIED};
use tree::{trainer, Tree, TreeBuilder};
fn main() {
    let tree_builder = TreeBuilder::new(4, 2, 0, 1);
    let system = System::new(100.0, 0.001, 2.0);
    let mut trainer = trainer::new(
        tree_builder,
        300,
        300,
        0.2,
        System::update_vals,
        system,
        fail_crit,
    );
    let best = trainer.train();

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .default_conf(c)
        .build()
        .unwrap();
    event::run(
        ctx,
        event_loop,
        Display::new(best, System::new(100.0, 0.001, 2.0)),
    );
}

struct Display {
    net: Tree,
    system: System,
    cool: usize,
}
impl Display {
    fn new(best: Tree, sys: System) -> Self {
        Display {
            net: best,
            system: sys,
            cool: 0,
        }
    }
}

impl ggez::event::EventHandler<GameError> for Display {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.cool == 0 {
            if input::keyboard::is_key_pressed(ctx, KeyCode::A) {
                self.system.update_vals(&[1000.0]);
                self.cool = 100;

            }
            if input::keyboard::is_key_pressed(ctx, KeyCode::D) {
                self.system.update_vals(&[-1000.0]);
                self.cool = 100;
            }
        } else {
            self.cool -= 1;
        }

        let res = self.net.run_through_bf(&[
            self.system.pole_angle(),
            self.system.pole_acc(),
            self.system.cart_acc(),
            self.system.cart_pos(),
        ]);
        if res[0] > res[1] {
            self.system.update_vals(&[FORCE_APPLIED]);
        } else {
            self.system.update_vals(&[-FORCE_APPLIED]);
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::WHITE);
        match graphics::Mesh::new_line(
            ctx,
            &[
                mint::Point2 {
                    x: self.system.cart_pos()
                        + 300.0
                        + (self.system.pole_angle() + 1.57).cos() * self.system.pole_length(),
                    y: 300.0 + (self.system.pole_angle() + 1.57).sin() * self.system.pole_length(),
                },
                mint::Point2 {
                    x: self.system.cart_pos() + 300.0,
                    y: 300.0,
                },
            ],
            2.0,
            graphics::Color::BLACK,
        ) {
            Ok(mesh) => graphics::draw(ctx, &mesh, graphics::DrawParam::default())?,
            Err(e) => {
                eprintln!("{}", e)
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

fn fail_crit(inputs: &[f32]) -> bool {
    if inputs[0] > 1.57 && inputs[0] < 4.64 && inputs[1] > -200.0 && inputs[1] < 200.0 {
        false
    } else {
        true
    }
}
