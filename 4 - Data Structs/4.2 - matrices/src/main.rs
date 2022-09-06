fn main() {
    let mtx: [[f32;3];2] = // 3 sütun 2 satır
        [
            [1.0, 0.0, 0.2],
            [1.2, 2.2, 3.3]
        ];

    println!(" elemanları: {:?}, boyut: {}", mtx, mtx.len()); // mtx.len 2 olarak çıkacak, 2 satır

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            println!("mtx[{}, {}] = {}",i, j, mtx[i][j]);
        }
    }
}
