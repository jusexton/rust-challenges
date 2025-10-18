struct ParkingSystem {
    spaces: [i32; 3],
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            spaces: [big, medium, small],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let i = car_type as usize - 1;
        self.spaces[i] -= 1;
        self.spaces[i] >= 0
    }
}
#[cfg(test)]
mod tests {
    use super::ParkingSystem;

    #[test]
    fn can_add_car_when_spaces_exist() {
        let mut parking = ParkingSystem::new(1, 1, 1);
        assert!(parking.add_car(1))
    }

    #[test]
    fn cant_add_car_when_spaces_do_not_exist() {
        let mut parking = ParkingSystem::new(0, 0, 0);
        assert!(!parking.add_car(1));
        assert!(!parking.add_car(2));
        assert!(!parking.add_car(3))
    }
}
