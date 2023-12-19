use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::Read;
use std::iter::once;
use std::str::FromStr;
use itertools::{cloned, Itertools};
use crate::day19::InstrFilter::Reject;

#[derive(Debug,Copy, Clone,PartialEq,Eq)]
enum InstrFilter {
    Greater(char,usize),
    Lesser(char,usize),
    All,
    Reject

}

impl  InstrFilter {
    fn not(&self)->Self{
        match &self{
            Self::Greater(c, v) => Self::Lesser(*c, *v+1),
            Self::Lesser(c, v) => if *v == 0 {Self::All}else { Self::Greater(*c, *v - 1) },
            Self::All => Self::Reject,
            Self::Reject => Self::All
        }
    }
    fn target_carac(&self, carac:char)->bool {
        match &self{
            Self::Greater(c, _) if *c==carac => {true}
            Self::Lesser(c, _) if *c == carac => {true}
            _ => false
        }
    }
}
#[derive(Debug,Clone)]
struct Instruction {
    filter: InstrFilter,
    out: String,
}
impl Instruction{
    fn accept(&self, part : &Part)->Option<String>{
        match self.filter{
            InstrFilter::Greater(car,val) => if part.get_carac(car) > val {Some(self.out.clone())}else { None },
            InstrFilter::Lesser(car, val) => if part.get_carac(car) < val {Some(self.out.clone())}else { None },
            InstrFilter::All => Some(self.out.clone()),
            InstrFilter::Reject => Some("R".to_string())
        }
    }
}

struct Filter{
    workflows:HashMap<String,Vec<Instruction>>
}
impl FromStr for Filter{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let workflows=s.lines().map(|l|{
            let mut scan=l.split(&['{','}',',']).filter(|p|!p.is_empty());
            let name=scan.next().unwrap();
            let instructions = scan.map(|i|{
                if i.contains(':'){
                    let (compar,dest)=i.split_once(':').unwrap();
                    let (_,val)=compar.split_once(&['>','<']).unwrap();
                    let val:usize=val.parse().unwrap();
                    let carac = compar.chars().nth(0).unwrap();
                    if compar.contains('>'){
                        Instruction { filter: InstrFilter::Greater(carac, val),out: dest.to_string() }
                    }else{
                        Instruction { filter: InstrFilter::Lesser(carac,val),out:dest.to_string()}
                    }
                }else{
                    Instruction { filter: InstrFilter::All, out: i.to_string()}
                }
            }).collect_vec();
            (name.to_string(),instructions)
        }).collect();
        Ok(Self{workflows})
    }
}
impl Filter{

    fn apply(instructions : &[Instruction], part:&Part)-> String {
        for i in  instructions {
            if let Some(out) = i.accept(part) {return out}
        }
        unreachable!()
    }
    fn accept(&self, part:&Part)->bool{

        let mut pos = "in".to_string();
        while pos != "A" && pos != "R"{
            let instr = self.workflows.get(&pos).unwrap();
            pos= Self::apply(&instr,part);
        }
        &pos == "A"
    }

