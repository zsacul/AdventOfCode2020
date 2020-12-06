use std::collections::HashMap;

pub fn count(s:String,cnt:usize)->(usize,usize)
{
    let mut hash : HashMap<char,usize> = HashMap::new();

    for c in s.chars() {
        hash.insert(c, hash.get(&c).unwrap_or(&0)+1);
    }

    (hash.keys().len() , hash.values().filter(|&&s| s==cnt).count())
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)->(usize,usize)
{
    let mut res = (0,0);

    let mut acc="".to_string();
    let mut cnt=0;

    for line in data {
        if line.len()==0 {
            let c = count(acc,cnt);
            acc="".to_string();
            res.0+=c.0;
            res.1+=c.1;
            cnt=0;
        }
          else
        {
            acc.push_str(line);
            cnt+=1;
        }
    }

    if acc.len()>0 {
        let c = count(acc,cnt);
        res.0+=c.0;
        res.1+=c.1;
    }

    println!("Day5");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}

#[test]
fn test1()
{
    let v = vec![
        "abc".to_string(),
        "".to_string(),
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "".to_string(),
        "ab".to_string(),
        "ac".to_string(),
        "".to_string(),
        "a".to_string(),
        "a".to_string(),
        "a".to_string(),
        "a".to_string(),
        "".to_string(),
        "b".to_string()
    ];
    assert_eq!(solve(&v).0,11);
}

#[test]
fn test2()
{
    let v = vec![
        "abc".to_string(),
        "".to_string(),
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "".to_string(),
        "ab".to_string(),
        "ac".to_string(),
        "".to_string(),
        "a".to_string(),
        "a".to_string(),
        "a".to_string(),
        "a".to_string(),
        "".to_string(),
        "b".to_string()
    ];
    assert_eq!(solve(&v).1,6);
}
