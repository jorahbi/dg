use crate::{error::Result, extract::AuthUser, schema::common::ApiResponse, state::AppState};
use axum::{
    extract::{Multipart, State},
    response::{IntoResponse, Json},
};
use serde_json::json;

// 获取KYC状态
pub async fn get_kyc_status(
    State(_state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let kyc_status = json!({
        "userId": auth_user.id,
        "status": "pending",
        "verificationLevel": 0,
        "submittedAt": null,
        "reviewedAt": null,
        "rejectedReason": null,
        "documents": {
            "idCard": {
                "status": "not_submitted",
                "frontUrl": null,
                "backUrl": null
            },
            "passport": {
                "status": "not_submitted",
                "frontUrl": null
            },
            "selfie": {
                "status": "not_submitted",
                "url": null
            }
        },
        "personalInfo": {
            "realName": null,
            "idNumber": null,
            "nationality": null,
            "birthDate": null
        }
    });

    let response = ApiResponse::success(kyc_status);
    Ok(Json(response))
}

// 上传身份证件
pub async fn upload_id_card(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let upload_result = json!({
        "userId": _auth_user.id,
        "documentId": format!("DOC{}", chrono::Utc::now().timestamp()),
        "documentType": payload.get("documentType"),
        "frontImageUrl": payload.get("frontImageUrl"),
        "backImageUrl": payload.get("backImageUrl"),
        "status": "uploaded",
        "uploadedAt": chrono::Utc::now().to_rfc3339(),
        "message": "证件上传成功"
    });

    let response = ApiResponse::success_with_message(upload_result, "Document uploaded successfully");
    Ok(Json(response))
}

// 提交KYC申请
pub async fn submit_kyc(
    State(_state): State<AppState>,
    auth_user: AuthUser,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    let mut personal_info = json!({});
    let mut files = std::collections::HashMap::new();

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();

        if name == "personalInfo" {
            let data = field.text().await?;
            personal_info = serde_json::from_str(&data).unwrap_or(json!({}));
        } else if name.starts_with("file_") {
            let filename = field.file_name().unwrap_or("upload.jpg").to_string();

            let _data = field.bytes().await?;

            // 模拟文件上传处理
            let file_url = format!(
                "https://example.com/uploads/kyc/{}/{}",
                auth_user.id, filename
            );
            files.insert(name, file_url);
        }
    }

    let submission_result = json!({
        "userId": auth_user.id,
        "status": "submitted",
        "submittedAt": chrono::Utc::now().to_rfc3339(),
        "applicationId": format!("KYC{}", chrono::Utc::now().timestamp()),
        "estimatedReviewTime": "24-48小时",
        "documents": {
            "idCard": {
                "status": "submitted",
                "frontUrl": files.get("file_id_card_front"),
                "backUrl": files.get("file_id_card_back")
            },
            "passport": {
                "status": files.get("file_passport_front")
                    .map(|_| "submitted")
                    .unwrap_or("not_required"),
                "frontUrl": files.get("file_passport_front")
            },
            "selfie": {
                "status": files.get("file_selfie")
                    .map(|_| "submitted")
                    .unwrap_or("not_required"),
                "url": files.get("file_selfie")
            }
        },
        "personalInfo": personal_info
    });

    let response =
        ApiResponse::success_with_message(submission_result, "KYC application submitted, please wait for review");
    Ok(Json(response))
}

// 获取KYC文档类型
pub async fn get_document_types(State(_state): State<AppState>) -> Result<impl IntoResponse> {
    let document_types = json!({
        "idCard": {
            "name": "身份证",
            "description": "请上传身份证正反面照片",
            "requiredFiles": ["front", "back"],
            "acceptedFormats": ["jpg", "jpeg", "png"],
            "maxFileSize": "5MB",
            "minResolution": {"width": 800, "height": 600}
        },
        "passport": {
            "name": "护照",
            "description": "请上传护照个人信息页照片",
            "requiredFiles": ["front"],
            "acceptedFormats": ["jpg", "jpeg", "png"],
            "maxFileSize": "5MB",
            "minResolution": {"width": 800, "height": 600}
        },
        "selfie": {
            "name": "自拍照片",
            "description": "请上传手持身份证的自拍照片",
            "requiredFiles": ["selfie"],
            "acceptedFormats": ["jpg", "jpeg", "png"],
            "maxFileSize": "5MB",
            "minResolution": {"width": 600, "height": 800}
        }
    });

    let response = ApiResponse::success(document_types);
    Ok(Json(response))
}

// 重新提交KYC申请
pub async fn resubmit_kyc_application(
    State(_state): State<AppState>,
    auth_user: AuthUser,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    let mut files = std::collections::HashMap::new();

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();

        if name.starts_with("file_") {
            let filename = field.file_name().unwrap_or("upload.jpg").to_string();

            let _data = field.bytes().await?;

            // 模拟文件上传处理
            let file_url = format!(
                "https://example.com/uploads/kyc/{}/{}",
                auth_user.id, filename
            );
            files.insert(name, file_url);
        }
    }

    let resubmission_result = json!({
        "userId": auth_user.id,
        "status": "resubmitted",
        "resubmittedAt": chrono::Utc::now().to_rfc3339(),
        "applicationId": format!("KYC{}", chrono::Utc::now().timestamp()),
        "estimatedReviewTime": "24-48小时",
        "documents": {
            "idCard": {
                "status": "resubmitted",
                "frontUrl": files.get("file_id_card_front"),
                "backUrl": files.get("file_id_card_back")
            },
            "passport": {
                "status": files.get("file_passport_front")
                    .map(|_| "resubmitted")
                    .unwrap_or("not_required"),
                "frontUrl": files.get("file_passport_front")
            },
            "selfie": {
                "status": files.get("file_selfie")
                    .map(|_| "resubmitted")
                    .unwrap_or("not_required"),
                "url": files.get("file_selfie")
            }
        }
    });

    let response =
        ApiResponse::success_with_message(resubmission_result, "KYC application resubmitted, please wait for review");
    Ok(Json(response))
}

// 获取支持的国籍列表
pub async fn get_supported_nationalities(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse> {
    let nationalities = json!({
        "countries": [
            {"code": "CN", "name": "中国", "nameEn": "China"},
            {"code": "US", "name": "美国", "nameEn": "United States"},
            {"code": "UK", "name": "英国", "nameEn": "United Kingdom"},
            {"code": "JP", "name": "日本", "nameEn": "Japan"},
            {"code": "KR", "name": "韩国", "nameEn": "South Korea"},
            {"code": "SG", "name": "新加坡", "nameEn": "Singapore"},
            {"code": "HK", "name": "香港", "nameEn": "Hong Kong"},
            {"code": "TW", "name": "台湾", "nameEn": "Taiwan"},
            {"code": "CA", "name": "加拿大", "nameEn": "Canada"},
            {"code": "AU", "name": "澳大利亚", "nameEn": "Australia"},
            {"code": "DE", "name": "德国", "nameEn": "Germany"},
            {"code": "FR", "name": "法国", "nameEn": "France"},
            {"code": "IT", "name": "意大利", "nameEn": "Italy"},
            {"code": "ES", "name": "西班牙", "nameEn": "Spain"},
            {"code": "NL", "name": "荷兰", "nameEn": "Netherlands"}
        ]
    });

    let response = ApiResponse::success(nationalities);
    Ok(Json(response))
}
