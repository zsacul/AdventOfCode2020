
pub fn solve1(data:&Vec<String>)->i64
{
    0
}

pub fn solve2(data:&Vec<String>)->i64
{
    0
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)->(i64,i64)
{
    let res = (solve1(data),solve2(data));

    println!("Day18");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}


#[test]
fn test1()
{
    let v = vec!["1 + 2 * 3 + 4 * 5 + 6".to_string()];
    assert_eq!(solve1(&v),71);
}

#[test]
fn test2()
{
    let v = vec!["1 + (2 * 3) + (4 * (5 + 6))".to_string()];    
    assert_eq!(solve1(&v),51);
}
