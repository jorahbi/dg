# Coin项目API接口文档

> 更新时间：2025-11-23 17:59:00
> 版本：1.0.0
> API版本：v1
> 基础URL：`https://api.example.com`

## 接口概览

Coin项目提供完整的RESTful API接口，支持用户认证、算力管理、空投活动、资产管理等核心功能。

## 基础信息

### HTTP请求方法
- `GET`: 获取资源
- `POST`: 创建资源
- `PUT`: 更新资源
- `DELETE`: 删除资源

### 通用响应格式
```json
{
  "code": 200,
  "message": "success",
  "data": {},
  "timestamp": "2025-11-23T17:59:00.000Z"
}
```

### 错误响应格式
```json
{
  "success": false,
  "code": "ERROR_CODE",
  "message": "错误描述",
  "statusCode": 400,
  "timestamp": "2025-11-23T17:59:00.000Z"
}
```

### 请求头要求
```http
Content-Type: application/json
Authorization: Bearer {token}
Accept: application/json
```

---

## 1. 认证管理模块

### 1.1 用户注册

**接口地址**: `POST /api/auth/register`

**请求参数**:
```json
{
  "username": "string",
  "password": "string",
  "confirmPassword": "string"
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "注册成功",
  "data": {
    "userId": "user_1234567890",
    "username": "string",
    "message": "注册成功，请设置密保问题"
  }
}
```

**错误码**:
- `400`: 参数错误
- `409`: 用户名已存在 (USERNAME_EXISTS)
- `500`: 服务器内部错误 (REGISTER_ERROR)

---

### 1.2 用户登录

**接口地址**: `POST /api/auth/login`

**请求参数**:
```json
{
  "username": "string",
  "password": "string"
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "登录成功",
  "data": {
    "username": "string",
    "email": "string",
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "createdAt": "2025-11-23T17:59:00.000Z",
    "isEmailVerified": true,
    "hasSecurityQuestions": false,
    "permissions": ["read", "write"]
  }
}
```

**测试账号**:
- `demo` / `demo12345`
- `user` / `password123`

**错误码**:
- `400`: 参数错误
- `401`: 用户名或密码错误
- `404`: 用户不存在
- `423`: 账户被锁定

---

### 1.3 获取安全问题列表

**接口地址**: `GET /api/auth/security-questions`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": [
    {
      "id": 1,
      "question": "你的宠物叫什么名字？"
    },
    {
      "id": 2,
      "question": "你的出生城市是哪里？"
    }
  ]
}
```

---

### 1.4 保存安全问题答案

**接口地址**: `POST /api/auth/security-questions`

**请求参数**:
```json
{
  "username": "string",
  "questions": [
    {
      "questionId": 1,
      "answer": "string"
    }
  ]
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "保存成功",
  "data": {
    "success": true,
    "username": "string"
  }
}
```

---

### 1.5 检查密保问题设置状态

**接口地址**: `GET /api/auth/security-questions/check`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "username": "string",
    "hasSecurityQuestions": true,
    "questionsCount": 3,
    "message": "用户已设置密保问题"
  }
}
```

---

### 1.6 忘记密码

#### 1.6.1 获取用户密保问题

**接口地址**: `POST /api/auth/forgot-password/questions`

**请求参数**:
```json
{
  "username": "string"
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "username": "string",
    "questions": [
      {
        "id": 1,
        "question": "你的宠物叫什么名字？"
      }
    ]
  }
}
```

---

#### 1.6.2 验证密保问题答案

**接口地址**: `POST /api/auth/forgot-password/verify`

**请求参数**:
```json
{
  "username": "string",
  "answers": [
    {
      "questionId": 1,
      "answer": "string"
    }
  ]
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "验证成功",
  "data": {
    "verified": true,
    "username": "string",
    "token": "reset_token_1234567890"
  }
}
```

---

#### 1.6.3 重置密码

**接口地址**: `POST /api/auth/forgot-password/reset`

**请求参数**:
```json
{
  "username": "string",
  "newPassword": "string",
  "resetToken": "string"
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "密码重置成功",
  "data": {
    "success": true
  }
}
```

---

### 1.7 用户登出

**接口地址**: `POST /api/auth/logout`

**请求参数**:
```json
{
  "token": "string"
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "登出成功",
  "data": {
    "success": true
  }
}
```

---

## 2. 用户管理模块

### 2.1 获取用户信息

**接口地址**: `GET /api/user/info`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "username": "string",
    "totalAssets": 147193,
    "dgAmount": 14719,
    "userLevel": 3,
    "isKycVerified": false,
    "isLoggedIn": true,
    "qrCodeUrl": "https://api.example.com/qrcode/123456",
    "inviteCode": "INVITE123"
  }
}
```

---

### 2.2 获取用户资料

**接口地址**: `GET /api/user/profile`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "avatarUrl": "https://api.example.com/avatar/123456",
    "securityQuestions": [
      {
        "id": 1,
        "question": "你的生日是？"
      }
    ]
  }
}
```

