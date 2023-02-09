use renderer::{DonutModel, translate, rotate_by_z, scale, VectorSpace};
use ladus::*;

pub fn render_buffer(radius: f32, step: f32) -> VectorSpace {
  let points = DonutModel { radius, inner_radius: 20. }
    .circle(step)
    .into_iter();
    //.map(|v| translate(10., 10., 0.) * v.into_vec().with_nth(3, 1.))
    //.map(|v| (rotate_by_z(angel.to_radians()) * v).round())
    //.map(|v| (scale(1., 2.2, 1.) * v).round());
  let space = VectorSpace(points.collect());

  space
}

pub fn render_content<const R: usize, const C: usize>(buffer: [[bool; C]; R]) -> String {
  let mut content = String::new();
  for row in buffer.into_iter() {
    for is_point in row.into_iter() {
      if is_point {
        content.push('@');
      } else {
        content.push(' ');
      }
    }
    content.push('\n');
  }

  content
}

fn main() {
  let radius: f32 = 10.0;
  let step: f32 = 0.5;
  let mut angel: f32 = 0.;
  loop {
    let buffer = render_buffer(radius, step)
      .map(|v| rotate_by_z(angel.to_radians()) * v)
      .map(|v| translate(10., 10., 10.) * v.into_vec().with_nth(3, 1.))
      .map(|v| scale(1., 2.2, 1.) * v)
      .map(|v| v.round())
      .into_buffer::<40, 110>();
    let content = render_content(buffer);
    print!("{}", content);
    angel += step
  }
}
