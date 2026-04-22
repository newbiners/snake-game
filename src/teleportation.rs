/// Ukuran grid (20x20)
const GRID_SIZE: i32 = 50;

/// Wrap posisi kepala ular agar teleportasi ke sisi berlawanan:
/// - Keluar kanan  → muncul dari kiri
/// - Keluar kiri   → muncul dari kanan
/// - Keluar bawah  → muncul dari atas
/// - Keluar atas   → muncul dari bawah
pub fn wrap_position(pos: &mut (i32, i32)) {
    // Horizontal wrap
    if pos.0 >= GRID_SIZE {
        pos.0 = 0; // Keluar kanan → muncul kiri
    } else if pos.0 < 0 {
        pos.0 = GRID_SIZE - 1; // Keluar kiri → muncul kanan
    }

    // Vertical wrap
    if pos.1 >= GRID_SIZE {
        pos.1 = 0; // Keluar bawah → muncul atas
    } else if pos.1 < 0 {
        pos.1 = GRID_SIZE - 1; // Keluar atas → muncul bawah
    }
}
