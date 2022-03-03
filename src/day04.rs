use std::collections::HashSet;

fn number_of_digits(s:&str)->usize 
{
    s.chars().filter(|&c| c.is_digit(10)).count()
}

fn number_of_hex(s:&str)->usize 
{
    s.chars().filter(|&c| c.is_digit(16)).count()
}

fn scan_n(s:&str)->i32
{
    let mut id=0;
    while s.chars().nth(id).unwrap_or(' ').is_digit(10) { id+=1; } 
    s[..id].parse().unwrap_or(0)
}

fn pass(code:&str,val:&str)->bool 
{
    let vali = scan_n(val);

    match code {
        "byr" => (1920..=2002).contains(&vali),
        "iyr" => (2010..=2020).contains(&vali),
        "eyr" => (2020..=2030).contains(&vali),
        "hgt" => (val.len()==5 && val.contains("cm") && number_of_digits(val)==3 && vali>=150 && vali<=193) ||
                 (val.len()==4 && val.contains("in") && number_of_digits(val)==2 && vali>= 59 && vali<= 76),
        "hcl" =>  val.len()==7 && val.starts_with('#') && number_of_hex(&val[1..])==6,        
        "ecl" =>  val=="amb" || val=="blu" || val=="brn" || val=="gry" || val=="grn" || val=="hzl" || val=="oth",
        "pid" =>  number_of_digits(val)==9,
        "cid" =>  true,
        _     =>  false,
    }
}

fn is_valid(v:&[&str])->(i32,i32) {

    let req = vec!["byr",
                            "iyr",
                            "eyr",
                            "hgt",
                            "hcl",
                            "ecl",
                            "pid"];
    
    let mut h : HashSet<&str> = HashSet::new();
    let mut r1 = 0;
    let mut r2 = 0;
    let mut p1_ok = true;
    let mut p2_ok = true;

    for p in v {
        let s:Vec<_> = p.split(':').collect();
        let code = s[0];

        if !h.contains(p) 
        {          
            if s.len()>1 && !pass(code,s[1]) 
            { 
                p2_ok = false;
            }
            else if req.contains(&code) { r2+=1; }            

            if req.contains(&code) { r1+=1; }
            h.insert(code);
        }
    }
    
    if r1!=req.len() { p1_ok = false; }
    if r2!=req.len() { p2_ok = false; }
    (p1_ok as i32,p2_ok as i32)
}

#[allow(unused)]
pub fn solve(data:&[String])->(i32,i32)
{
    let mut res = (0,0);
    let mut acc = "".to_string();

    for line in data
    {
        if line.is_empty() {
            let v: Vec<_> = acc.split(' ').collect();
            let r = is_valid(&v); 
            res.0+=r.0;
            res.1+=r.1;
             
            acc = "".to_string();   
        }
        else
        {
            acc.push(' ');
            acc.push_str(line);
        }

    }

    if !acc.is_empty()
    {
        let v: Vec<_> = acc.split(' ').collect();        
        let r = is_valid(&v); 
        res.0+=r.0;
        res.1+=r.1;
    }

    println!("Day4");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    res
}

#[test]
fn test1()
{
    let v = vec![
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
        "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
        "".to_string(),
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
        "hcl:#cfa07d byr:1929".to_string(),
        "".to_string(),
        "hcl:#ae17e1 iyr:2013".to_string(),
        "eyr:2024".to_string(),
        "ecl:brn pid:760753108 byr:1931".to_string(),
        "hgt:179cm".to_string(),
        "".to_string(),
        "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
        "iyr:2011 ecl:brn hgt:59in".to_string(),
    ];
    assert_eq!(solve(&v).0,2);
}

#[test]
fn test2()
{
    let v = vec![
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
        "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
        "".to_string(),
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
        "hcl:#cfa07d byr:1929".to_string(),
        "".to_string(),
        "hcl:#ae17e1 iyr:2013".to_string(),
        "eyr:2024".to_string(),
        "ecl:brn pid:760753108 byr:1931".to_string(),
        "hgt:179cm".to_string(),
        "".to_string(),
        "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
        "iyr:2011 ecl:brn hgt:59in".to_string(),
    ];
    assert_eq!(solve(&v).1,2);
}
