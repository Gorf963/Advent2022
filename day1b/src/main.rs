use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    //get file name from command line
    let args: Vec<String> = env::args().collect();
    if args.len() <2 {
        println!("You must pass a filename");
        return;
    }
    let file_name = args[1].clone();

    //Get file ref
    let input_file_result  = File::open(file_name);
    
    let input_file = match input_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening file: {:?}",error),
    };

    let mut elf = file_parser(input_file);
    elf.sort_by(|a,b| b.1.cmp(&a.1));
    let first = elf.iter().nth(0).unwrap();
    let second = elf.iter().nth(1).unwrap();
    let third = elf.iter().nth(2).unwrap();
    println!(" The Elf with the most cal is Elf {0} with cal of {1}",first.0, first.1);
    println!(" The Elf with the 2nd cal is Elf {0} with cal of {1}",second.0, second.1);
    println!(" The Elf with the 3rd cal is Elf {0} with cal of {1}",third.0, third.1);

    /*
    for topelf in elf.iter().nth(3) {
        println!(" The Elf with the most cal is Elf {0} with cal of {1}",topelf.0,topelf.1);
    }
    */
}
fn file_parser(f:File) -> Vec<(i32,i32)> {
    //elf array
    let mut elf_cal: Vec<(i32,i32)> = Vec::new();
    //temp working variables
    let mut elf_num:i32 = 1;
    let mut temp_cal:i32 = 0;
    //load buffrer reader
    let reader = BufReader::new(f);

    //Read and find max
    for line in reader.lines() {
        //Handle Change of Elf

        let l = match line{
            Ok(string)=> string,
            Err(error) => panic!("Bad line in file {:?}", error),
        };

        if  l.len() ==0 {
            
            elf_cal.push((elf_num.clone(), temp_cal.clone()));
            elf_num +=1 ;
            temp_cal = 0;
        }
        else {
            temp_cal += l.parse::<i32>().unwrap();
        }
    
    }
    //check last elf         
    elf_cal.push((elf_num.clone(), temp_cal.clone()));

    return elf_cal ;
}
