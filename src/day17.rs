const INPUT: usize = 329;

struct CircularBuffer {
    // data is literally a map from idx -> succ(idx)
    data: Vec<usize>,
    pos: usize,
}

impl CircularBuffer {
    fn new(capacity: usize) -> CircularBuffer {
        let mut data = Vec::with_capacity(capacity);
        data.push(0);
        CircularBuffer { data, pos: 0 }
    }

    fn advance(&mut self, amt: usize) {
        for _ in 0..amt {
            self.pos = self.data[self.pos];
        }
    }

    fn extend(&mut self) {
        // this will be the next index
        let next = self.data.len();

        // this will be the successor of next (which will be the successor of pos)
        // that is, next will be inserted between pos and succ(pos)
        let next_next = self.data[self.pos];

        // self.data[next] = next_next
        self.data.push(next_next);

        self.data[self.pos] = next;

        // finally, set pos to the new index
        self.pos = next;
    }
}

fn run_17a_with_input(input: usize) -> usize {
    let mut buffer = CircularBuffer::new(2018);
    for _ in 0..2017 {
        buffer.advance(input);
        buffer.extend();
    }

    buffer.data[2017]
}

pub fn run_17a() -> usize {
    run_17a_with_input(INPUT)
}

fn run_17b_with_input(skip_size: usize, iterations: usize) -> usize {
    let mut buffer = CircularBuffer::new(iterations + 1);

    let start = std::time::Instant::now();
    let mut batch_start = std::time::Instant::now();

    let mut adv_time = 0.0;
    let mut ext_time = 0.0;

    for _ in 0..iterations {
        let now = start.elapsed().as_secs_f32();
        buffer.advance(skip_size);
        let adv = start.elapsed().as_secs_f32();
        buffer.extend();
        let ext = start.elapsed().as_secs_f32();

        adv_time = adv_time * 0.9 + (adv - now) * 0.1;
        ext_time = ext_time * 0.9 * (ext - adv) * 0.1;

        if buffer.data.len() % (1 << 17) == 0 {
            println!(
                "Iteration: {}; {:.3} sec",
                buffer.data.len(),
                start.elapsed().as_secs_f32()
            );
            println!("Batch time: {:.3} sec", batch_start.elapsed().as_secs_f32());
            batch_start = std::time::Instant::now();
            println!("Adv time {:.10} sec", adv_time);
            println!("Ext time {:.10} sec", ext_time);
            println!();
        }
    }

    buffer.data[0]
}

pub fn run_17b() -> usize {
    run_17b_with_input(INPUT, 50_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_17a() {
        assert_eq!(run_17a_with_input(3), 638);
    }

    #[test]
    fn sample_17b() {
        assert_eq!(run_17b_with_input(3, 1), 1);
        assert_eq!(run_17b_with_input(3, 2), 2);
        assert_eq!(run_17b_with_input(3, 3), 2);
        assert_eq!(run_17b_with_input(3, 4), 2);
        assert_eq!(run_17b_with_input(3, 5), 5);
        assert_eq!(run_17b_with_input(3, 6), 5);
        assert_eq!(run_17b_with_input(3, 7), 5);
        assert_eq!(run_17b_with_input(3, 8), 5);
        assert_eq!(run_17b_with_input(3, 9), 9);
    }
}
