use std::f32::consts::PI;
use crate::food::Food;

static FOV_RANGE: f32 = 0.25;
static FOV_ANGLE: f32 = PI + std::f32::consts::FRAC_PI_2;
static CELL_COUNT: usize = 9;

pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cell_count: usize
}
impl Eye {
    pub fn new(fov_range: f32, fov_angle:f32, cell_count: usize) -> Self {
        Self {
            fov_range,
            fov_angle,
            cell_count
        }
    }

    pub fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELL_COUNT)
    }

    pub fn get_cell_count(&self) -> usize {
        self.cell_count
    }

    pub fn see_foods(&self, bird_position: nalgebra::Point2<f32>, bird_rotation: nalgebra::Rotation2<f32>, foods: &[Food]) -> Vec<f32> {
        let mut cell_list = vec![0.0; self.cell_count];

        for food in foods {
            let vector_from_eye_to_food = food.position - bird_position;
            let distance_between_eye_and_food = vector_from_eye_to_food.norm();

            if distance_between_eye_and_food > self.fov_range {
                continue;
            }

            let food_angle_y_based = nalgebra::Rotation2::rotation_between(&nalgebra::Vector2::y(), &vector_from_eye_to_food).angle();
            let angle_between_food_and_eye = nalgebra::wrap(food_angle_y_based - bird_rotation.angle(), -PI, PI) ;

            if angle_between_food_and_eye < -self.fov_range / 2.0 || angle_between_food_and_eye > self.fov_range / 2.0 {
                continue;
            }

            let normalized_angle_between_food_and_eye = angle_between_food_and_eye + self.fov_angle / 2.0;
            let percentage_in_fov_angle = normalized_angle_between_food_and_eye / self.fov_angle;

            let cell_index_in_cell_list = ((percentage_in_fov_angle * (self.cell_count as f32)) as usize)
                .min(self.cell_count - 1);
            let food_energy = (self.fov_range - distance_between_eye_and_food) / self.fov_range;
            cell_list[cell_index_in_cell_list] += food_energy;
        }

        cell_list
    }
}