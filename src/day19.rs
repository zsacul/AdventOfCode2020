use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    a1:i32,
    b1:i32,
    c1:i32,
    a2:i32,
    b2:i32,
    c2:i32,
    //id:i32,
    ch:Option<char>,
}

impl Rule
{
    fn new(s:String)->Self
    {
        let mut a1=-1;
        let mut b1=-1;
        let mut c1=-1;
        let mut a2=-1;
        let mut b2=-1;
        let mut c2=-1;        
        let mut ch = None;

        if s.find('"')!=None 
        { 
            ch = Some(s.chars().nth(1).unwrap()) 
        } 
          else 
        {
            let ss : Vec<_> = s.split(" | ").collect();
            let ss1 : Vec<_> = ss[0].split(' ').collect();
            
            //println!("ss:{:?}",ss);
            if !ss1.is_empty() { a1 = ss1[0].to_string().parse::<i32>().unwrap(); }
            if ss1.len()>=2 { b1 = ss1[1].to_string().parse::<i32>().unwrap(); }
            if ss1.len()>=3 { c1 = ss1[2].to_string().parse::<i32>().unwrap(); }
            if ss1.len()>3 {panic!("tm {} \n{:?}",s,ss)};

            if ss.len()>1
            {
                let ss2 : Vec<_> = ss[1].split(' ').collect();
                if !ss2.is_empty() { a2 = ss2[0].to_string().parse::<i32>().unwrap(); }
                if ss2.len()>=2 { b2 = ss2[1].to_string().parse::<i32>().unwrap(); }
                if ss2.len()>=3 { c2 = ss2[2].to_string().parse::<i32>().unwrap(); }
                if ss2.len()>3 {panic!("tm {} \n{:?}",s,ss)};
            }
        }

        Rule {
//            a1,b1,c1,a2,b2,c2,id,ch
            a1,b1,c1,a2,b2,c2,ch
        }
    }

    #[allow(unused)]
    fn print(&self)
    {
        println!("l:{:?} a1:{} b1:{} c1:{} a2:{} b2:{} c2:{}",self.ch,self.a1,self.b1,self.c1,self.a2,self.b2,self.c2);
    }

    fn match_string(&self,hash:&HashMap<i32,Rule>,s:&str,i:usize)->(bool,usize)
    {
        //print!("{},",self.id);

        if self.ch!=None {
            
            if self.ch.unwrap()==s.chars().nth(i).unwrap_or(' ')
            {
                return (true,i+1);
            }
            else
            {
                return (false,i);
            }
        }
        if i==s.len() { return (true,s.len()); } 

        if self.a1!=-1 
        {
            let m1 = hash.get(&self.a1).unwrap().match_string(hash, s, i);            

            if m1.0 { 
                
                if self.b1==-1 
                { 
                    return m1; 
                }
                if m1.1>=s.len() { return (false,i); }
              
                let m2 = hash.get(&self.b1).unwrap().match_string(hash, s, m1.1);
                if m2.0 { 

                    if self.c1==-1 
                    { 
                        return m2; 
                    }
                    //if m2.1>=s.len() { return (false,i); }

                    let m3 = hash.get(&self.c1).unwrap().match_string(hash, s, m2.1);
                    if m3.0 
                    { 
                        return m3; 
                    }
                }
            
            }     
        }

        if self.a2!=-1 
        {
            let m1 = hash.get(&self.a2).unwrap().match_string(hash, s, i);            

            if m1.0 { 
                
                if self.b2==-1 { 
                    return m1; 
                }
                if m1.1>=s.len() { return (false,i); }
              
                let m2 = hash.get(&self.b2).unwrap().match_string(hash, s, m1.1);
                if m2.0 { 

                    if self.c2==-1 { 
                        return m2; 
                    }
                    //if m2.1>=s.len() { return (false,i); }

                    let m3 = hash.get(&self.c2).unwrap().match_string(hash, s, m2.1);
                    if m3.0 { 
                        return m3; 
                    }
                }
            
            }     
        }
        
        (false,0)     
    }
}

