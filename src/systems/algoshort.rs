use rand::Rng;
use std::io;
use std::time::Instant;

fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_index = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

fn insertion_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i as i32 - 1;
        while j >= 0 && arr[j as usize] > key {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = key;
    }
}

fn merge_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n > 1 {
        let mid = n / 2;
        merge_sort(&mut arr[0..mid]);   // Sort left half
        merge_sort(&mut arr[mid..]);     // Sort right half
        merge(arr);                      // Merge them back together
    }
}

fn merge(arr: &mut [i32]) {
    let mid = arr.len() / 2;
    let left = arr[0..mid].to_vec();   // Create a copy of the left half
    let right = arr[mid..].to_vec();    // Create a copy of the right half

    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}


fn quick_sort(arr: &mut [i32]) {
    if arr.len() > 1 {
        let pivot_index = partition(arr);
        let (left, right) = arr.split_at_mut(pivot_index);
        quick_sort(left);
        quick_sort(&mut right[1..]);
    }
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);
    let mut store_index = 0;
    for i in 0..arr.len() - 1 {
        if arr[i] < arr[arr.len() - 1] {
            arr.swap(i, store_index);
            store_index += 1;
        }
    }
    arr.swap(store_index, arr.len() - 1);
    store_index
}

pub fn main() {
    println!("Menu de Tri");

    // Demander le nombre d'éléments à trier
    println!("Veuillez entrer le nombre d'éléments à trier :");
    let mut size_input = String::new();
    io::stdin().read_line(&mut size_input).expect("Erreur de lecture");
    let size: usize = size_input.trim().parse().expect("Veuillez entrer un nombre valide");

    // Générer un tableau d'éléments aléatoires
    let mut arr: Vec<i32> = (0..size).map(|_| rand::thread_rng().gen_range(0..1000)).collect();


    // Afficher le menu de choix des algorithmes de tri
    println!("Choisissez un algorithme de tri :");
    println!("1. Bubble Sort");
    println!("2. Selection Sort");
    println!("3. Insertion Sort");
    println!("4. Merge Sort");
    println!("5. Quick Sort");
    println!("Veuillez entrer le numéro de l'algorithme :");

    let mut choice_input = String::new();
    io::stdin().read_line(&mut choice_input).expect("Erreur de lecture");
    let choice: u32 = choice_input.trim().parse().expect("Veuillez entrer un nombre valide");

    // Effectuer le tri en fonction du choix de l'utilisateur
    let start = Instant::now();
    match choice {
        1 => bubble_sort(&mut arr),
        2 => selection_sort(&mut arr),
        3 => insertion_sort(&mut arr),
        4 => {
            let mut temp_arr = arr.clone();
            merge_sort(&mut temp_arr);
            arr.copy_from_slice(&temp_arr);
        }
        5 => quick_sort(&mut arr),
        _ => println!("Choix invalide"),
    }
    let duration = start.elapsed();


    println!("Temps de tri : {:?}", duration);
}