---

### 2.3 更新头像

**接口地址**: `POST /api/user/profile/avatar`

**请求参数**: `multipart/form-data`
- `avatar`: 头像文件

**响应示例**:
```json
{
  "code": 200,
  "message": "头像更新成功",
  "data": {
    "avatarUrl": "https://api.example.com/avatar/123456_new",
    "uploadTime": "2025-11-23T17:59:00.000Z"
  }
}
```

---

### 2.4 修改密码

**接口地址**: `POST /api/user/profile/password`

**请求参数**:
```json
{
  "currentPassword": "string",
  "newPassword": "string"
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "密码修改成功",
  "data": {
    "success": true,
    "updateTime": "2025-11-23T17:59:00.000Z"
  }
}
```

---

## 3. 算力管理模块

### 3.1 获取用户Power信息

**接口地址**: `GET /api/power/info`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "currentLevel": 7,
    "totalEarnings": "1,250.00 USDT",
    "availableBalance": "150.00 USDT",
    "teamRewards": "850.00 USDT",
    "currentInvites": 18,
    "requiredInvites": 25,
    "levels": [
      {
        "id": 0,
        "name": "LV.0",
        "isUnlocked": true,
        "isCurrent": false,
        "requiredInvites": 0
      }
    ]
  }
}
```

---

### 3.2 获取用户等级列表

**接口地址**: `GET /api/power/levels`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": [
    {
      "id": 0,
      "name": "LV.0",
      "description": "初始等级",
      "requiredInvites": 0,
      "rewardMultiplier": 1.0,
      "isUnlocked": true,
      "isCurrent": false
    },
    {
      "id": 1,
      "name": "LV.1",
      "description": "基础等级",
      "requiredInvites": 5,
      "rewardMultiplier": 1.1,
      "isUnlocked": true,
      "isCurrent": true
    }
  ]
}
```

---

### 3.3 获取算力记录

**接口地址**: `GET /api/power/records`

**查询参数**:
- `page`: 页码 (默认: 1)
- `limit`: 每页数量 (默认: 20)
- `powerType`: 算力类型 (可选)
- `status`: 状态 (可选)

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "records": [
      {
        "id": 1,
        "amount": 1500.00,
        "title": '挖矿算力V1',
        "description": '高效挖矿算力包',
        "status": 'active',
        "dailyYieldPercentage": 0.01, // 2.5%
        "lv": 1,
      }
      
    ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 1,
      "totalPages": 1
    }
  }
}
```

---

### 3.4 升级等级

**接口地址**: `POST /api/power/upgrade`

**请求参数**:
```json
{
  "targetLevel": 8
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "升级成功",
  "data": {
    "newLevel": 8,
    "upgradeTime": "2025-11-23T17:59:00.000Z",
    "rewardMultiplier": 1.8
  }
}
```

---

### 3.5 提交提现请求

**接口地址**: `POST /api/power/withdraw`

**请求参数**:
```json
{
  "amount": "100.00",
  "currency": "USDT",
  "destinationAddress": "TRX742d35Cc6634C0532925a3b844Bc454e4438f33e"
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "提现申请提交成功",
  "data": {
    "withdrawalId": "withdrawal_1234567890",
    "amount": 100.00,
    "currency": "USDT",
    "status": "pending",
    "submitTime": "2025-11-23T17:59:00.000Z",
    "estimatedProcessingTime": "24小时"
  }
}
```

---

### 3.6 获取提现详情

**接口地址**: `GET /api/power/withdrawals/{withdrawalId}`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "id": "withdrawal_1234567890",
    "amount": 100.00,
    "currency": "USDT",
    "destinationAddress": "TRX742d35Cc6634C0532925a3b844Bc454e4438f33e",
    "status": "completed",
    "submitTime": "2025-11-23T17:59:00.000Z",
    "processedTime": "2025-11-23T18:00:00.000Z",
    "transactionHash": "0x1234567890abcdef",
    "fee": 1.00
  }
}
```

---

### 3.7 取消提现请求

**接口地址**: `POST /api/power/withdrawals/{withdrawalId}/cancel`

**响应示例**:
```json
{
  "code": 200,
  "message": "提现请求已取消",
  "data": {
    "withdrawalId": "withdrawal_1234567890",
    "status": "cancelled",
    "cancelTime": "2025-11-23T17:59:00.000Z"
  }
}
```

---

## 4. 购买管理模块

### 4.1 获取算力详情