pub fn solve12(data:&[String],part1:bool)->i64
{
    let mut res = 0;
    let mut rules = true;

    let mut hash:HashMap<i32,Rule> = HashMap::new();

    for l in data {
        if l.is_empty()
        {
            rules = false;

            if !part1 {
                hash.insert(8,Rule::new("42 | 42 8".to_string()));
                hash.insert(11,Rule::new("42 31 | 42 11 31".to_string()));
            }           
        }
        else if rules {
                  if l.find(':')!=None {
                      let v:Vec<_> = l.split(": ").collect();
                      let id:i32 = v[0].parse().unwrap();
                      hash.insert(id,Rule::new(v[1].to_string()));
                  }
              }
                else
              {
                  let ll:String = l.chars().filter(|&c|c=='a' || c=='b').collect();
                  
                  if hash.get(&0).unwrap().match_string(&hash,&ll,0)==(true,ll.len())
                  {
                      res+=1;
                  }
              }
    }

    res
}


#[allow(unused)]
pub fn solve(data:&[String])->(i64,i64)
{
    let res = (solve12(data,true),solve12(data,false));

    println!("Day19");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}


#[test]
fn test1()
{
    let v = vec!["0: 4 6".to_string(),
                            "1: 2 3 | 3 2".to_string(),
                            "2: 4 4 | 5 5".to_string(),
                            "3: 4 5 | 5 4".to_string(),
                            "4: \"a\"".to_string(),
                            "5: \"b\"".to_string(),
                            "6: 1 5".to_string(),
                            "".to_string(),
                            "ababbb".to_string(),
                            "bababa".to_string(),
                            "abbbab".to_string(),
                            "aaabbb".to_string(),
                            "aaaabbb".to_string()];
    assert_eq!(solve12(&v,true),2);
}

#[test]
fn test2()
{
    let v = vec!["0: 4 6".to_string(),
                            "1: 2 3 | 3 2".to_string(),
                            "2: 4 4 | 5 5".to_string(),
                            "3: 4 5 | 5 4".to_string(),
                            "4: \"a\"".to_string(),
                            "5: \"b\"".to_string(),
                            "6: 1 5".to_string(),
                            "".to_string(),
                            "aaabbb".to_string()];
                            assert_eq!(solve12(&v,true),0);
}



#[test]
fn test3()
{
    let v = vec!["42: 9 14 | 10 1".to_string(),
                            "9: 14 27 | 1 26".to_string(),
                            "10: 23 14 | 28 1".to_string(),
                            "1: \"a\"".to_string(),
                            "11: 42 31".to_string(),
                            "5: 1 14 | 15 1".to_string(),
                            "19: 14 1 | 14 14".to_string(),
                            "12: 24 14 | 19 1".to_string(),
                            "16: 15 1 | 14 14".to_string(),
                            "31: 14 17 | 1 13".to_string(),
                            "6: 14 14 | 1 14".to_string(),
                            "2: 1 24 | 14 4".to_string(),
                            "0: 8 11".to_string(),
                            "13: 14 3 | 1 12".to_string(),
                            "15: 1 | 14".to_string(),
                            "17: 14 2 | 1 7".to_string(),
                            "23: 25 1 | 22 14".to_string(),
                            "28: 16 1".to_string(),
                            "4: 1 1".to_string(),
                            "20: 14 14 | 1 15".to_string(),
                            "3: 5 14 | 16 1".to_string(),
                            "27: 1 6 | 14 18".to_string(),
                            "14: \"b\"".to_string(),
                            "21: 14 1 | 1 14".to_string(),
                            "25: 1 1 | 1 14".to_string(),
                            "22: 14 14".to_string(),
                            "8: 42".to_string(),
                            "26: 14 22 | 1 20".to_string(),
                            "18: 15 15".to_string(),
                            "7: 14 5 | 1 21".to_string(),
                            "24: 14 1".to_string(),
                            "".to_string(),
                            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa".to_string(),
                            "bbabbbbaabaabba".to_string(),
                            "babbbbaabbbbbabbbbbbaabaaabaaa".to_string(),
                            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa".to_string(),
                            "bbbbbbbaaaabbbbaaabbabaaa".to_string(),
                            "bbbababbbbaaaaaaaabbababaaababaabab".to_string(),
                            "ababaaaaaabaaab".to_string(),
                            "ababaaaaabbbaba".to_string(),
                            "baabbaaaabbaaaababbaababb".to_string(),
                            "abbbbabbbbaaaababbbbbbaaaababb".to_string(),
                            "aaaaabbaabaaaaababaa".to_string(),
                            "aaaabbaaaabbaaa".to_string(),
                            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa".to_string(),
                            "babaaabbbaaabaababbaabababaaab".to_string(),
                            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba".to_string()
                            ];
                            assert_eq!(solve12(&v,true),3);
}



