use std::collections::VecDeque;
use std::io::{self, Write};

fn main() {
    let mut daftar_barang: Vec<Barang> = vec![
        Barang::baru(1, "Buku", 100),
        Barang::baru(2, "Pensil", 200),
        Barang::baru(3, "Penggaris", 150),
    ];

    let mut stack_barang: Vec<Barang> = Vec::new();
    let mut queue_barang: VecDeque<Barang> = VecDeque::new();

    loop {
        println!("Pilihan:");
        println!("1. Tampilkan semua barang");
        println!("2. Tambah barang baru");
        println!("3. Tambah ke Stack");
        println!("4. Tambah ke Queue");
        println!("5. Keluar");

        let pilihan: u32 = input("Masukkan pilihan Anda: ").trim().parse().expect("Harap masukkan angka");

        match pilihan {
            1 => tampilkan_barang(&daftar_barang),
            2 => tambah_barang(&mut daftar_barang),
            3 => {
                if let Some(barang) = daftar_barang.last().cloned() {
                    stack_barang.push(barang);
                    println!("Barang berhasil ditambahkan ke Stack!");
                }
            },
            4 => {
                if let Some(barang) = daftar_barang.pop() {
                    queue_barang.push_back(barang);
                    println!("Barang berhasil ditambahkan ke Queue!");
                }
            },
            5 => break,
            _ => println!("Pilihan tidak valid!"),
        }
    }
}

fn input(pesan: &str) -> String {
    print!("{}", pesan);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().to_string()
}

fn tampilkan_barang(daftar: &Vec<Barang>) {
    println!("Daftar Barang:");
    for barang in daftar {
        println!("ID: {}, Nama: {}, Jumlah: {}", barang.id, barang.nama, barang.jumlah);
    }
}

fn tambah_barang(daftar: &mut Vec<Barang>) {
    let id: u32 = input("Masukkan ID barang: ").trim().parse().expect("Harap masukkan angka");
    let nama = input("Masukkan nama barang: ");
    let jumlah: u32 = input("Masukkan jumlah barang: ").trim().parse().expect("Harap masukkan angka");

    daftar.push(Barang::baru(id, &nama, jumlah));
    println!("Barang berhasil ditambahkan!");
}

#[derive(Clone)] // Menambahkan trait Clone untuk struct Barang
struct Barang {
    id: u32,
    nama: String,
    jumlah: u32,
}

impl Barang {
    fn baru(id: u32, nama: &str, jumlah: u32) -> Self {
        Self {
            id,
            nama: nama.to_string(),
            jumlah,
        }
    }
}
