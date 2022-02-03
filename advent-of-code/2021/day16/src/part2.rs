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

    evaluate_bits(&mut Bits::new(&arr))
}

#[allow(clippy::collapsible_else_if)]
fn evaluate_bits(bits: &mut Bits) -> usize {
    bits.take_num(3);
    let type_id = bits.take_num(3);

    let sub_packets = if type_id == 4 {
        vec![bits.take_literal()]
    } else {
        if bits.take_num(1) == 0 {
            let len = bits.take_num(15);
            let mut sub_bits = bits.consume_sub_bits(len);
            let mut arr = vec![];
            while !sub_bits.is_empty() {
                arr.push(evaluate_bits(&mut sub_bits));
            }
            arr
        } else {
            let len = bits.take_num(11);
            let mut arr = vec![];
            for _ in 0..len {
                arr.push(evaluate_bits(bits));
            }
            arr
        }
    };

    match type_id {
        0 => sub_packets.iter().sum(),
        1 => sub_packets.iter().product(),
        2 => *sub_packets.iter().min().unwrap(),
        3 => *sub_packets.iter().max().unwrap(),
        4 => sub_packets[0],
        5 => (sub_packets[0] > sub_packets[1]) as usize,
        6 => (sub_packets[0] < sub_packets[1]) as usize,
        7 => (sub_packets[0] == sub_packets[1]) as usize,
        _ => unreachable!(),
    }
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
        assert_eq!(solve("C200B40A82"), 3);
        assert_eq!(solve("04005AC33890"), 54);
        assert_eq!(solve("880086C3E88112"), 7);
        assert_eq!(solve("CE00C43D881120"), 9);
        assert_eq!(solve("D8005AC2A8F0"), 1);
        assert_eq!(solve("F600BC2D8F"), 0);
        assert_eq!(solve("9C005AC2F8F0"), 0);
        assert_eq!(solve("9C0141080250320F1802104A08"), 1);
    }
}
