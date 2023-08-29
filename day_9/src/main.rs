fn main() {
    println!("Hello, world!");
}

fn calculate_distance(head: &(i32, i32), tail: &(i32, i32)) -> f32 {
    let (x_head, y_head) = head;
    let (x_tail, y_tail) = tail;
    (((x_head - x_tail) as f32).powf(2.0) + ((y_head - y_tail) as f32).powf(2.0)).sqrt()
}

fn is_tail_move_required(distance: f32) -> bool {
    distance > 2.0_f32.sqrt()
}


#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        let head = (0, 0);
        let tail = (1, 1);
        let distance = calculate_distance(&head, &tail);
        assert_eq!(distance, 2.0_f32.sqrt());
    }

    #[test]
    fn test_is_tail_move_required() {
        let distance = 2.0_f32.sqrt();
        assert_eq!(is_tail_move_required(distance), false);
        let head = (0, 0);
        let tail = (2, 1);
        let distance = calculate_distance(&head, &tail);
        assert_eq!(is_tail_move_required(distance), true);
    }
}