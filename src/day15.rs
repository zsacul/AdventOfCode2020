use std::collections::HashMap;

#[derive(PartialEq,Debug)]
struct Pair
{
    a:i32,
    b:i32,
}

impl Pair
{
    fn new()->Self
    {
        Pair {
            a:-1,
            b:-1,
        }
    }

    fn add(&mut self,n:i32)
    {
        if self.a==-1 { self.a = n; }
        else
        {            
            self.b = self.a;
            self.a = n;            
        }
    }

    fn diff(&self)->i32
    {
        if self.a!=-1 && self.b!=-1 
        {
            self.a - self.b
        }
          else 
        {
            0
        }
    }
}

fn solve1(data:&Vec<i32>,rounds:usize)->i32
{
    let mut mem    : HashMap<i32,Pair> = HashMap::new();
    let mut spoken = 0;

    for round in 0..rounds 
    {
        if round<data.len()
        {
            spoken = data[round];
        }
            else
        {
            spoken = mem.get(&spoken).unwrap_or( &Pair::new()).diff();
        }

        if mem.get(&spoken)==None 
        {
            mem.insert(spoken,Pair::new());
        }
            
        mem.get_mut(&spoken).unwrap().add(round as i32+1);
    }
    spoken   
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)->(i32,i32)
{
    let tab = data[0].split(",").map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
    let res = (solve1(&tab,2020),
                         solve1(&tab,30000000));

    println!("Day15");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}




#[test]
fn test0()  { let v = vec![0,3,6];       assert_eq!(solve1(&v,2020),436);    }

#[test]
fn test1() { let v = vec![1,3,2];       assert_eq!(solve1(&v,2020),1);       }

#[test]
fn test2() { let v = vec![2,1,3];      assert_eq!(solve1(&v,2020),10);       }

#[test]
fn test3() { let v = vec![1,2,3];      assert_eq!(solve1(&v,2020),27);       }

#[test]
fn test4() { let v = vec![2,3,1];      assert_eq!(solve1(&v,2020),78);       }

#[test]
fn test5()   {  let v = vec![3,2,1];  assert_eq!(solve1(&v,2020),438);       }
 
#[test]
fn test6()  { let v = vec![3,1,2];  assert_eq!(solve1(&v,2020),1836);        }

#[test]
fn test7()  { let v = vec![0,3,6];  assert_eq!(solve1(&v,30000000),175594);  }

#[test]
fn test8()  { let v = vec![1,3,2];  assert_eq!(solve1(&v,30000000),2578);    }

#[test]
fn test9()  { let v = vec![2,1,3];  assert_eq!(solve1(&v,30000000),3544142); }

#[test]
fn test10() { let v = vec![1,2,3];  assert_eq!(solve1(&v,30000000),261214);  }

#[test]
fn test11() { let v = vec![2,3,1];  assert_eq!(solve1(&v,30000000),6895259); }

#[test]
fn test12() { let v = vec![3,2,1];  assert_eq!(solve1(&v,30000000),18);      }

#[test]
fn test13() { let v = vec![3,1,2];  assert_eq!(solve1(&v,30000000),362);     }
