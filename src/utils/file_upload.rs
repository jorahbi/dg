use crate::config::UploadConfig;
use crate::error::{AppError, Result};
use crate::schema::common::{FileUploadResponse, FileValidityCheck};
use axum::extract::multipart::Multipart;
use chrono::Utc;
use mime_guess::from_path;
use std::path::Path;

pub struct FileUploadService<'a> {
    config: &'a UploadConfig,
}

impl<'a> FileUploadService<'a> {
    pub fn new(config: &'a UploadConfig) -> Self {
        Self { config }
    }

    pub async fn upload_file(&self, mut multipart: Multipart) -> Result<FileUploadResponse> {
        while let Some(field) = multipart
            .next_field()
            .await
            .map_err(|e| AppError::Validation(format!("Failed to parse file field: {}", e)))?
        {
            let name = field.name().unwrap_or("unknown");

            if name != "avatar" && name != "image" && name != "file" {
                continue; // 跳过非文件字段
            }

            let filename = field
                .file_name()
                .ok_or_else(|| AppError::Validation("Filename cannot be empty".to_string()))?
                .to_string();

            let data = field
                .bytes()
                .await
                .map_err(|e| AppError::Validation(format!("Failed to read file content: {}", e)))?;

            return self.process_file_upload_round_name(&filename, &data).await;
        }

        Err(AppError::Validation("No valid file field found".to_string()))
    }

    pub async fn process_file_upload_round_name(
        &self,
        filename: &str,
        data: &[u8],
    ) -> Result<FileUploadResponse> {
        let file_ext = self.upload_check(data, filename)?;

        // 生成唯一文件名
        let file_id = uuid::Uuid::new_v4().to_string();
        let dir = format!(
            "{}/{}/{}",
            &self.config.upload_path,
            &file_id[0..2],
            &file_id[2..4]
        );
        let safe_filename = format!("{}-{}.{}", Utc::now().timestamp(), &file_id[..8], file_ext);
        self.save_image(data, &dir, &file_ext, &safe_filename).await
    }

    pub async fn process_file_upload(
        &self,
        filename: &str,
        path: &str,
        data: &[u8],
    ) -> Result<FileUploadResponse> {
        let file_ext = self.upload_check(data, filename)?;
        let dir = format!("{}/{}", &self.config.upload_path, path);
        self.save_image(data, &dir, &file_ext, filename).await
    }

    fn upload_check(&self, data: &[u8], filename: &str) -> Result<String> {
        // 验证文件大小
        if data.len() > self.config.max_file_size {
            return Err(AppError::Validation(format!(
                "文件大小超过限制，最大允许{}字节",
                self.config.max_file_size
            )));
        }

        // 验证文件扩展名
        let file_ext = Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        if !self
            .config
            .allowed_extensions
            .contains(&file_ext.to_lowercase())
        {
            return Err(AppError::Validation(format!(
                "不支持的文件类型: {}，支持的类型: {}",
                file_ext,
                self.config.allowed_extensions.join(", ")
            )));
        }
        Ok(file_ext.to_string())
    }

    async fn save_image(
        &self,
        data: &[u8],
        dir: &str,
        file_ext: &str,
        safe_filename: &str,
    ) -> Result<FileUploadResponse> {
        // 确保上传目录存在
        std::fs::create_dir_all(&dir)?;

        // 保存文件
        let file_path = Path::new(&dir).join(&safe_filename);
        tokio::fs::write(&file_path, data)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to save file: {}", e)))?;

        // 生成文件URL
        let file_url = format!("{}", safe_filename);

        // 检查文件有效性（如果是图片）
        let validity_check = if self.is_image_file(file_ext) {
            Some(self.validate_image(data).await?)
        } else {
            None
        };

        Ok(FileUploadResponse {
            url: format!("{}/{}", dir, file_url),
            file_size: data.len() as u64,
            upload_time: Utc::now(),
            status: "success".to_string(),
            validity_check,
        })
    }

    fn is_image_file(&self, extension: &str) -> bool {
        matches!(
            extension.to_lowercase().as_str(),
            "jpg" | "jpeg" | "png" | "gif" | "webp"
        )
    }

    async fn validate_image(&self, data: &[u8]) -> Result<FileValidityCheck> {
        // 简单的图片格式验证
        let is_valid = match data {
            [0xFF, 0xD8, ..] if data.len() > 4 => {
                // JPEG
                data.len() > 100 // 最小大小检查
            }
            [0x89, 0x50, 0x4E, 0x47, ..] => {
                // PNG
                true
            }
            [0x47, 0x49, 0x46, 0x38, ..] => {
                // GIF
                true
            }
            _ => false,
        };

        Ok(FileValidityCheck {
            is_valid,
            confidence: if is_valid { 0.95 } else { 0.1 },
            issues: if is_valid {
                vec![]
            } else {
                vec!["不支持的图片格式".to_string()]
            },
        })
    }

    pub async fn delete_file(&self, _file_id: &str) -> Result<()> {
        // 在实际实现中，这里应该根据file_id查找并删除文件
        // 目前返回成功，因为我们没有实现文件索引
        Ok(())
    }

    pub fn get_file_info(&self, file_path: &str) -> Result<(String, u64)> {
        let path = Path::new(file_path);

        if !path.exists() {
            return Err(AppError::NotFound("File not found".to_string()));
        }

        let metadata = std::fs::metadata(path)?;
        let mime_type = from_path(path).first_or_octet_stream().to_string();

        Ok((mime_type, metadata.len()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_upload_config() -> UploadConfig {
        UploadConfig {
            max_file_size: 5 * 1024 * 1024, // 5MB
            allowed_extensions: vec![
                "jpg".to_string(),
                "jpeg".to_string(),
                "png".to_string(),
                "gif".to_string(),
                "webp".to_string(),
            ],
            upload_path: "test_uploads".to_string(),
            qrcord_size: 200,
        }
    }

    #[tokio::test]
    async fn test_validate_image() {
        let cfg = create_test_upload_config();
        let upload_service = FileUploadService::new(&cfg);

        // 测试有效的JPEG图片头
        let jpeg_data = vec![0xFF, 0xD8, 0xFF, 0xE0];
        let result = upload_service.validate_image(&jpeg_data).await.unwrap();
        assert!(result.is_valid);

        // 测试无效的图片数据
        let invalid_data = vec![0x00, 0x01, 0x02, 0x03];
        let result = upload_service.validate_image(&invalid_data).await.unwrap();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_is_image_file() {
        let cfg = create_test_upload_config();
        let upload_service = FileUploadService::new(&cfg);

        assert!(upload_service.is_image_file("jpg"));
        assert!(upload_service.is_image_file("png"));
        assert!(upload_service.is_image_file("gif"));
        assert!(!upload_service.is_image_file("txt"));
        assert!(!upload_service.is_image_file("pdf"));
    }
}
