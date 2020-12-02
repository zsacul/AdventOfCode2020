pub fn part1(data:&Vec<i32>)->i32
{
    for i in 0..data.len() {
        for j in i..data.len() {
            if data[i]+data[j]==2020 
            {
                return data[i]*data[j];
            }
        }
    }
    -1
}

pub fn part2(data:&Vec<i32>)->i32
{
    for i in 0..data.len() {
        for j in i..data.len() {
            for k in j..data.len() {
                if data[i]+data[j]+data[k]==2020 
                {
                    return data[i]*data[j]*data[k];                    
                }
            }
        }
    }
    -1
}

pub fn solve(data:&Vec<i32>)
{    
    println!("Day1");
    println!("{}",part1(data));
    println!("{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![1721,    979,    366,    299,    675,    1456];
    assert_eq!(part1(&v),514579);
}

#[test]
fn test2()
{
    let v = vec![1721,    979,    366,    299,    675,    1456];
    assert_eq!(part2(&v),241861950);
}
