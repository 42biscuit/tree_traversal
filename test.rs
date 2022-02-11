const G: f32 = -9.81;
const TIME_STEP: f32 = 0.01;
pub const FORCE_APPLIED:f32 = 400.0;
#[allow(unused_variables, non_snake_case)]
#[derive(Clone)]
struct Pole {
    Angle: f32,
    Velocity: f32,
    Acceleration: f32,
    Mass: f32,
    Length: f32,
    _Failiaure_angle: f32,
}
#[derive(Clone)]
pub struct System {
    pole: Pole,
    cart: Cart,
}
impl System {
    pub fn new(length: f32, polemass: f32, cartmass: f32) -> Self {
        System {
            pole: Pole::new(length, polemass),
            cart: Cart::new(cartmass),
        }
    }
    pub fn update_vals(&mut self, force: &[f32]) -> Vec<f32> {
        self.pole.Acceleration += (G * self.pole.Angle.sin()
            + self.pole.Angle.cos()
                * ((-force[0]
                    * self.pole.Mass
                    * self.pole.Length
                    * self.pole.Velocity.powf(2.0)
                    * self.pole.Angle.sin())
                    / (self.pole.Mass + self.cart.Mass)))
            / (self.pole.Length
                * ((4.0 / 3.0)
                    - ((self.pole.Mass * self.pole.Angle.cos().powf(2.0))
                        / (self.pole.Mass + self.cart.Mass))));

        self.cart.Acceleration += (-force[0]
            + self.pole.Mass
                * self.pole_length()
                * (self.pole.Velocity.powf(2.0) * self.pole_angle().sin()
                    + self.pole.Acceleration * self.pole_angle().cos()))
            / (self.cart.Mass + self.pole.Mass);
        //self.cart.Velocity += self.cart.Acceleration * TIME_STEP;
        self.cart.Position += self.cart.Acceleration * TIME_STEP;
        self.pole.Angle += (self.pole.Acceleration + self.cart.Acceleration * -0.008) * TIME_STEP;
        //println!("{}", self.pole_angle());
        vec![
            self.pole_angle(),
            self.pole.Acceleration,
            self.cart.Acceleration,
            self.cart_pos()
        ]
    }
    pub fn pole_angle(&self) -> f32 {
        self.pole.Angle
    }
    pub fn pole_length(&self) -> f32 {
        self.pole.Length
    }
    pub fn cart_pos(&self) -> f32 {
        self.cart.Position
    }
    pub fn cart_acc(&self)->f32{
        self.cart.Acceleration
    }
    pub fn pole_acc(&self)->f32{
        self.pole.Acceleration
    }
    
}
impl Pole {
    pub fn new(lengh: f32, mass: f32) -> Self {
        Pole {
            Angle: 3.14, //1.5708
            Velocity: 0.0,
            Acceleration: 0.0,
            Mass: mass,
            Length: lengh,
            _Failiaure_angle: 0.2443,
        }
    }
}
#[allow(unused_variables, non_snake_case)]
#[derive(Clone)]
struct Cart {
    Mass: f32,
    _Velocity: f32,
    Acceleration: f32,
    Position: f32, //measured as the distance of the cart from the centre of the track
}
impl Cart {
    pub fn new(mass: f32) -> Self {
        Cart {
            Mass: mass,
            _Velocity: 0.0,
            Acceleration: 0.0,
            Position: 0.0,
        }
    }
}
