use std::io;
use std::time::Instant;

pub fn main() {
menu();
}

fn all(){
    benchmark_square_loop(10000000);

    benchmark_vector_sum(10000000);

    benchmark_fibonacci(40);
}

fn benchmark_square_loop(n: u64){
    let now = Instant::now();
    for k in 0..n {
        let _x = k.pow(2);
    }
    let elapsed = now.elapsed();
    println!("Temps d'exécution de benchmark square loop est: {:?} pour {} itération", elapsed, n);
}

fn benchmark_vector_sum(n: usize) {
    let mut vec = vec![0; n];
    for i in 0..n {
        vec[i] = i as u64;
    }

    let now = Instant::now();
    let _result = vec.iter().sum::<u64>();
    let elapsed = now.elapsed();

    println!("Temps d'exécution de benchmark vector sum est: {:?} pour {} itération", elapsed, n);
}

fn benchmark_fibonacci(n: u64) {
    let now = Instant::now();
    let _result = fibonacci(n);
    let elapsed = now.elapsed();
    println!("Temps d'exécution de fibonacci est: {:?} pour {} itération", elapsed, n);
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn menu(){
    loop {
        println!("1. Tous exécuter");
        println!("2. Benchmark square loop");
        println!("3. Benchmark vector sum");
        println!("4. Fibonacci");
        println!("0. Quitter");

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let choice = input.trim().parse::<usize>().unwrap();

        match choice {
            1 => all(),
            2 => {
                println!("Entrez le nombre d'itération");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let n = input.trim().parse::<u64>().unwrap();
                benchmark_square_loop(n);
            },
            3 => {
                println!("Entrez le nombre d'itération");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let n = input.trim().parse::<usize>().unwrap();
                benchmark_vector_sum(n);
            },
            4 => {
                println!("Entrez le nombre de fibonnaci");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let n = input.trim().parse::<u64>().unwrap();
                benchmark_fibonacci(n);
            },
            0 => {
                println!("Au revoir !");
                break;
            },
            _ => println!("Choix invalide"),
        }
    }
}