**接口地址**: `GET /api/power/{powerId}`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "id": 1,
    "taskType": "AI智能计算",
    "requiredLevel": 1,
    "earningsPercent": 5.8,
    "amount": 150.00,
    "currency": "USDT",
    "description": "为您提供稳定高效的AI算力服务",
    "duration": "30天",
    "features": [
      "高性能AI算力",
      "稳定收益",
      "自动复投"
    ],
    "dailyEarnings": 8.70,
    "totalEarnings": 261.00
  }
}
```

---

### 4.2 提交购买订单

**接口地址**: `POST /api/purchase/order`

**请求参数**:
```json
{
  "powerId": 1,
  "quantity": 1
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "订单创建成功",
  "data": {
    "orderId": "ORD_1234567890",
    "orderNumber": "ORD001",
    "blockchainType": "USDT (TRC-20)",
    "blockchainAddress": "TRX742d35Cc6634C0532925a3b844Bc454e4438f33e",
    "amount": 150.00,
    "transactionHash": null,
    "isPaid": false,
    "createdAt": "2025-11-23T17:59:00.000Z",
    "expiredAt": "2025-11-23T18:59:00.000Z"
  }
}
```

---

### 4.3 获取订单详情

**接口地址**: `GET /api/purchase/orders/{orderId}`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "id": "ORD_1234567890",
    "orderNumber": "ORD001",
    "powerId": 1,
    "powerName": "AI智能计算",
    "quantity": 1,
    "amount": 150.00,
    "currency": "USDT",
    "blockchainType": "USDT (TRC-20)",
    "blockchainAddress": "TRX742d35Cc6634C0532925a3b844Bc454e4438f33e",
    "transactionHash": "0x1234567890abcdef",
    "isPaid": true,
    "paidAt": "2025-11-23T18:00:00.000Z",
    "createdAt": "2025-11-23T17:59:00.000Z",
    "expiredAt": "2025-11-23T18:59:00.000Z",
    "status": "completed"
  }
}
```

---

### 4.4 取消订单

**接口地址**: `POST /api/purchase/orders/{orderId}/cancel`

**响应示例**:
```json
{
  "code": 200,
  "message": "订单已取消",
  "data": {
    "orderId": "ORD_1234567890",
    "status": "cancelled",
    "cancelTime": "2025-11-23T17:59:00.000Z",
    "refundAmount": 150.00
  }
}
```

---

## 5. 收益管理模块

### 5.1 获取收益数据

**接口地址**: `GET /api/earnings`

**查询参数**:
- `start_date`: 开始日期 (YYYY-MM-DD)
- `end_date`: 结束日期 (YYYY-MM-DD)
- `page`: 页码 (默认: 1)
- `limit`: 每页数量 (默认: 20)

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "earnings": [
      {
        "id": 1,
        "date": "2025-11-23",
        "amount": 156.5,
        "source": {
          "id": "mining",
          "name": "挖矿收益",
          "color": "#4ECDC4"
        },
        "status": "confirmed",
        "description": "算力挖矿收益 156.50 DG - 已确认"
      }
    ],
    "summary": {
      "totalAmount": 1587.5,
      "todayAmount": 181.5,
      "weekAmount": 234.8,
      "monthAmount": 892.3,
      "totalCount": 25,
      "statusCounts": {
        "confirmed": 20,
        "pending": 3,
        "failed": 1,
        "cancelled": 1
      },
      "sourceAmounts": {
        "mining": 892.5,
        "referral": 234.0,
        "task": 156.3,
        "staking": 189.7,
        "airdrop": 115.0
      }
    },
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 25,
      "totalPages": 2
    }
  }
}
```

---

## 6. 空投活动模块

### 6.1 获取空投列表

**接口地址**: `GET /api/airdrops`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": [
    {
      "id": 1,
      "type": "daily",
      "title": "每日空投",
      "subtitle": "所有人可参与",
      "description": "每天定时抢空投，获得随机DG奖励",
      "activityStartTime": "2025-11-23T00:00:00.000Z",
      "activityEndTime": "2025-11-23T23:59:59.000Z",
      "totalRounds": 999,
      "roundDuration": 60,
      "intervalDuration": 20,
      "participationType": {
        "runtimeType": "allUsers"
      },
      "color": "0xFF4ECDC4",
      "status": "active",
      "participantCount": 15420,
      "successRate": 0.856
    },
    {
      "id": 2,
      "type": "vip",
      "title": "会员专属空投",
      "subtitle": "VIP会员专享",
      "participationType": {
        "runtimeType": "memberLevel",
        "requiredLevel": 3
      }
    }
  ]
}
```

---

### 6.2 抢空投

**接口地址**: `POST /api/airdrops/claim`

