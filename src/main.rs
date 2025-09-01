// Fungsi mengambil ownership String dan mengembalikan panjangnya
fn panjang_ambil_ownership(s: String) -> usize {
    s.len()
} // s di-drop di sini karena fungsi habis

// Fungsi meminjam (borrow) String secara immutable untuk mendapatkan panjangnya
fn panjang_pinjam(s: &String) -> usize {
    s.len()
}

// Fungsi meminjam dua string dan mengembalikan referensi ke string yang lebih panjang
// Menggunakan lifetime annotation 'a agar Rust tahu referensi output valid selama input valid
fn string_terpanjang<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    // Ownership example
    let s1 = String::from("Halo");
    let len1 = panjang_ambil_ownership(s1); // s1 ownership pindah ke fungsi
    println!("Panjang s1: {}", len1);
    // println!("{}", s1); // ERROR! s1 sudah tidak valid karena ownership pindah

    // Borrowing example
    let s2 = String::from("Pinjam saya");
    let len2 = panjang_pinjam(&s2); // meminjam referensi ke s2
    println!("Panjang s2: {}", len2);
    println!("Masih bisa pakai s2: {}", s2); // s2 masih valid karena hanya dipinjam

    // Lifetime example
    let kata1 = "apel";
    let kata2 = "jeruk";

    let terpanjang = string_terpanjang(kata1, kata2);
    println!("String terpanjang: {}", terpanjang);
}
