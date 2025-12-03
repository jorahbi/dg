use axum::{
    extract::{State, Query},
    response::{IntoResponse, Json},
};
use serde_json::json;
use crate::{
    schema::common::ApiResponse,
    state::AppState,
    error::Result,
};

// 获取关于我们信息
pub async fn get_about_us(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse> {
    let about_us = json!({
        "company": {
            "name": "Astra Ai",
            "nameEn": "Astra Ai Technology Ltd.",
            "logo": "https://example.com/logo.png",
            "establishedAt": "2024-01-01",
            "description": "Astra Ai是一家专注于人工智能算力服务的创新科技公司，致力于为全球用户提供高效、稳定、安全的AI算力挖矿解决方案。",
            "vision": "成为全球领先的AI算力服务平台，推动人工智能技术的普及和发展。",
            "mission": "通过创新的区块链技术和AI算法，为用户提供优质的算力服务，实现价值共创。"
        },
        "statistics": {
            "totalUsers": 158650,
            "totalHashrate": 45680.5,
            "totalRevenue": 258000000.0,
            "globalNodes": 1250,
            "countriesCovered": 45,
            "operatingDays": 687
        },
        "highlights": [
            {
                "title": "技术领先",
                "description": "采用最先进的AI算法和区块链技术",
                "icon": "https://example.com/icons/tech.svg"
            },
            {
                "title": "安全可靠",
                "description": "多重安全防护，保障用户资产安全",
                "icon": "https://example.com/icons/security.svg"
            },
            {
                "title": "全球布局",
                "description": "节点遍布全球45个国家和地区",
                "icon": "https://example.com/icons/global.svg"
            },
            {
                "title": "专业服务",
                "description": "7x24小时专业客服支持",
                "icon": "https://example.com/icons/service.svg"
            }
        ],
        "coreValues": [
            {
                "title": "创新",
                "description": "持续技术创新，引领行业发展"
            },
            {
                "title": "诚信",
                "description": "诚信经营，透明公开"
            },
            {
                "title": "共赢",
                "description": "与用户共同成长，实现价值共赢"
            },
            {
                "title": "责任",
                "description": "承担社会责任，推动可持续发展"
            }
        ]
    });

    let response = ApiResponse::success(about_us);
    Ok(Json(response))
}

// 获取团队成员信息
pub async fn get_team(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse> {
    let team = json!({
        "leadership": [
            {
                "id": 1,
                "name": "张明",
                "position": "创始人 & CEO",
                "avatar": "https://example.com/team/ceo.jpg",
                "bio": "拥有15年AI和区块链行业经验，曾任多家知名科技公司高管，清华大学计算机科学博士。",
                "linkedin": "https://linkedin.com/in/example",
                "achievements": [
                    "2023年度AI创新人物",
                    "区块链技术专家",
                    "清华大学客座教授"
                ]
            },
            {
                "id": 2,
                "name": "李婷婷",
                "position": "联合创始人 & CTO",
                "avatar": "https://example.com/team/cto.jpg",
                "bio": "专注于分布式系统和AI算法研究12年，MIT计算机科学硕士，曾任Google高级工程师。",
                "linkedin": "https://linkedin.com/in/example",
                "achievements": [
                    "分布式系统专家",
                    "AI算法专利持有者",
                    "技术团队管理专家"
                ]
            },
            {
                "id": 3,
                "name": "王强",
                "position": "CFO",
                "avatar": "https://example.com/team/cfo.jpg",
                "bio": "资深金融专家，拥有10年区块链投资和管理经验，CFA持证人，北京大学金融学硕士。",
                "linkedin": "https://linkedin.com/in/example",
                "achievements": [
                    "CFA特许金融分析师",
                    "区块链投资专家",
                    "金融风险管理师"
                ]
            }
        ],
        "advisors": [
            {
                "id": 4,
                "name": "陈教授",
                "position": "技术顾问",
                "avatar": "https://example.com/team/advisor1.jpg",
                "bio": "中国科学院院士，人工智能领域权威专家，在深度学习和神经网络方面有深厚造诣。"
            },
            {
                "id": 5,
                "name": "Dr. Smith",
                "position": "国际顾问",
                "avatar": "https://example.com/team/advisor2.jpg",
                "bio": "前Microsoft AI研究院院长，在机器学习和云计算领域拥有20年经验。"
            }
        ],
        "teamSize": 85,
        "departments": [
            {"name": "技术研发部", "size": 35, "percentage": 41.2},
            {"name": "产品运营部", "size": 20, "percentage": 23.5},
            {"name": "市场销售部", "size": 15, "percentage": 17.6},
            {"name": "客户服务部", "size": 10, "percentage": 11.8},
            {"name": "行政管理部", "size": 5, "percentage": 5.9}
        ]
    });

    let response = ApiResponse::success(team);
    Ok(Json(response))
}

// 获取发展历程
pub async fn get_timeline(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse> {
    let timeline = json!({
        "milestones": [
            {
                "date": "2024-01-01",
                "title": "公司成立",
                "description": "Astra Ai在香港正式成立，获得天使轮融资500万美元",
                "type": "founding",
                "icon": "https://example.com/icons/founding.svg"
            },
            {
                "date": "2024-03-15",
                "title": "技术团队组建",
                "description": "核心技术团队组建完成，开始AI算力算法研发",
                "type": "team",
                "icon": "https://example.com/icons/team.svg"
            },
            {
                "date": "2024-06-20",
                "title": "产品原型发布",
                "description": "首个AI算力挖矿产品原型发布，开始内部测试",
                "type": "product",
                "icon": "https://example.com/icons/product.svg"
            },
            {
                "date": "2024-09-01",
                "title": "Beta版本上线",
                "description": "平台Beta版本正式上线，首批注册用户突破10000人",
                "type": "launch",
                "icon": "https://example.com/icons/launch.svg"
            },
            {
                "date": "2024-11-10",
                "title": "A轮融资完成",
                "description": "完成A轮融资2000万美元，估值达到1亿美元",
                "type": "funding",
                "icon": "https://example.com/icons/funding.svg"
            },
            {
                "date": "2024-12-01",
                "title": "正式版上线",
                "description": "平台正式版上线，全球用户突破50000人",
                "type": "milestone",
                "icon": "https://example.com/icons/milestone.svg"
            },
            {
                "date": "2025-03-15",
                "title": "全球化布局",
                "description": "在全球45个国家设立节点，算力规模达到25000 TH/s",
                "type": "global",
                "icon": "https://example.com/icons/global.svg"
            },
            {
                "date": "2025-06-01",
                "title": "用户突破10万",
                "description": "全球注册用户突破10万，日活跃用户超过20000人",
                "type": "milestone",
                "icon": "https://example.com/icons/users.svg"
            }
        ],
        "achievements": {
            "totalFunding": 25000000.0,
            "totalUsers": 158650,
            "globalNodes": 1250,
            "countriesCovered": 45,
            "totalHashrate": 45680.5,
            "operatingDays": 687
        }
    });

    let response = ApiResponse::success(timeline);
    Ok(Json(response))
}

// 获取合作伙伴
pub async fn get_partners(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse> {
    let partners = json!({
        "strategicPartners": [
            {
                "id": 1,
                "name": "Microsoft",
                "logo": "https://example.com/partners/microsoft.png",
                "description": "云计算服务战略合作",
                "website": "https://microsoft.com",
                "cooperationLevel": "strategic"
            },
            {
                "id": 2,
                "name": "Google Cloud",
                "logo": "https://example.com/partners/google.png",
                "description": "AI基础设施合作",
                "website": "https://cloud.google.com",
                "cooperationLevel": "strategic"
            },
            {
                "id": 3,
                "name": "Amazon Web Services",
                "logo": "https://example.com/partners/aws.png",
                "description": "全球云服务合作",
                "website": "https://aws.amazon.com",
                "cooperationLevel": "strategic"
            }
        ],
        "technologyPartners": [
            {
                "id": 4,
                "name": "NVIDIA",
                "logo": "https://example.com/partners/nvidia.png",
                "description": "GPU硬件供应商",
                "website": "https://nvidia.com",
                "cooperationLevel": "technology"
            },
            {
                "id": 5,
                "name": "Intel",
                "logo": "https://example.com/partners/intel.png",
                "description": "芯片技术合作",
                "website": "https://intel.com",
                "cooperationLevel": "technology"
            }
        ],
        "financialPartners": [
            {
                "id": 6,
                "name": "Sequoia Capital",
                "logo": "https://example.com/partners/sequoia.png",
                "description": "投资合作伙伴",
                "website": "https://sequoiacap.com",
                "cooperationLevel": "financial"
            },
            {
                "id": 7,
                "name": "Andreessen Horowitz",
                "logo": "https://example.com/partners/a16z.png",
                "description": "风险投资合作",
                "website": "https://a16z.com",
                "cooperationLevel": "financial"
            }
        ],
        "totalPartners": 28,
        "partnershipLevels": [
            {"level": "strategic", "count": 8, "name": "战略合作伙伴"},
            {"level": "technology", "count": 12, "name": "技术合作伙伴"},
            {"level": "financial", "count": 5, "name": "金融合作伙伴"},
            {"level": "regional", "count": 3, "name": "区域合作伙伴"}
        ]
    });

    let response = ApiResponse::success(partners);
    Ok(Json(response))
}

// 获取联系方式
pub async fn get_contact_info(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse> {
    let contact_info = json!({
        "headquarters": {
            "address": "香港中环金融街8号国际金融中心72楼",
            "addressEn": "72/F, International Finance Centre, 8 Harcourt Road, Central, Hong Kong",
            "phone": "+852-1234-5678",
            "email": "contact@astrai.com",
            "workingHours": "周一至周五 9:00-18:00 (UTC+8)",
            "coordinates": {"lat": 22.2783, "lng": 114.1747}
        },
        "offices": [
            {
                "city": "北京",
                "address": "北京市朝阳区建国门外大街1号国贸大厦A座2801",
                "phone": "+86-10-8888-9999",
                "email": "beijing@astrai.com"
            },
            {
                "city": "上海",
                "address": "上海市浦东新区陆家嘴环路1000号恒生银行大厦35楼",
                "phone": "+86-21-6666-8888",
                "email": "shanghai@astrai.com"
            },
            {
                "city": "深圳",
                "address": "深圳市南山区科技园南区深南大道9988号",
                "phone": "+86-755-8888-7777",
                "email": "shenzhen@astrai.com"
            },
            {
                "city": "新加坡",
                "address": "1 Raffles Place, #50-01, One Raffles Place, Singapore 048616",
                "phone": "+65-6888-9999",
                "email": "singapore@astrai.com"
            }
        ],
        "customerService": {
            "hotline": "+852-8888-9999",
            "email": "support@astrai.com",
            "wechat": "AstraAi_Service",
            "telegram": "@AstraAi_Support",
            "workingHours": "7x24小时在线服务"
        },
        "socialMedia": {
            "website": "https://astrai.com",
            "twitter": "https://twitter.com/astrai_official",
            "facebook": "https://facebook.com/astrai.official",
            "linkedin": "https://linkedin.com/company/astrai",
            "youtube": "https://youtube.com/c/astrai",
            "instagram": "https://instagram.com/astrai_official",
            "reddit": "https://reddit.com/r/astrai",
            "discord": "https://discord.gg/astrai"
        },
        "businessCooperation": {
            "email": "business@astrai.com",
            "phone": "+852-9999-8888",
            "description": "商务合作、技术合作、投资合作等商务事宜请联系我们"
        },
        "mediaRelations": {
            "email": "media@astrai.com",
            "phone": "+852-7777-9999",
            "description": "媒体采访、新闻发布、品牌合作等媒体事宜请联系我们"
        }
    });

    let response = ApiResponse::success(contact_info);
    Ok(Json(response))
}

// 获取常见问题
pub async fn get_faq(
    State(_state): State<AppState>,
    Query(params): Query<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let category = params.get("category").and_then(|v| v.as_str()).unwrap_or("general");

    let faq_data = match category {
        "account" => json!({
            "category": "账户相关",
            "questions": [
                {
                    "id": 1,
                    "question": "如何注册Astra Ai账户？",
                    "answer": "您可以通过我们的官方网站或手机应用进行注册。只需提供手机号码或邮箱地址，设置安全密码，完成验证即可。整个注册过程只需2-3分钟。",
                    "tags": ["注册", "账户", "新手"]
                },
                {
                    "id": 2,
                    "question": "忘记密码怎么办？",
                    "answer": "您可以通过注册时使用的手机号或邮箱进行密码重置。在登录页面点击'忘记密码'，按照指引完成身份验证后即可设置新密码。",
                    "tags": ["密码", "重置", "安全"]
                },
                {
                    "id": 3,
                    "question": "如何提高账户安全等级？",
                    "answer": "建议您：1）设置复杂密码并定期更换；2）开启双因素认证；3）完成KYC身份认证；4）不要在公共设备上登录账户。",
                    "tags": ["安全", "KYC", "2FA"]
                }
            ]
        }),
        "mining" => json!({
            "category": "挖矿相关",
            "questions": [
                {
                    "id": 4,
                    "question": "什么是AI算力挖矿？",
                    "answer": "AI算力挖矿是利用人工智能算法进行算力资源优化，通过分布式网络为AI模型训练提供计算资源，并从中获得收益的新型挖矿方式。",
                    "tags": ["AI挖矿", "算力", "概念"]
                },
                {
                    "id": 5,
                    "question": "收益是如何计算的？",
                    "answer": "收益基于您购买的算力大小、当前网络难度、AI模型需求等因素计算。系统会实时计算您的贡献度，并按比例分配收益。",
                    "tags": ["收益", "计算", "分配"]
                },
                {
                    "id": 6,
                    "question": "最低投资金额是多少？",
                    "answer": "我们的算力套餐起投金额为100 USDT，适合不同层级的投资者。您也可以根据自己的情况选择更高金额的套餐。",
                    "tags": ["投资", "门槛", "套餐"]
                }
            ]
        }),
        "withdrawal" => json!({
            "category": "提现相关",
            "questions": [
                {
                    "id": 7,
                    "question": "如何申请提现？",
                    "answer": "在资产页面选择提现，输入提现金额和收款地址，确认后提交申请。我们会在24小时内处理您的提现请求。",
                    "tags": ["提现", "流程", "申请"]
                },
                {
                    "id": 8,
                    "question": "提现手续费是多少？",
                    "answer": "提现手续费根据区块链网络和提现金额有所不同，一般在1-5 USDT之间。VIP会员可享受手续费折扣优惠。",
                    "tags": ["手续费", "费用", "VIP"]
                },
                {
                    "id": 9,
                    "question": "提现到账时间是多久？",
                    "answer": "一般情况下，USDT-TRC20提现5-30分钟到账，USDT-ERC20提现15-60分钟到账。具体时间取决于区块链网络拥堵情况。",
                    "tags": ["到账", "时间", "网络"]
                }
            ]
        }),
        _ => json!({
            "category": "常见问题",
            "questions": [
                {
                    "id": 1,
                    "question": "Astra Ai是什么？",
                    "answer": "Astra Ai是一个专业的AI算力挖矿平台，为用户提供稳定、高效的AI算力投资服务。我们通过先进的AI算法和分布式技术，帮助用户获得可观的算力收益。",
                    "tags": ["介绍", "平台", "服务"]
                },
                {
                    "id": 2,
                    "question": "如何开始使用Astra Ai？",
                    "answer": "1）注册账户并完成身份认证；2）选择适合的算力套餐；3）完成支付；4）开始享受算力收益。整个过程简单快捷，我们的客服团队随时为您提供帮助。",
                    "tags": ["新手", "流程", "教程"]
                },
                {
                    "id": 3,
                    "question": "平台是否安全可靠？",
                    "answer": "是的，我们采用多重安全防护措施，包括SSL加密、双因素认证、冷钱包存储等。同时，我们在香港合法注册，接受相关监管，确保用户资产安全。",
                    "tags": ["安全", "监管", "保障"]
                }
            ]
        })
    };

    let response = ApiResponse::success(faq_data);
    Ok(Json(response))
}