**请求参数**:
```json
{
  "airdropId": 1,
  "currentRound": 1
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "恭喜成功抢到空投！",
  "data": {
    "success": true,
    "status": "success",
    "dgAmount": 125,
    "message": "恭喜成功抢到空投！",
    "transactionId": "txn_1234567890",
    "claimedAt": "2025-11-23T17:59:00.000Z",
    "metadata": {
      "airdropId": 1,
      "round": 1,
      "bonusMultiplier": 1.0
    }
  }
}
```

**错误码**:
- `400`: 参数错误
- `401`: 未授权
- `403`: 参与资格不足
- `404`: 空投活动不存在
- `429`: 参与过于频繁
- `500`: 服务器内部错误

---

### 6.3 获取空投历史记录

**接口地址**: `GET /api/airdrops/history`

**查询参数**:
- `page`: 页码 (默认: 1)
- `limit`: 每页数量 (默认: 20)
- `type`: 空投类型 (可选)

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "records": [
      {
        "id": 1,
        "airdropId": "1",
        "airdropTitle": "每日空投",
        "airdropType": "daily",
        "points": 85,
        "round": 1,
        "status": "success",
        "failureReason": null,
        "claimedAt": "2025-11-23T17:59:00.000Z"
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 1,
      "totalPages": 1
    }
  }
}
```

---

## 7. 任务管理模块

### 7.1 获取任务列表

**接口地址**: `GET /api/tasks`

**查询参数**:
- `page`: 页码 (默认: 1)
- `pageSize`: 每页数量 (默认: 10)
- `type`: 任务类型 (可选)

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": [
    {
      "id": 1,
      "taskType": "AI智能计算",
      "requiredLevel": 1,
      "earningsPercent": 5.8,
      "amount": 150.00,
      "currency": "USDT",
      "description": "为您提供稳定高效的AI算力服务",
      "isAccelerating": false,
      "status": "available",
      "completionTime": "2小时",
      "difficulty": "简单"
    }
  ]
}
```

---

### 7.2 开始任务

**接口地址**: `POST /api/tasks/{taskId}/start`

**响应示例**:
```json
{
  "code": 200,
  "message": "任务已开始",
  "data": {
    "taskId": 1,
    "status": "running",
    "startTime": "2025-11-23T17:59:00.000Z",
    "estimatedCompletionTime": "2025-11-23T19:59:00.000Z",
    "earningsRate": 8.70
  }
}
```

---

### 7.3 加速任务

**接口地址**: `POST /api/tasks/{taskId}/accelerate`

**请求参数**:
```json
{
  "accelerationHours": 2,
  "pointsCost": 100
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "任务加速成功",
  "data": {
    "taskId": 1,
    "accelerationMultiplier": 2.0,
    "pointsUsed": 100,
    "newCompletionTime": "2025-11-23T18:59:00.000Z",
    "remainingAccelerationTime": "2小时"
  }
}
```

---

## 8. 邀请奖励模块

### 8.1 获取邀请奖励列表

**接口地址**: `GET /api/invite/rewards`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": [
    {
      "id": "level_1",
      "title": "一级好友奖励",
      "description": "邀请1位好友注册并完成实名认证",
      "level": 1,
      "rewardAmount": 100,
      "rewardType": "points",
      "currentProgress": 0,
      "requiredProgress": 1,
      "status": "pending"
    },
    {
      "id": "level_2",
      "title": "二级好友奖励",
      "description": "累计邀请5位好友注册并完成实名认证",
      "level": 2,
      "rewardAmount": 500,
      "rewardType": "points",
      "currentProgress": 3,
      "requiredProgress": 5,
      "status": "progress"
    }
  ]
}
```

---

### 8.2 获取邀请码

**接口地址**: `GET /api/invite/code`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "inviteCode": "INVITE123",
    "inviteLink": "https://app.example.com/invite/INVITE123",
    "qrcodeUrl": "https://api.example.com/qrcode/INVITE123",
    "generatedAt": "2025-11-23T17:59:00.000Z"
  }
}
```

---

### 8.3 获取邀请记录

**接口地址**: `GET /api/invite/records`

