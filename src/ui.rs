pub fn print_qr_code(ip_adress: &str) {
    qr2term::print_qr(ip_adress).unwrap();
}
