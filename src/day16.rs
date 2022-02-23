fn read_bits(hex: &[u8], bit_caret: &mut usize, bit_len: usize) -> u64 {
    let hex_start = *bit_caret / 4;
    *bit_caret += bit_len;
    let hex_end = (*bit_caret + 3) / 4;
    let mut num = 0;
    for d in &hex[hex_start..hex_end] {
        let half_byte = (d & 0xf) + (d >> 6) + ((d >> 6) << 3);
        num = num << 4 | half_byte as u64;
    }
    let trim_len = 4 * hex_end - *bit_caret;
    (num >> trim_len) & ((1 << bit_len) - 1)
}

pub fn part1(input: &str) -> u64 {
    let hex = &input.as_bytes()[..input.len() - 1];
    let mut sum = 0;
    let mut caret = 0;
    while caret + 6 < 4 * hex.len() {
        sum += read_bits(hex, &mut caret, 3);
        if read_bits(hex, &mut caret, 3) == 4 {
            let mut group = 0b10000;
            while group >> 4 == 1 {
                group = read_bits(hex, &mut caret, 5);
            }
        } else {
            let len_type = read_bits(hex, &mut caret, 1);
            caret += if len_type == 0 { 15 } else { 11 };
        }
    }
    sum
}

fn eval(hex: &[u8], caret: &mut usize) -> u64 {
    *caret += 3;
    let packet_type = read_bits(hex, caret, 3);
    if packet_type == 4 {
        let mut num = 0;
        let mut group = 0b10000;
        while group >> 4 == 1 {
            group = read_bits(hex, caret, 5);
            num = num << 4 | (group & 0xf);
        }
        return num;
    }
    let len_type = read_bits(hex, caret, 1);
    let sub_packet_limit = if len_type == 0 {
        read_bits(hex, caret, 15)
    } else {
        read_bits(hex, caret, 11)
    } as _;
    let sub_packet_bit_start = *caret;
    let mut sub_packet_count = 1;
    let mut num = eval(hex, caret);
    loop {
        if len_type == 0 && *caret - sub_packet_bit_start >= sub_packet_limit {
            break;
        }
        if len_type == 1 && sub_packet_count >= sub_packet_limit {
            break;
        }
        let next_num = eval(hex, caret);
        sub_packet_count += 1;
        num = match packet_type {
            0 => num + next_num,
            1 => num * next_num,
            2 => num.min(next_num),
            3 => num.max(next_num),
            5 => (num > next_num) as _,
            6 => (num < next_num) as _,
            7 => (num == next_num) as _,
            _ => panic!("invalid input"),
        };
    }
    num
}

pub fn part2(input: &str) -> u64 {
    eval(input.as_bytes(), &mut 0)
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day16.txt").unwrap()),
        913
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day16.txt").unwrap()),
        1510977819698
    );
}
