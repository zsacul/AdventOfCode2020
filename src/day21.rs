use std::collections::{HashSet,HashMap};
use std::iter::FromIterator;

fn intersection(a: HashSet<String>, b: &HashSet<String>) -> HashSet<String> 
{    
    a.into_iter().filter(|e| b.contains(e)).collect()
}

pub fn solve12(data:&[String])->(i64,String)
{
    let mut alergens : HashMap<String,bool> = HashMap::new();
    let mut pieces   : Vec<(HashSet<String>,HashSet<String>)> = vec![];
    let mut solution : Vec<(String,String)> = vec![];

    for line in data.iter() {
        if line.contains("(contains ")
        {            
            let v : Vec<_> = line.split(" (contains ").collect();
            let alerg = v[1][..v[1].len()-1].split(", ").map(|e| e.to_string() ).collect::<Vec<String>>();
            let ingr  = v[0].split(' ').map(|e| e.to_string() ).collect::<Vec<String>>();

            for aler in alerg.iter() {
                alergens.insert(aler.clone(),false);
            }            

            pieces.push((HashSet::from_iter(ingr),HashSet::from_iter(alerg)));
        }
    }

    let mut found = true;

    while found 
    {
        found = false;
    
        for (aler,used) in alergens.iter() {
        if !(*used)
        {
            let mut first = true;
            let mut acc_hash = HashSet::new();
                       
            for (h_ing,h_aler) in pieces.iter()
            {
                if h_aler.contains(aler) {
                    if first {
                        first = false;
                        acc_hash = h_ing.clone();
                    }
                      else   
                    {
                        acc_hash = intersection(acc_hash,h_ing);
                    }                
                }
            }

            if acc_hash.len()==1
            {
                let  name = acc_hash.iter().last().unwrap();
                solution.push((aler.clone(),name.clone()));

                for (ing,ale) in pieces.iter_mut()
                {
                    ing.remove(name);
                    ale.remove(aler);
                }
                found = true;                
                break;
            }
        }
        }
        alergens.insert(solution.last().unwrap().0.clone(),true);
    }

    solution.sort();

    let part1 = pieces.iter().map(|(ing,_)| ing.len() as i64).sum();    
    let part2 = solution.iter().map(|(_,alg)| alg.to_string()).collect::<Vec<String>>().join(",");

    (part1,part2) 
}



#[allow(unused)]
pub fn solve(data:&[String])->(i64,String)
{
    let res = solve12(data);
  
    println!("Day21");
    println!("part1:{}",res.0);
    println!("part2:{}",res.1);    
    
    res
}


#[test]
fn test1()
{
    let v = vec![
        "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)".to_string(),
        "trh fvjkl sbzzf mxmxvkd (contains dairy)".to_string(),
        "sqjhc fvjkl (contains soy)".to_string(),
        "sqjhc mxmxvkd sbzzf (contains fish)".to_string(),
        ];
    assert_eq!(solve12(&v).0,5);
}

#[test]
fn test2()
{
    let v = vec![
        "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)".to_string(),
        "trh fvjkl sbzzf mxmxvkd (contains dairy)".to_string(),
        "sqjhc fvjkl (contains soy)".to_string(),
        "sqjhc mxmxvkd sbzzf (contains fish)".to_string(),
        ];
    assert_eq!(solve12(&v).1,"mxmxvkd,sqjhc,fvjkl".to_string());
}