**查询参数**:
- `page`: 页码 (默认: 1)
- `limit`: 每页数量 (默认: 20)

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "records": [
      {
        "id": 1,
        "name": "Alice",
        "avatar": "A",
        "level": "LV.3",
        "joinDate": "2025-11-15",
        "reward": 50,
        "children": 3,
        "directReward": "50 DG",
        "indirectReward": "25 DG",
        "totalReward": "75 DG"
      }
    ],
    "stats": {
      "totalInvites": 15,
      "totalRewards": "1,250 DG",
      "directInvites": 8,
      "indirectInvites": 7,
      "activeInvites": 12
    }
  }
}
```

---

## 9. 资产中心模块

### 9.1 获取充值记录

**接口地址**: `GET /api/asset/recharge-records`

**查询参数**:
- `page`: 页码 (默认: 1)
- `pageSize`: 每页数量 (默认: 10)

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "records": [
      {
        "id": 1,
        "type": "USDT",
        "amount": 1000.00,
        "transactionHash": "0x1234567890abcdef",
        "address": "TRX742d35Cc6634C0532925a3b844Bc454e4438f33e",
        "date": "2025-11-15 14:30",
        "status": "completed",
        "confirmations": 20,
        "fee": 1.00
      }
    ],
    "pagination": {
      "page": 1,
      "pageSize": 10,
      "total": 1,
      "totalPages": 1
    }
  }
}
```

---

### 9.2 获取兑换记录

**接口地址**: `GET /api/asset/conversion-records`

**查询参数**:
- `page`: 页码 (默认: 1)
- `pageSize`: 每页数量 (默认: 10)

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "records": [
      {
        "id": 1,
        "fromCurrency": "USDT",
        "toCurrency": "DG",
        "fromAmount": 100.00,
        "toAmount": 1000.00,
        "exchangeRate": 10.0,
        "fee": 0.50,
        "transactionId": "exchange_1234567890",
        "date": "2025-11-15 14:30",
        "status": "completed"
      }
    ]
  }
}
```

---

### 9.3 执行货币兑换

**接口地址**: `POST /api/asset/exchange`

**请求参数**:
```json
{
  "fromCurrency": "USDT",
  "toCurrency": "DG",
  "amount": 100.00
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "兑换成功",
  "data": {
    "transactionId": "exchange_1234567890",
    "fromCurrency": "USDT",
    "toCurrency": "DG",
    "fromAmount": 100.00,
    "toAmount": 1000.00,
    "exchangeRate": 10.0,
    "fee": 0.50,
    "timestamp": "2025-11-23T17:59:00.000Z",
    "status": "completed"
  }
}
```

---

### 9.4 获取支持的区块链类型

**接口地址**: `GET /api/asset/supported-blockchains`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": [
    {
      "code": "TRC20",
      "name": "TRC20 (TRON)",
      "fee": 1.0,
      "minAmount": 10.0,
      "maxAmount": 100000.0,
      "icon": "tron",
      "confirmationTime": "5分钟",
      "isAvailable": true
    },
    {
      "code": "ERC20",
      "name": "ERC20 (Ethereum)",
      "fee": 5.0,
      "minAmount": 10.0,
      "maxAmount": 100000.0,
      "icon": "ethereum",
      "confirmationTime": "15分钟",
      "isAvailable": false
    }
  ]
}
```

---

### 9.5 提交提现请求

**接口地址**: `POST /api/asset/withdraw`

**请求参数**:
```json
{
  "amount": "100.00",
  "blockchainCode": "TRC20",
  "address": "0x1234567890abcdef"
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "提现请求已提交，等待处理",
  "data": {
    "transactionId": "withdraw_1234567890",
    "amount": "100.00",
    "blockchainCode": "TRC20",
    "address": "0x1234567890abcdef",
    "status": "pending",
    "fee": 1.00,
    "estimatedProcessingTime": "24小时",
    "timestamp": "2025-11-23T17:59:00.000Z"
  }
}
```

---

## 10. 消息中心模块

### 10.1 获取收件箱消息列表

**接口地址**: `GET /api/inbox/messages`

**查询参数**:
- `page`: 页码 (默认: 1)
- `limit`: 每页数量 (默认: 20)
- `type`: 消息类型 (可选)

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "messages": [
      {
        "id": 1,
        "title": "系统通知",
        "content": "系统将于今晚22:00进行维护升级",
        "time": "2025-11-23T17:59:00.000Z",
        "isRead": false,
        "type": "system",
        "priority": "high",
        "actions": ["确认", "了解详情"]
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 1,
      "totalPages": 1,
      "unreadCount": 1
    }
  }
}
```

---

### 10.2 标记消息为已读

**接口地址**: `PUT /api/inbox/messages/{messageId}/read`

**响应示例**:
```json
{
  "code": 200,
  "message": "已标记为已读",
  "data": {
    "messageId": 1,
    "isRead": true,
    "readTime": "2025-11-23T17:59:00.000Z"
  }
}
```

---

### 10.3 标记所有消息为已读

**接口地址**: `PUT /api/inbox/messages/read-all`

**响应示例**:
```json
{
  "code": 200,
  "message": "所有消息已标记为已读",
  "data": {
    "markedCount": 5,
    "unreadCount": 0
  }
}
```

---

### 10.4 删除消息

**接口地址**: `DELETE /api/inbox/messages/{messageId}`

**响应示例**:
```json
{
  "code": 200,
  "message": "消息已删除",
  "data": {
    "messageId": 1,
    "deletedAt": "2025-11-23T17:59:00.000Z"
  }
}
```

---

### 10.5 获取未读消息数量

**接口地址**: `GET /api/inbox/messages/unread-count`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "unreadCount": 3,
    "unreadByType": {
      "system": 1,
      "transaction": 1,
      "promotion": 1
    }
  }
}
```

