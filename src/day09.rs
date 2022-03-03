fn any_sum(data:&[i64],f:usize,t:usize,v:i64)->bool
{
    for i in f..t
    {
        for j in i+1..t
        {
            if data[i]+data[j]==v { return true; }
        }
    }
    false
}

fn find_span_sum(data:&[i64],v:i64)->i64
{
    for i in 0..data.len() {
        for j in i+1..data.len() {
            let span = data[i..j].to_vec();
            
            if span.iter().sum::<i64>()==v
            {
                let min_v:i64 = *span.iter().min().unwrap();
                let max_v:i64 = *span.iter().max().unwrap(); 
                return min_v + max_v;
            }
        }        
    }
    -1
}

#[allow(unused)]
pub fn solve(data:&[i64],preambles :usize)->(i64,i64)
{
    let mut res = (0,0);    

    for i in preambles+1..data.len()
    {
        if !any_sum(data,i-preambles,i,data[i])
        {
            res.0 = data[i];
            break;
        }
    }
    
    res.1 = find_span_sum(data,res.0);    

    println!("Day9");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}

#[test]
fn test1()
{
    let v = vec![
        35,
        20,
        15,
        25,
        47,
        40,
        62,
        55,
        65,
        95,
        102,
        117,
        150,
        182,
        127,
        219,
        299,
        277,
        309,
        576,        
    ];
    assert_eq!(solve(&v,5).0,127);
}


#[test]
fn test2()
{
    let v = vec![
        35,
        20,
        15,
        25,
        47,
        40,
        62,
        55,
        65,
        95,
        102,
        117,
        150,
        182,
        127,
        219,
        299,
        277,
        309,
        576,        
        ];    
    assert_eq!(solve(&v,5).1,62);
}
