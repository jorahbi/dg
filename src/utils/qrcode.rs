use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgb};
use qrcode::QrCode;

/// 二维码生成器
pub struct QRGenerator {
    /// 默认二维码尺寸
    default_size: u32,
    /// 默认错误纠正级别
    error_correction: qrcode::EcLevel,
}

impl Default for QRGenerator {
    fn default() -> Self {
        Self {
            default_size: 200,
            error_correction: qrcode::EcLevel::M,
        }
    }
}

impl QRGenerator {
    /// 创建新的二维码生成器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置默认尺寸
    pub fn with_size(mut self, size: u32) -> Self {
        self.default_size = size;
        self
    }

    /// 设置错误纠正级别
    pub fn with_error_correction(mut self, level: qrcode::EcLevel) -> Self {
        self.error_correction = level;
        self
    }

    /// 生成二维码图像 (RGB格式)
    ///
    /// # 参数
    /// * `data` - 要编码的数据
    /// * `size` - 图像尺寸 (可选，使用默认值如果为None)
    ///
    /// # 返回
    /// 返回RGB图像数据的字节数组
    pub async fn generate_image(&self, data: &str, size: Option<u32>) -> Result<Vec<u8>> {
        let qr_size = size.unwrap_or(self.default_size);

        // 创建QR码
        let qr_code = QrCode::with_error_correction_level(data, self.error_correction)?;

        // 转换为图像
        let image = qr_code
            .render::<image::Luma<u8>>()
            .min_dimensions(qr_size, qr_size)
            .max_dimensions(qr_size, qr_size)
            .build();

        // 转换为RGB格式
        let rgb_image = ImageBuffer::from_fn(image.width(), image.height(), |x, y| {
            let pixel = image.get_pixel(x, y);
            let value = pixel[0];
            if value == 0 {
                Rgb([0, 0, 0]) // 黑色
            } else {
                Rgb([255, 255, 255]) // 白色
            }
        });

        // 转换为DynamicImage并编码为PNG字节数组
        let dynamic_image = DynamicImage::ImageRgb8(rgb_image);
        let mut buffer = Vec::new();
        dynamic_image.write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Png)?;

        Ok(buffer)
    }

    /// 生成二维码并保存到文件
    ///
    /// # 参数
    /// * `data` - 要编码的数据
    /// * `path` - 保存路径
    /// * `size` - 图像尺寸 (可选)
    ///
    /// # 返回
    /// 返回保存的文件大小
    pub fn generate_to_file(&self, data: &str, path: &str, size: Option<u32>) -> Result<u64> {
        let qr_size = size.unwrap_or(self.default_size);

        // 创建QR码
        let qr_code = QrCode::with_error_correction_level(data, self.error_correction)?;

        // 保存到文件
        let image = qr_code
            .render::<image::Luma<u8>>()
            .min_dimensions(qr_size, qr_size)
            .max_dimensions(qr_size, qr_size)
            .build();

        image.save(path)?;

        // 获取文件大小
        std::fs::metadata(path)
            .map(|metadata| metadata.len())
            .map_err(|e| anyhow!("Failed to get file metadata: {}", e))
    }

    /// 生成SVG格式的二维码
    ///
    /// # 参数
    /// * `data` - 要编码的数据
    /// * `size` - 图像尺寸 (可选)
    ///
    /// # 返回
    /// 返回SVG字符串
    pub fn generate_svg(&self, data: &str, size: Option<u32>) -> Result<String> {
        let qr_size = size.unwrap_or(self.default_size) as usize;

        // 创建QR码
        let qr_code = QrCode::with_error_correction_level(data, self.error_correction)?;

        // 生成SVG字符串
        let image = qr_code
            .render::<image::Luma<u8>>()
            .min_dimensions(qr_size as u32, qr_size as u32)
            .max_dimensions(qr_size as u32, qr_size as u32)
            .build();

        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
            qr_size, qr_size, qr_size, qr_size
        ));
        svg.push_str(r#"<rect width="100%" height="100%" fill="white"/>"#);

        let cell_size = qr_size / image.width() as usize;
        for (y, row) in image.rows().enumerate() {
            for (x, pixel) in row.enumerate() {
                if pixel[0] == 0 {
                    // 黑色像素
                    svg.push_str(&format!(
                        r#"<rect x="{}" y="{}" width="{}" height="{}" fill="black"/>"#,
                        x * cell_size,
                        y * cell_size,
                        cell_size,
                        cell_size
                    ));
                }
            }
        }

        svg.push_str("</svg>");
        Ok(svg)
    }

    /// 生成Base64编码的二维码图像
    ///
    /// # 参数
    /// * `data` - 要编码的数据
    /// * `size` - 图像尺寸 (可选)
    ///
    /// # 返回
    /// 返回Base64编码的PNG图像数据
    pub async fn generate_base64(&self, data: &str, size: Option<u32>) -> Result<String> {
        let image_data = self.generate_image(data, size).await?;
        Ok(general_purpose::STANDARD.encode(&image_data))
    }
}

/// 便捷函数：生成二维码图像
///
/// # 参数
/// * `data` - 要编码的数据
/// * `size` - 图像尺寸 (可选)
///
/// # 返回
/// 返回RGB图像数据的字节数组
pub async fn generate_qr_image(data: &str, size: Option<u32>) -> Result<Vec<u8>> {
    QRGenerator::new().generate_image(data, size).await
}

/// 便捷函数：生成二维码到文件
///
/// # 参数
/// * `data` - 要编码的数据
/// * `path` - 保存路径
/// * `size` - 图像尺寸 (可选)
///
/// # 返回
/// 返回保存的文件大小
pub fn generate_qr_to_file(data: &str, path: &str, size: Option<u32>) -> Result<u64> {
    QRGenerator::new().generate_to_file(data, path, size)
}

/// 便捷函数：生成SVG格式二维码
///
/// # 参数
/// * `data` - 要编码的数据
/// * `size` - 图像尺寸 (可选)
///
/// # 返回
/// 返回SVG字符串
pub fn generate_qr_svg(data: &str, size: Option<u32>) -> Result<String> {
    QRGenerator::new().generate_svg(data, size)
}

/// 便捷函数：生成Base64编码二维码
///
/// # 参数
/// * `data` - 要编码的数据
/// * `size` - 图像尺寸 (可选)
///
/// # 返回
/// 返回Base64编码的PNG图像数据
pub async  fn generate_qr_base64(data: &str, size: Option<u32>) -> Result<String> {
    QRGenerator::new().generate_base64(data, size).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::UploadConfig;
    use crate::utils::FileUploadService;

    #[tokio::test]
    async fn test_generate_qr_to_file() {
        let config = UploadConfig {
            max_file_size: 5 * 1024 * 1024, // 5MB
            allowed_extensions: vec![
                "jpg".to_string(),
                "jpeg".to_string(),
                "png".to_string(),
                "gif".to_string(),
                "webp".to_string(),
            ],
            upload_path: "assets".to_string(),
            qrcord_size: 200,
        };
        let upload_service = FileUploadService::new(&config);
        let data = "data";
        let base_data = generate_qr_image(data, Some(100)).await.unwrap();

        let res = upload_service
            .process_file_upload("./test.png", "qrcode", &base_data.as_slice())
            .await;

        match res {
            Ok(r) => println!("{:#?}", r),
            Err(e) => println!("{:#?}", e),
        };
    }



    #[test]
    fn test_generate_qr_svg() {
        let result = generate_qr_svg("Hello, World!", Some(50));
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }


}
