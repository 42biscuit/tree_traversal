mod node;
mod test;
mod tree;
use ggez::*;
use test::{System,FORCE_APPLIED};
use tree::{Tree,trainer, TreeBuilder};
#[allow(overflowing_literals)]
fn main() {
    let base = TreeBuilder::new(4, 2, 0,0 );
    let test_sys = System::new(100.0, 0.0010, 2.0);
    let mut trainer = trainer::new(
        base,
        300,
        400,
        0.2,
        System::update_vals,
        test_sys,
        (4.81, 2.01),
    );
    let best = trainer.train();

    let state = Display::new(best,System::new(100.0, 0.0010, 2.0));
    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .default_conf(c)
        .build()
        .unwrap();
    event::run(ctx, event_loop, state);
}
struct Display{
    sys:System,
    net:Tree,
}
impl Display{
    fn new(net:Tree,sys:System)->Self{
        Display{
            sys,
            net,
        }
    }
}

impl ggez::event::EventHandler<GameError> for Display {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let res = self.net.run_through_bf(&[self.sys.pole_angle(),self.sys.pole_acc(),self.sys.cart_acc(),self.sys.cart_pos()]);
        if res[0] >= res[1]{
            self.sys.update_vals(&[FORCE_APPLIED]);
        } else{
            self.sys.update_vals(&[-FORCE_APPLIED]);
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::WHITE);
        match graphics::Mesh::new_line(
            ctx,
            &[
                mint::Point2 {
                    x: self.sys.cart_pos()
                        + 300.0
                        + (self.sys.pole_angle() + 1.5708).cos() * self.sys.pole_length(),
                    y: 300.0 + (self.sys.pole_angle() + 1.5708).sin() * self.sys.pole_length(),
                },
                mint::Point2 {
                    x: self.sys.cart_pos() + 300.0,
                    y: 300.0,
                },
            ],
            2.0,
            graphics::Color::BLACK,
        ) {
            Ok(pole) => graphics::draw(ctx, &pole, graphics::DrawParam::default())?,
            Err(e) => {
                eprint!("error {}", &e)
            }
        }
        //std::hread::sleep_ms(100);

        graphics::present(ctx)?;
        Ok(())
    }
}
