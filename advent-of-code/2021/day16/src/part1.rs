fn main() {
    println!("{}", solve(include_str!("../input.txt")));
}

fn solve(input: &str) -> usize {
    let arr = input
        .chars()
        .flat_map(|ch| {
            match ch {
                '0' => "0000",
                '1' => "0001",
                '2' => "0010",
                '3' => "0011",
                '4' => "0100",
                '5' => "0101",
                '6' => "0110",
                '7' => "0111",
                '8' => "1000",
                '9' => "1001",
                'A' => "1010",
                'B' => "1011",
                'C' => "1100",
                'D' => "1101",
                'E' => "1110",
                'F' => "1111",
                _ => unreachable!(),
            }
            .bytes()
            .map(|b| b - b'0')
        })
        .collect::<Vec<_>>();

    find_version_sum(&mut Bits::new(&arr))
}

#[allow(clippy::collapsible_else_if)]
fn find_version_sum(bits: &mut Bits) -> usize {
    let mut version = bits.take_num(3);
    if bits.take_num(3) == 4 {
        bits.take_literal();
    } else {
        if bits.take_num(1) == 0 {
            let len = bits.take_num(15);
            let mut sub_bits = bits.consume_sub_bits(len);
            while !sub_bits.is_empty() {
                version += find_version_sum(&mut sub_bits);
            }
        } else {
            let len = bits.take_num(11);
            for _ in 0..len {
                version += find_version_sum(bits);
            }
        }
    }
    version
}

struct Bits<'a> {
    arr: &'a [u8],
    pos: usize,
}

impl<'a> Bits<'a> {
    fn new(arr: &'a [u8]) -> Self {
        Bits { arr, pos: 0 }
    }

    fn is_empty(&self) -> bool {
        self.pos >= self.arr.len()
    }

    fn pop_bit(&mut self) -> u8 {
        self.pos += 1;
        self.arr[self.pos - 1]
    }

    fn take_num(&mut self, bit_size: usize) -> usize {
        let mut val = 0;
        for _ in 0..bit_size {
            val = val << 1 | self.pop_bit() as usize;
        }
        val
    }

    fn take_literal(&mut self) -> usize {
        let mut val = 0;
        loop {
            let flag = self.pop_bit();
            for _ in 0..4 {
                val = val << 1 | self.pop_bit() as usize;
            }
            if flag == 0 {
                break;
            }
        }
        val
    }

    fn consume_sub_bits(&mut self, bit_size: usize) -> Bits {
        self.pos += bit_size;
        Bits {
            arr: &self.arr[(self.pos - bit_size)..self.pos],
            pos: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("8A004A801A8002F478"), 16);
        assert_eq!(solve("620080001611562C8802118E34"), 12);
        assert_eq!(solve("C0015000016115A2E0802F182340"), 23);
        assert_eq!(solve("A0016C880162017C3686B18A3D4780"), 31);
    }
}
