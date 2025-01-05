use crate::utils::LinesIterator;

pub fn run1(lines: &mut LinesIterator) -> String {
    let data = lines
        .map(Result::unwrap)
        .map(|x| x.split(": ").map(String::from).collect::<Vec<String>>())
        .map(|x| {
            (
                x[0].parse::<u64>().unwrap(),
                x[1].split(' ')
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            )
        });

    let mut output = 0;

    for (target, v) in data {
        // println!("testing target: {target}, with vector: {v:?}");
        if test(target, &v).is_ok() {
            output += target;
            // println!("\tsuccess! {x:0b}\n");
        } else {
            // println!("\tfail!\n");
        }
    }

    format!("{output}")
}

enum TestError {
    ConversionError(std::num::TryFromIntError),
    TargetNotFound,
}

impl From<std::num::TryFromIntError> for TestError {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self::ConversionError(err)
    }
}

fn test(target: u64, v: &[u64]) -> Result<u64, TestError> {
    let mut op_max = 0..2_u64.pow(u32::try_from(v.len())?);
    op_max
        .find(|&x| evaluate(v, x) == target)
        .ok_or(TestError::TargetNotFound)
}

fn evaluate(v: &[u64], ops: u64) -> u64 {
    assert!(ops < 2_u64.pow(v.len() as u32));
    let mut ops = ops;

    let mut v = v.iter();
    let mut output = *v.next().unwrap();

    for num in v {
        if ops & 1 == 0 {
            output += num;
        } else {
            output *= num;
        }
        ops >>= 1;
    }

    output
}

pub fn run2(lines: &mut LinesIterator) -> String {
    lines.next();
    format!("{lines:?}")
}
