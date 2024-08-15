// src/utils/file.rs

use crate::utils::error::{HubSpotError, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_properties<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>> {
    let file = File::open(file_path).map_err(|e| HubSpotError::FileError(e.to_string()))?;
    let reader = BufReader::new(file);
    let properties: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    Ok(properties)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Seek, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_properties() -> Result<()> {
        // 임시 파일 생성
        let mut temp_file = NamedTempFile::new()?;

        // 테스트 데이터 작성
        writeln!(temp_file, "property1")?;
        writeln!(temp_file, "  property2  ")?; // 앞뒤 공백 포함
        writeln!(temp_file, "")?; // 빈 줄
        writeln!(temp_file, "property3")?;

        // 파일 포인터를 시작으로 되돌림
        temp_file.rewind()?;

        // 프로퍼티 읽기
        let properties = read_properties(temp_file.path())?;

        // 결과 검증
        assert_eq!(properties, vec!["property1", "property2", "property3"]);

        Ok(())
    }

    #[test]
    fn test_read_properties_empty_file() -> Result<()> {
        // 빈 임시 파일 생성
        let temp_file = NamedTempFile::new()?;

        // 프로퍼티 읽기
        let properties = read_properties(temp_file.path())?;

        // 결과 검증
        assert!(properties.is_empty());

        Ok(())
    }

    #[test]
    fn test_read_properties_nonexistent_file() {
        // 존재하지 않는 파일 경로
        let result = read_properties("nonexistent_file.txt");

        // 오류 검증
        assert!(result.is_err());
        if let Err(HubSpotError::FileError(_)) = result {
            // 예상된 오류 타입
        } else {
            panic!("Expected FileError, got {:?}", result);
        }
    }
}
