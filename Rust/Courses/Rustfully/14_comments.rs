fn get_rating(movie: &str) -> i32 {
    // MOVIE API: Docs: ...
    let rating: i32 = get_movie_data(movie);
    // expression -> retorna um valor, faz sentido pois é uma função
    rating
}

// This is a dummy function used for testing
fn get_movie_data(movie: &str) -> i32 {
    10
}

fn add(numbers: &[i32]) {
    let mut sum = 0;
    for n in numbers {
        sum += n;

        // // Debug: print each step of the summing process
        // println!("Adding {}, current sum is {}", n, sum);
    }
}

// This is our main entry point
fn main() {
    get_rating("Matrix");
    add(&[1, 2, 3]);
}

/*
This is a comment
written by Nuna

x = 1 + 1
*/