---

## 11. 图表数据模块

### 11.1 获取K线图数据

**接口地址**: `GET /api/v1/chart/kline`

**查询参数**:
- `symbol`: 交易对 (如: BTC/USDT)
- `interval`: 时间间隔 (1m, 5m, 15m, 30m, 1h, 4h, 1d, 1w)
- `page`: 页码 (默认: 1)
- `limit`: 每页数量 (默认: 500)

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "data": [
      {
        "timestamp": 1640995200,
        "open": 45000.0,
        "high": 45500.0,
        "low": 44800.0,
        "close": 45200.0,
        "volume": 1000000,
        "amount": 45200000000
      }
    ],
    "currency": "BTC/USDT",
    "timeRange": "1d",
    "currentPage": 1,
    "totalPages": 1,
    "hasNextPage": false,
    "priceInfo": {
      "currentPrice": 45200.0,
      "priceChange": 200.0,
      "priceChangePercent": 0.44,
      "high24h": 45500.0,
      "low24h": 44800.0,
      "volume24h": 1000000,
      "lastUpdate": "2025-11-23T17:59:00.000Z"
    }
  }
}
```

---

### 11.2 获取实时价格信息

**接口地址**: `GET /api/v1/ticker/price`

**查询参数**:
- `symbol`: 交易对 (如: BTC/USDT)

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "symbol": "BTC/USDT",
    "currentPrice": 45200.0,
    "priceChange": 200.0,
    "priceChangePercent": 0.44,
    "high24h": 45500.0,
    "low24h": 44800.0,
    "volume24h": 1000000,
    "lastUpdate": "2025-11-23T17:59:00.000Z",
    "marketCap": 856000000000,
    "circulatingSupply": 18900000
  }
}
```

---

### 11.3 获取支持的货币对列表

**接口地址**: `GET /api/v1/markets`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": [
    {
      "symbol": "BTC/USDT",
      "baseCurrency": "BTC",
      "quoteCurrency": "USDT",
      "precision": 2,
      "supportedTimeRanges": ["1m", "5m", "15m", "30m", "1h", "4h", "1d", "1w"],
      "isActive": true,
      "minTradeAmount": 0.0001,
      "maxTradeAmount": 100
    },
    {
      "symbol": "ETH/USDT",
      "baseCurrency": "ETH",
      "quoteCurrency": "USDT",
      "precision": 2,
      "supportedTimeRanges": ["1m", "5m", "15m", "30m", "1h", "4h", "1d", "1w"],
      "isActive": true,
      "minTradeAmount": 0.001,
      "maxTradeAmount": 1000
    }
  ]
}
```

---

## 12. 首页数据模块

### 12.1 获取轮播图数据

**接口地址**: `GET /api/home/carousel`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": [
    {
      "id": 1,
      "imageUrl": "https://api.example.com/images/carousel-1.png",
      "title": "advanced_mining.title",
      "subtitle": "advanced_mining.subtitle",
      "description": "先进的AI挖矿技术，为您提供稳定的收益",
      "actionUrl": "/promotion/advanced-mining",
      "order": 1,
      "isActive": true
    },
    {
      "id": 2,
      "imageUrl": "https://api.example.com/images/carousel-2.png",
      "title": "smart_investing.title",
      "subtitle": "smart_investing.subtitle",
      "description": "智能投资策略，让您的资产增值",
      "actionUrl": "/promotion/smart-investing",
      "order": 2,
      "isActive": true
    }
  ]
}
```

---

### 12.2 获取统计数据

**接口地址**: `GET /api/home/statistics`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "dailyYield": 0.011,
    "totalYield": 2108.0,
    "todayProgress": "0%",
    "activeUsers": 15420,
    "totalUsers": 256890,
    "totalAirdrop": 8921,
    "successRate": "85.6",
    "systemStatus": "healthy",
    "lastUpdateTime": "2025-11-23T17:59:00.000Z"
  }
}
```

---

## 13. 限时优惠模块

### 13.1 获取限时优惠套餐列表

**接口地址**: `GET /api/promotion/packages`

**查询参数**:
- `isActive`: 是否激活 (可选)

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": [
    {
      "id": 1,
      "name": "高级算力套餐",
      "price": 2999,
      "originalPrice": 3999,
      "currency": "USDT",
      "description": "送100T算力",
      "profitPercentage": 15.0,
      "duration": "30天",
      "features": [
        "100T AI算力",
        "15% 收益率",
        "专属客服",
        "优先支持"
      ],
      "startTime": "2025-11-23T00:00:00.000Z",
      "endTime": "2025-11-30T23:59:59.000Z",
      "stock": 100,
      "sold": 75,
      "isAvailable": true
    }
  ]
}
```

