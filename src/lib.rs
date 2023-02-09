use ladus::*;

pub fn rotate_by_z(angel: f32) -> Mat3x3<f32> {
  [
    [angel.cos(), angel.sin(), 0.],
    [-angel.sin(), angel.cos(), 0.],
    [0., 0., 1.],
  ].into_matrix()
}

pub fn rotate_by_y(angel: f32) -> Mat3x3<f32> {
  [
    [angel.sin(), 0., -angel.cos()],
    [0., 1., 0.],
    [angel.cos(), 0., angel.sin()],
  ].into_matrix()
}

pub fn rotate_by_x(angel: f32) -> Mat3x3<f32> {
  [
    [1., 0., 0.],
    [0., angel.sin(), angel.cos()],
    [0., angel.cos(), -angel.sin()],
  ].into_matrix()
}

pub fn translate(tx: f32, ty: f32, tz: f32) -> Mat3x4<f32> {
  [
    [1., 0., 0., tx],
    [0., 1., 0., ty],
    [0., 0., 1., tz],
  ].into_matrix()
}

pub fn scale(sx: f32, sy: f32, sz: f32) -> Mat3x3<f32> {
  [
    [sx, 0., 0.],
    [0., sy, 0.],
    [0., 0., sz],
  ].into_matrix()
}

pub struct DonutModel {
  pub radius: f32,
  pub inner_radius: f32,
}

impl DonutModel {
  pub fn point(&self, angel: f32) -> Vec3<f32> {
    [self.radius * angel.sin(), self.radius * angel.cos(), 0.].into_vec()
  }

  pub fn circle(&self, step: f32) -> Vec<Vec3<f32>> {
    let mut angel: f32 = 0.;
    let mut points = Vec::new();
    while angel <= 360. {
      let point = self.point(angel).round();
      if !points.contains(&point) {
        points.push(point);
      }
      points.dedup();
      angel += step;
    }

    points
  }

//  pub fn donut(&self, step: f32) -> Vec<Vec3<f32>> {
//    let original_circle = self.circle(step)
//      .into_iter()
//      .map(|v| translate(self.inner_radius, 0., 0.) * v.into_vec());
//  }
}

#[derive(Debug, Clone)]
pub struct VectorSpace(pub Vec<Vec3<f32>>);
impl VectorSpace {
  pub fn add_points(&mut self, points: impl Iterator<Item = Vec3<f32>>) {
    self.0.append(&mut points.collect());
  }

  pub fn map(self, mapper: impl Fn(Vec3<f32>) -> Vec3<f32>) -> Self {
    VectorSpace(self.0.into_iter().map(|v| mapper(v.clone())).collect())
  }

  pub fn into_buffer<const R: usize, const C: usize>(self) -> [[bool; C]; R] {
    let mut buffer = [[false; C]; R];

    for point in self.0.into_iter() {
      if point.x() > R as f32 || point.y() > C as f32 {
        break;
      } else {
        buffer[point.x() as usize][point.y() as usize] = true;
      }
    }

    buffer
  }
}

#[test]
fn test() {
  let vector = [4., 4., 0., 1.].into_vec();
  let translate = translate(1., 1., 1.);

  assert_eq!(translate * vector, [5., 5., 1.].into_vec());
}