    fn compact(filters: &[InstrFilter]) -> Vec<InstrFilter>{
        "xmas".chars().flat_map(|c| {
            let mut max_greater:Option<usize> =None;
            let mut min_lesser:Option<usize>=None;
            let mut no_solutions=false;
            for f in filters.iter().filter(|f|f.target_carac(c)){
                match f {
                    InstrFilter::Greater(_, gt) => { max_greater = max_greater.map_or(Some(*gt), |maxgt|Some(max(maxgt, *gt)));}
                    InstrFilter::Lesser(_, lt) => { min_lesser = min_lesser.map_or(Some(*lt), |minlt|Some(min(minlt, *lt)));}
                    InstrFilter::All => {},
                    InstrFilter::Reject => {no_solutions=true;}

                }
            }
            let no_solutions=  no_solutions || max_greater.map(|maxgt| min_lesser.map(|minlt| minlt <= maxgt)).flatten().unwrap_or(false);

            if no_solutions { vec![InstrFilter::Reject]}else {
            [max_greater.filter(|_|!no_solutions).map(|v|InstrFilter::Greater(c, v)), min_lesser.filter(|_|!no_solutions).map(|v|InstrFilter::Lesser(c, v))].into_iter().flatten().collect()}
        }).collect()
    }
    fn accepted_combinations(&self)->usize{
        let mut accepted_paths=vec![];
        let mut current_paths=vec![("in".to_string(),vec![])];
        while ! current_paths.is_empty() {
            let new_paths:Vec<(String,Vec<InstrFilter>)>= current_paths.iter().flat_map(|(src, path)|{
                let workflow=self.workflows.get(src).unwrap();
                workflow.iter().enumerate().map(|(i,instr)| {
                    (instr.out.clone(), Self::compact(&path.iter().cloned().chain(workflow[..i].iter().map(|i|i.filter.not())).chain(once(instr.filter.clone())).collect_vec()))
                })
            })
                .filter(|(dst,instr)|dst!="R" && !instr.contains(&InstrFilter::Reject))
                .collect_vec();

            for p in new_paths.iter().filter(|(out,_)|out=="A"){
                accepted_paths.push(p.1.clone());
            }
            current_paths = new_paths.into_iter().filter(|(out,_)|out!="A").collect();
        }

        accepted_paths.into_iter().map(|p|{
            let compact= Self::compact(&p);
            let result= "xmas".chars().map(|c|{
                let mut min_greater :Option<usize> =None;
                let mut max_lesser :Option<usize>=None;
                for f in p.iter().filter(|f|f.target_carac(c)){
                    match f {
                        InstrFilter::Greater(_, gt) => {min_greater = min_greater.map_or(Some(*gt),|mingt|Some(min(mingt,*gt)));}
                        InstrFilter::Lesser(_, lt) => {max_lesser = max_lesser.map_or(Some(*lt),|maxls|Some(max(maxls,*lt)));}
                        InstrFilter::All => {panic!("should not have an All here, since were compacted")},
                        InstrFilter::Reject => {panic!("Reject should have bee filtered out")}

                    }
                }
                max_lesser.unwrap_or(4001).saturating_sub(1).saturating_sub(min_greater.unwrap_or(0))
            }).product::<usize>();
            result
        } ).sum()
    }
}

pub fn filter_parts(){
    let input=include_str!("../resources/day19_workflows_parts.txt");
    // let (filter,parts)=read_input(input);
    let sum_filtered=sum_accepted_parts(input);
    println!("sum of filtered parts : {sum_filtered}");

    let (filter,_)=read_input(input);
    let combinations= filter.accepted_combinations();
    println!("combinations : {combinations}");
}
#[derive(Debug)]
struct Part {
    x:usize,
    m:usize,
    a:usize,
    s:usize
}
impl FromStr for Part{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts=s.split(&['{','}',',']).filter(|p|!p.is_empty());
        let x:usize = parts.next().unwrap().split_once('=').unwrap().1.parse().unwrap();
        let m:usize = parts.next().unwrap().split_once('=').unwrap().1.parse().unwrap();
        let a:usize = parts.next().unwrap().split_once('=').unwrap().1.parse().unwrap();
        let s:usize = parts.next().unwrap().split_once('=').unwrap().1.parse().unwrap();
        Ok(Self{x,m,a,s})
    }
}
impl Part {
    fn get_carac(&self, carac:char)-> usize{
    match carac{
        'x'=> self.x,
        'm'=> self.m,
        'a'=> self.a,
        's'=> self.s,
        _ => panic!("no carac '{carac}'")
    }
    }
}

fn read_input(input:&str)->(Filter,Vec<Part>){

    let sep=input.lines().enumerate().find(|(_,l)|l.is_empty()).map(|(i,_)|i).unwrap();
    let filter:Filter=input.lines().take(sep).join("\n").parse().unwrap();
    let parts:Vec<Part>=input.lines().skip(sep+1).filter_map(|l|l.parse().ok()).collect();

    (filter,parts)
}

fn sum_accepted_parts(input:&str)->usize{

    let (filter,parts)=read_input(input);
    parts.into_iter().filter(|p|filter.accept(p)).map(|Part{x,m,a,s}|x+m+a+s).sum()

}

#[cfg(test)]
mod tests {

    use indoc::indoc;
    use super::*;

#[test]
fn aoc_example_works() {
    let input=indoc! {"
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    "};

    assert_eq!(19114,sum_accepted_parts(input));
    let (filter,_)=read_input(input);
    assert_eq!(167409079868000,filter.accepted_combinations());

}
}