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
        if test(target, &v).is_ok() {
            output += target;
        }
    }

    format!("{output}")
}

#[derive(Debug, Clone, Copy)]
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
        .find(|&x| evaluate(v, x).unwrap() == target)
        .ok_or(TestError::TargetNotFound)
}

fn evaluate(v: &[u64], ops: u64) -> Result<u64, TestError> {
    assert!(ops < 2_u64.pow(u32::try_from(v.len())?));
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

    Ok(output)
}

fn concatenate(x: u64, y: u64) -> Result<u64, TestError> {
    Ok(x * 10_u64.pow(u32::try_from(y.to_string().len())?) + y)
}

fn evaluate_2(v: &[u64], ops: u64) -> Result<u64, TestError> {
    assert!(ops < 3_u64.pow(u32::try_from(v.len())?));
    let mut ops = ops;
    let mut v = v.iter();
    let mut output = *v.next().unwrap();

    for num in v {
        if ops % 3 == 0 {
            output += num;
        } else if ops % 3 == 1 {
            output *= num;
        } else {
            output = concatenate(output, *num).unwrap();
        }
        ops /= 3;
    }
    Ok(output)
}

fn test_2(target: u64, v: &[u64]) -> Result<u64, TestError> {
    let mut op_max = 0..3_u64.pow(u32::try_from(v.len())?);
    op_max
        .find(|&x| evaluate_2(v, x).unwrap() == target)
        .ok_or(TestError::TargetNotFound)
}

pub fn run2(lines: &mut LinesIterator) -> String {
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
        if test_2(target, &v).is_ok() {
            output += target;
        }
    }

    format!("{output}")
}