---

## 14. KYC认证模块

### 14.1 上传身份证图片

**接口地址**: `POST /api/kyc/upload-id-card`

**请求参数**: `multipart/form-data`
- `image`: 身份证图片文件
- `type`: 图片类型 (front/back)
- `category`: 证件类别 (id_card/passport)

**响应示例**:
```json
{
  "code": 200,
  "message": "上传成功",
  "data": {
    "imageUrl": "https://api.example.com/images/id_card_front_123456.png",
    "imageId": "img_1234567890",
    "fileSize": 1024000,
    "uploadTime": "2025-11-23T17:59:00.000Z",
    "status": "pending",
    "validityCheck": {
      "isValid": true,
      "confidence": 0.95,
      "issues": []
    }
  }
}
```

---

### 14.2 提交KYC认证

**接口地址**: `POST /api/kyc/submit`

**请求参数**:
```json
{
  "fullName": "张三",
  "idNumber": "110101199001011234",
  "idCardFrontUrl": "https://api.example.com/images/id_card_front_123456.png",
  "idCardBackUrl": "https://api.example.com/images/id_card_back_123456.png",
  "selfieUrl": "https://api.example.com/images/selfie_123456.png"
}
```

**响应示例**:
```json
{
  "code": 200,
  "message": "KYC认证提交成功，正在审核中",
  "data": {
    "submissionId": "sub_1234567890",
    "status": "submitted",
    "submittedAt": "2025-11-23T17:59:00.000Z",
    "estimatedReviewTime": "24小时",
    "nextSteps": ["等待审核结果通知", "保持联系方式畅通"]
  }
}
```

---

### 14.3 获取KYC状态

**接口地址**: `GET /api/kyc/status`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "status": "approved",
    "submissionId": "sub_1234567890",
    "submittedAt": "2025-11-23T17:59:00.000Z",
    "reviewedAt": "2025-11-24T17:59:00.000Z",
    "rejectionReason": null,
    "fullName": "张三",
    "idNumber": "110101199001011234",
    "idCardFrontUrl": "https://api.example.com/images/id_card_front_123456.png",
    "idCardBackUrl": "https://api.example.com/images/id_card_back_123456.png",
    "selfieUrl": "https://api.example.com/images/selfie_123456.png",
    "kycLevel": "level_2",
    "reviewedBy": "admin"
  }
}
```

---

## 15. 节点统计模块

### 15.1 获取节点统计数据

**接口地址**: `GET /api/node-stats`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "computingPower": "2000K",
    "statusMessage": "Please stay tuned",
    "totalNodes": 1250,
    "activeNodes": 1180,
    "earnings": "$2.5K",
    "isActive": true,
    "networkStatus": "healthy",
    "lastUpdateTime": "2025-11-23T17:59:00.000Z",
    "utilizationRate": 0.856,
    "averageResponseTime": 0.045,
    "throughput": 1250.5
  }
}
```

---

### 15.2 刷新节点统计数据

**接口地址**: `POST /api/node-stats/refresh`

**响应示例**:
```json
{
  "code": 200,
  "message": "统计数据已刷新",
  "data": {
    "refreshTimestamp": "2025-11-23T17:59:00.000Z",
    "dataFreshness": 0.998,
    "nextRefreshTime": "2025-11-23T18:04:00.000Z"
  }
}
```

---

## 16. 新手福利模块

### 16.1 获取新手福利信息

