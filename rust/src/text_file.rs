//! Leitura de arquivo texto UTF-8 tolerante a BOM.
//!
//! Caches gravados por outros programas costumam começar com o BOM (EF BB BF),
//! e o `serde_json` o recusa: o arquivo inteiro passava por "ilegível" e a
//! transcrição, os metadados e a fila sumiam. Este helper tira o BOM antes.

use std::path::Path;

/// `std::fs::read_to_string` sem o BOM UTF-8 inicial, se houver.
pub fn read(path: impl AsRef<Path>) -> std::io::Result<String> {
    let s = std::fs::read_to_string(path)?;
    Ok(match s.strip_prefix('\u{feff}') {
        Some(rest) => rest.to_owned(),
        None => s,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn tira_o_bom() {
        let p = std::env::temp_dir().join(format!("noobs-bom-{}.json", std::process::id()));
        std::fs::write(&p, b"\xEF\xBB\xBF{\"a\":1}").unwrap();
        assert_eq!(super::read(&p).unwrap(), "{\"a\":1}");
        std::fs::write(&p, b"{\"a\":1}").unwrap();
        assert_eq!(super::read(&p).unwrap(), "{\"a\":1}");
        let _ = std::fs::remove_file(&p);
    }
}