#[test]
fn test4()
{
    let v = vec!["42: 9 14 | 10 1".to_string(),
                            "9: 14 27 | 1 26".to_string(),
                            "10: 23 14 | 28 1".to_string(),
                            "1: \"a\"".to_string(),
                            "11: 42 31 | 42 11 31".to_string(),
                            "5: 1 14 | 15 1".to_string(),
                            "19: 14 1 | 14 14".to_string(),
                            "12: 24 14 | 19 1".to_string(),
                            "16: 15 1 | 14 14".to_string(),
                            "31: 14 17 | 1 13".to_string(),
                            "6: 14 14 | 1 14".to_string(),
                            "2: 1 24 | 14 4".to_string(),
                            "0: 8 11".to_string(),
                            "13: 14 3 | 1 12".to_string(),
                            "15: 1 | 14".to_string(),
                            "17: 14 2 | 1 7".to_string(),
                            "23: 25 1 | 22 14".to_string(),
                            "28: 16 1".to_string(),
                            "4: 1 1".to_string(),
                            "20: 14 14 | 1 15".to_string(),
                            "3: 5 14 | 16 1".to_string(),
                            "27: 1 6 | 14 18".to_string(),
                            "14: \"b\"".to_string(),
                            "21: 14 1 | 1 14".to_string(),
                            "25: 1 1 | 1 14".to_string(),
                            "22: 14 14".to_string(),
                            "8: 42 | 42 8".to_string(), 
                            "26: 14 22 | 1 20".to_string(),
                            "18: 15 15".to_string(),
                            "7: 14 5 | 1 21".to_string(),
                            "24: 14 1".to_string(),
                            "".to_string(),
                            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa".to_string(),
                            "bbabbbbaabaabba".to_string(),
                            "babbbbaabbbbbabbbbbbaabaaabaaa".to_string(),
                            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa".to_string(),
                            "bbbbbbbaaaabbbbaaabbabaaa".to_string(),
                            "bbbababbbbaaaaaaaabbababaaababaabab".to_string(),
                            "ababaaaaaabaaab".to_string(),
                            "ababaaaaabbbaba".to_string(),
                            "baabbaaaabbaaaababbaababb".to_string(),
                            "abbbbabbbbaaaababbbbbbaaaababb".to_string(),
                            "aaaaabbaabaaaaababaa".to_string(),
                            "aaaabbaaaabbaaa".to_string(),
                            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa".to_string(),
                            "babaaabbbaaabaababbaabababaaab".to_string(),
                            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba".to_string(),
                            ];
                            assert_eq!(solve12(&v,false),12);
}



#[test]
fn test5()
{
    let v = vec!["42: 9 14 | 10 1".to_string(),
                            "9: 14 27 | 1 26".to_string(),
                            "10: 23 14 | 28 1".to_string(),
                            "1: \"a\"".to_string(),
                            "11: 42 31 | 42 11 31".to_string(),
                            "5: 1 14 | 15 1".to_string(),
                            "19: 14 1 | 14 14".to_string(),
                            "12: 24 14 | 19 1".to_string(),
                            "16: 15 1 | 14 14".to_string(),
                            "31: 14 17 | 1 13".to_string(),
                            "6: 14 14 | 1 14".to_string(),
                            "2: 1 24 | 14 4".to_string(),
                            "0: 8 11".to_string(),
                            "13: 14 3 | 1 12".to_string(),
                            "15: 1 | 14".to_string(),
                            "17: 14 2 | 1 7".to_string(),
                            "23: 25 1 | 22 14".to_string(),
                            "28: 16 1".to_string(),
                            "4: 1 1".to_string(),
                            "20: 14 14 | 1 15".to_string(),
                            "3: 5 14 | 16 1".to_string(),
                            "27: 1 6 | 14 18".to_string(),
                            "14: \"b\"".to_string(),
                            "21: 14 1 | 1 14".to_string(),
                            "25: 1 1 | 1 14".to_string(),
                            "22: 14 14".to_string(),
                            "8: 42 | 42 8".to_string(), 
                            "26: 14 22 | 1 20".to_string(),
                            "18: 15 15".to_string(),
                            "7: 14 5 | 1 21".to_string(),
                            "24: 14 1".to_string(),
                            "".to_string(),
                            "aaaabbaaaabbaaa".to_string(),
                            ];
                            assert_eq!(solve12(&v,false),0);
}