**接口地址**: `GET /api/user/new-user-benefit`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "amount": 100.0,
    "currency": "USDT",
    "isClaimed": false,
    "description": "新用户注册专享体验金",
    "expireAt": "2025-11-30T00:00:00.000Z",
    "claimConditions": [
      "完成实名认证",
      "绑定邮箱",
      "首次充值"
    ],
    "bonusMultiplier": 1.0
  }
}
```

---

### 16.2 领取新手福利

**接口地址**: `POST /api/user/new-user-benefit/claim`

**响应示例**:
```json
{
  "code": 200,
  "message": "新手福利领取成功",
  "data": {
    "success": true,
    "amount": 100.0,
    "currency": "USDT",
    "claimedAt": "2025-11-23T17:59:00.000Z",
    "bonusId": "bonus_1234567890",
    "transactionId": "tx_1234567890"
  }
}
```

---

## 17. 关于我们模块

### 17.1 获取关于我们信息

**接口地址**: `GET /api/about-us`

**响应示例**:
```json
{
  "code": 200,
  "message": "获取成功",
  "data": {
    "email": "support@astrai.com",
    "phone": "400-123-4567",
    "website": "www.astrai.com",
    "address": "新加坡科技园",
    "version": "1.0.0",
    "versionTag": "Latest",
    "copyright": "© 2025 Astra Ai. All rights reserved.",
    "disclaimer": "投资有风险，入市需谨慎",
    "appDescription": "Astra Ai是一款基于AI技术的算力挖矿应用，为用户提供稳定高效的数字货币挖矿服务。",
    "teamInfo": [
      {
        "name": "技术团队",
        "description": "来自全球顶级科技公司的AI专家"
      },
      {
        "name": "运营团队",
        "description": "拥有丰富数字货币运营经验的专业团队"
      }
    ]
  }
}
```

---

## WebSocket实时通信

### 聊天服务

**连接地址**: `wss://api.example.com/ws/chat`

**消息格式**:
```json
{
  "type": "message",
  "data": {
    "messageId": "msg_1234567890",
    "senderId": "user_1234567890",
    "content": "Hello, I need help!",
    "timestamp": "2025-11-23T17:59:00.000Z",
    "status": "sent"
  }
}
```

**消息类型**:
- `message`: 文本消息
- `image`: 图片消息
- `typing`: 正在输入
- `read`: 已读回执
- `system`: 系统消息

---

## 错误码说明

### 通用错误码
- `200`: 成功
- `400`: 请求参数错误
- `401`: 未授权/Token过期
- `403`: 禁止访问/权限不足
- `404`: 资源不存在
- `409`: 冲突（如用户名已存在）
- `422`: 请求格式错误
- `429`: 请求过于频繁
- `500`: 服务器内部错误
- `502`: 网关错误
- `503`: 服务不可用

### 业务错误码
- `USERNAME_EXISTS`: 用户名已存在
- `PASSWORD_INVALID`: 密码格式错误
- `INSUFFICIENT_BALANCE`: 余额不足
- `ACCOUNT_LOCKED`: 账户被锁定
- `KYC_REQUIRED`: 需要KYC认证
- `AIRDROP_EXPIRED`: 空投活动已过期
- `TASK_COMPLETED`: 任务已完成
- `INVITE_CODE_INVALID`: 邀请码无效

---

## 数据类型定义

### 货币类型
- `USDT`: Tether
- `DG`: 平台代币
- `BTC`: Bitcoin
- `ETH`: Ethereum

### 状态类型
- `pending`: 待处理
- `processing`: 处理中
- `completed`: 已完成
- `failed`: 失败
- `cancelled`: 已取消

### 用户等级
- `LV.0`: 初始等级
- `LV.1`: 基础等级
- `LV.2`: 进阶等级
- `LV.3`: 专业等级
- `LV.4`: 高级等级
- `LV.5`: 大师等级

---

## API版本控制

- **当前版本**: `v1`
- **版本格式**: `v{major}.{minor}.{patch}`
- **兼容性**: 主版本号变化时可能不兼容
- **废弃通知**: 废弃的API会提前30天通知

---

## 请求限制

### 速率限制
- 普通用户：每分钟 60 次请求
- VIP用户：每分钟 120 次请求
- 超过限制将返回 `429` 状态码

### 并发限制
- 单用户最大并发连接：10个
- 全局最大并发连接：10,000个

---

## 安全要求

### 请求安全
- 所有API必须使用HTTPS
- 请求头必须包含Authorization
- 敏感数据必须在请求体中传输
- 禁止在URL中传递敏感信息

### 数据安全
- 用户密码必须加密存储
- 交易数据必须加密传输
- 身份证等敏感信息必须脱敏显示
- 数据保留期限符合GDPR要求

---

## 测试环境

- **测试环境URL**: `https://test-api.example.com`
- **测试数据库**: 独立的测试数据库
- **测试账号**:
  - `test_user` / `test123456`
  - `test_vip` / `vip123456`

---

## 更新日志

### v1.0.0 (2025-11-23)
- 初始API发布
- 支持用户认证、算力管理、空投活动等核心功能
- 完整的RESTful API设计
- WebSocket实时通信支持
- 完善的错误处理机制

---

## 联系方式

- **API技术支持**: api-support@astrai.com
- **商务合作**: business@astrai.com
- **Bug反馈**: bugs@astrai.com
- **开发者社区**: https://dev.astrai.com

---

这份API接口文档提供了Coin项目所有API的详细规范，包括请求参数、响应格式、错误码说明等内容，为Rust后端开发提供了完整的接口规范参考。