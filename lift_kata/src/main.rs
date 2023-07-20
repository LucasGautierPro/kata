#[derive(Debug, PartialEq, Clone)]
struct Lift {
    floor_number: u16,
    position: u16,
    action: Action
}

#[derive(Debug, PartialEq, Clone)]
enum Action {
    StayingInPlace,
    OpeningDoors
}


impl Lift {
    fn new(floor_number: u16, position: u16, action: Action) -> Self {
        return Lift {
            floor_number,
            position,
            action,
        }
     }

    fn update(&self, queries: &[u16]) -> Self {
        let mut next = self.clone();

        if queries.len() > 0 {
            next.action = Action::OpeningDoors;
        }
        next
    }
}


fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use crate::{Action, Lift};


    #[test]
    fn lift_stay_still_when_nothing_happens() {
        let initial = Lift::new(10, 0, Action::StayingInPlace);
        let actual = initial.update(&[]);
        let expected = Lift::new(10, 0, Action::StayingInPlace);
        assert_eq!(actual, expected);
    }

    #[test]
    fn lift_open_door_when_called_on_its_current_floor() {
        let initial = Lift::new(10, 0, Action::StayingInPlace);
        let actual = initial.update(&[0]);
        let expected = Lift::new(10, 0, Action::OpeningDoors);
        assert_eq!(actual, expected);
    }
}