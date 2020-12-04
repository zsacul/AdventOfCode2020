#[allow(unused)]
pub fn solve(data:&Vec<&str>)->(i32,i32)
{
    let mut part1 = 0;
    let mut part2 = 0;
    
    for s in data {
            let tab    : Vec<_> =        s.split(": ").collect(); 
            let v      : Vec<_> =   tab[0].split(" ").collect();        
            let letter : char   =     v[1].chars().nth(0).unwrap();
            let min_c  : Vec<_> =     v[0].split("-").collect();
            let min_v  : usize  = min_c[0].parse().unwrap();
            let max_v  : usize  = min_c[1].parse().unwrap();
    
            let cnt =  tab[1].chars().filter(|&c| c==letter).count();
    
            let char_a = tab[1].chars().nth(min_v-1).unwrap();
            let char_b = tab[1].chars().nth(max_v-1).unwrap();
    
            if cnt>=min_v && cnt<=max_v                             { part1+=1; }
            if char_a!=char_b && (char_a==letter || char_b==letter) { part2+=1; }
        }

        println!("Day2");
        println!("part1:{}",part1);
        println!("part2:{}",part2);

        (part1,part2)
}


#[test]
fn test1()
{
    let v = vec!["1-3 a: abcde",
                          "1-3 b: cdefg",
                          "2-9 c: ccccccccc"];
    assert_eq!(solve(&v).0,2);

}

#[test]
fn test2()
{
    let v = vec!["1-3 a: abcde",
                          "1-3 b: cdefg",
                          "2-9 c: ccccccccc"];
    assert_eq!(solve(&v).1,1);
}