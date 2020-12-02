pub fn part1(data:&Vec<i32>)
{
    for i in 0..data.len() {
        for j in i..data.len() {
            if data[i]+data[j]==2020 
            {
                println!("{}",data[i]*data[j]);
            }
        }
    }
}

pub fn part2(data:&Vec<i32>)
{
    for i in 0..data.len() {
        for j in i..data.len() {
            for k in j..data.len() {
                if data[i]+data[j]+data[k]==2020 
                {
                    println!("{}",data[i]*data[j]*data[k]);
                }
            }
        }
    }
}

pub fn solve(data:&Vec<i32>)
{    
    println!("Day1");
    part1(data);
    part2(data);
}
