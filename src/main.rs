use std::fs::File;
use std::env;
use std::io::{self, BufRead,BufReader};

fn summary(ip: &str, flag:&mut usize,q20_base: &mut usize,q30_base: &mut usize,n_base: &mut usize,base_total: &mut usize, gc_base: &mut usize,reads_count: &mut usize){
    *flag+=1;
    match flag {
        2 => {
            for item in ip.chars(){
                match item {
                    'G' => *gc_base+= 1,
                    'C' => *gc_base+= 1,
                    'N' => *n_base+= 1,
                    _ => (),
                }
            }
            *reads_count+=1;
        },
        4 => {
            let seq_len = ip.len();
            for item in ip.as_bytes(){
                if item - 33 >= 20{
                    *q20_base+=1;
                    if item -33 >= 30{
                        *q30_base+=1;
                    }
                }
            }
            *base_total += seq_len;
            *flag = 0;
        },
        _ => (),
    }
}

fn scale_number(base_total:usize) -> String {
    let mut scale_base_total = String::new();
    if base_total > 1000 && base_total < 1000000{
        scale_base_total = (base_total as f64 / 1000.0).round().to_string() + "K" ;
    }else if base_total >= 1000000 && base_total < 1000000000{
        scale_base_total = (base_total as f64 / 1000000.0).round().to_string() + "M";
    }else if base_total >= 1000000000 {
        scale_base_total = (base_total as f64 / 1000000000.0).round().to_string() + "G";
    }else{
        scale_base_total = base_total.to_string();
    }
    scale_base_total
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut flag:usize = 0;
    let mut q20_base:usize = 0;
    let mut q30_base:usize = 0;
    let mut n_base:usize = 0;
    let mut base_total:usize = 0;
    let mut gc_base:usize = 0;
    let mut reads_count = 0;
    let help = "Usage: SummaryFastq <input.fastq> \nNote: pigz -dc *.fq.gz | SummaryFastq - is recommended for gzip file\nAuthor: xizzy\nVersion:1.0\n";

    match args.len() {
        1 => print!("{}",help),
        2 => {
                if args[1] == "-"{
                    let stdin = io::stdin();
                    for line in stdin.lock().lines() {
                        if let Ok(s) = line{
                            summary(&s, &mut flag,&mut q20_base,&mut q30_base,&mut n_base,&mut base_total,&mut gc_base,&mut reads_count);
                        }
                    }
                }
                else{
                    let file = File::open(&args[1]).unwrap();
                    let lines = BufReader::new(file).lines();
                    for line in lines{
                        if let Ok(data) = line {
                            summary(&data, &mut flag,&mut q20_base,&mut q30_base,&mut n_base,&mut base_total,&mut gc_base,&mut reads_count);
                        }
                    }
                }
                let q20 = q20_base as f64 / base_total as f64;
                let q30 = q30_base as f64 / base_total as f64;
                let gc_ratio = gc_base as f64 / base_total as f64;
                let n_ratio = n_base as f64 / base_total as f64;
                print!("BaseNum\tReadsNum\tN Bases\tQ20\tQ30\tGC\n");
                print!("{} ({})\t{} ({})\t{:.3}%\t{:.2}%\t{:.2}%\t{:.2}%\n", base_total, scale_number(base_total) ,reads_count, scale_number(reads_count), n_ratio*100.0, q20*100.0, q30*100.0, gc_ratio*100.0);  
            },    
        _ => print!("{}",help),
    }
}