#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn gerar_beneficios_01_test() {
        let mut count = 0;
        let args: Vec<String> = env::args().collect();
        match super::gerar_beneficios::gerar(readfile(args)) {
            Ok(beneficios) => {
                assert_eq!(13301798, count);
            }
        }
    }
}
