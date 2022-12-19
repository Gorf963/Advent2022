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

    let elf = file_parser(input_file);
    println!(" The Elf with the most cal is Elf {0} with cal of {1}",elf.0,elf.1);
}
fn file_parser(f:File) -> (i32,i32) {
    //load buffrer reader
    let reader = BufReader::new(f);

    //Read and find max
    let mut largest_cal:i32 = 0;
    let mut largest_elf:i32 = 0;
    let mut elf_num:i32 = 0;
    let mut temp_cal:i32 = 0;

    for line in reader.lines() {
        //Handle Change of Elf

        let l = match line{
            Ok(string)=> string,
            Err(error) => panic!("Bad line in file {:?}", error),
        };

        if  l.len() ==0 {
            if temp_cal > largest_cal {
                largest_cal = temp_cal.clone();
                largest_elf = elf_num.clone();
            }
            elf_num +=1 ;
            temp_cal = 0;
        }
        else {
            temp_cal += l.parse::<i32>().unwrap();
        }
        
    }
    //check last elf         
    if temp_cal > largest_cal {
        largest_cal = temp_cal.clone();
        largest_elf = elf_num.clone();
    }   
    return (largest_elf,largest_cal) ;
}
