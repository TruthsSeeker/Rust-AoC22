fn main() {
    println!("Hello, world!");
}

// a distance of of > 2.0_f32.sqrt() requires the tail to move because 
// that is the distance between two points one diagonal move away from each other
fn calculate_distance(head: &(i32, i32), tail: &(i32, i32)) -> f32 {
    let (x_head, y_head) = head;
    let (x_tail, y_tail) = tail;
    (((x_head - x_tail) as f32).powf(2.0) + ((y_head - y_tail) as f32).powf(2.0)).sqrt()
}

fn is_tail_move_required(distance: f32) -> bool {
    distance > 2.0_f32.sqrt()
}

fn move_tail(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    let (x_head, y_head) = head;
    let (x_tail, y_tail) = tail;
    let x_diff = x_head - x_tail;
    let y_diff = y_head - y_tail;
    
    let mut x_tail_new = *x_tail;
    if x_diff > 0 {
        x_tail_new += 1;
    } else if x_diff < 0 {
        x_tail_new -= 1;
    }

    let mut y_tail_new = *y_tail;
    if y_diff > 0 {
        y_tail_new += 1;
    } else if y_diff < 0 {
        y_tail_new -= 1;
    }
    (x_tail_new, y_tail_new)
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

    #[test]
    fn test_move_tail() {
        let head = (0, 0);
        let tail = (2, 1);
        let tail_new = move_tail(&head, &tail);
        assert_eq!(tail_new, (1, 0));
        
        let head = (0, 0);
        let tail = (0, 2);
        let tail_new = move_tail(&head, &tail);
        assert_eq!(tail_new, (0, 1));

        let head = (0, 0);
        let tail = (-2, 0);
        let tail_new = move_tail(&head, &tail);
        assert_eq!(tail_new, (-1, 0));
    }
}