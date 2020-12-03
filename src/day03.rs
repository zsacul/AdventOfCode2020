fn count_trees(data:&Vec<String>,slope_x:usize,slope_y:usize)->i64
{
    let mut res = 0i64;
    let mut x = 0usize;
    let mut y = 0usize;
    let w = data[0].len();
    let h = data.len();

    while y<h {
        if data[y].chars().nth(x).unwrap()=='#' { res+=1; }
        x =(x+slope_x)%w;
        y+=   slope_y;
    }

    res
}

pub fn part1(data:&Vec<String>)->i64
{
    count_trees(data,3,1)
}

pub fn part2(data:&Vec<String>)->i64
{
    let cnt1 = count_trees(data,1,1);
    let cnt2 = count_trees(data,3,1);
    let cnt3 = count_trees(data,5,1);
    let cnt4 = count_trees(data,7,1);
    let cnt5 = count_trees(data,1,2);
    cnt1*cnt2*cnt3*cnt4*cnt5
}

pub fn solve(data:&Vec<String>)
{
    println!("Day3");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "..##.......".to_string(),
        "#...#...#..".to_string(),
        ".#....#..#.".to_string(),
        "..#.#...#.#".to_string(),
        ".#...##..#.".to_string(),
        "..#.##.....".to_string(),
        ".#.#.#....#".to_string(),
        ".#........#".to_string(),
        "#.##...#...".to_string(),
        "#...##....#".to_string(),
        ".#..#...#.#".to_string(),
    ];
    assert_eq!(part1(&v),7);
}

#[test]
fn test2()
{
    let v = vec![
        "..##.......".to_string(),
        "#...#...#..".to_string(),
        ".#....#..#.".to_string(),
        "..#.#...#.#".to_string(),
        ".#...##..#.".to_string(),
        "..#.##.....".to_string(),
        ".#.#.#....#".to_string(),
        ".#........#".to_string(),
        "#.##...#...".to_string(),
        "#...##....#".to_string(),
        ".#..#...#.#".to_string(),
    ];
    assert_eq!(part2(&v),336);
}
