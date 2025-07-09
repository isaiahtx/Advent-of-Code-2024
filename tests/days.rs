use aoc::run_w_args;
use paste::paste;

fn assert_run(day: u8, part: u8, expected: &str) {
    let args = vec![
        "program".into(),
        day.to_string(),
        part.to_string(),
        format!("./inputs/input{day}.txt"),
    ];
    let result = run_w_args(&args);
    assert_eq!(result, expected);
}

macro_rules! test_day {
    ($day:literal, $part:literal, $expected:literal) => {
        paste! {
            #[test]
            fn [<test_day_ $day _part_ $part>]() {
                assert_run($day, $part, $expected);
            }
        }
    };
}

test_day!(1, 1, "2176849");
test_day!(1, 2, "23384288");
test_day!(2, 1, "402");
test_day!(2, 2, "455");
test_day!(3, 1, "171183089");
test_day!(3, 2, "63866497");
test_day!(4, 1, "2462");
test_day!(4, 2, "1877");
test_day!(5, 1, "5762");
test_day!(5, 2, "4130");
test_day!(6, 1, "4903");
test_day!(6, 2, "1911");
test_day!(7, 1, "1289579105366");
test_day!(7, 2, "92148721834692");
test_day!(8, 1, "285");
test_day!(8, 2, "944");
test_day!(9, 1, "6349606724455");
test_day!(9, 2, "6376648986651");
test_day!(10, 1, "719");
test_day!(10, 2, "1530");
test_day!(11, 1, "224529");
test_day!(11, 2, "266820198587914");
test_day!(12, 1, "1486324");
test_day!(12, 2, "898684");
test_day!(13, 1, "26299");
test_day!(13, 2, "107824497933339");
test_day!(14, 1, "228457125");
test_day!(14, 2, "6493");
test_day!(15, 1, "1438161");
test_day!(15, 2, "1437981");
test_day!(16, 1, "95444");
test_day!(16, 2, "513");
test_day!(17, 1, "6,0,6,3,0,2,3,1,6");
test_day!(17, 2, "236539226447469");
test_day!(18, 1, "252");
test_day!(18, 2, "5,60");
test_day!(19, 1, "242");
test_day!(19, 2, "595975512785325");
test_day!(20, 1, "1351");
test_day!(20, 2, "966130");
test_day!(21, 1, "105458");
