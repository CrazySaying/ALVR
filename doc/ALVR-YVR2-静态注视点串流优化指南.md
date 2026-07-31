# ALVR 串流优化指南（YVR 2 + RTX 4060 无眼追场景）

> 适用硬件与场景：
>
> - 头显：**YVR 2**（双目 3200×1600，单眼 1600×1600，90Hz，骁龙 XR2 Gen 1）
> - 显卡：**NVIDIA GeForce RTX 4060 Laptop**（NVENC 编码）+ AMD 核显 + MuMu 虚拟显卡
> - CPU：AMD Ryzen 7 7745HX（8C/16T）；内存 16GB；Windows 11
> - 约束：**无眼球追踪** → 只能使用**静态注视点编码（FFE）**；无 AV1 硬解 → 主力编解码器为 **HEVC**

---

## 目录

1. [硬件与能力边界](#一硬件与能力边界)
2. [必改项](#二必改项不改会直接损失画质)
3. [核心串流设置表](#三核心串流设置表)
4. [静态注视点怎么设（核心问题）](#四静态注视点怎么设核心问题)
5. [可选增强](#五可选增强)
6. [环境侧建议](#六环境侧建议)
7. [验证方法](#七验证方法)

---

## 一、硬件与能力边界

这些结论均来自代码与硬件规格，决定了后续所有设置：

| 项目 | 结论 | 依据 |
|---|---|---|
| YVR 2 屏幕 | 单眼 1600×1600，90Hz | 官方规格 |
| AV1 硬解 | **不支持** | `client_openxr/src/lib.rs:357-361`：`encoder_av1` 仅 `Quest3/Quest3S/Pico4Ultra` 为真 |
| 眼动追踪 | 不支持 | → 静态注视点（FFE）是唯一可用方案 |
| 客户端注视点扩展 | `fb_foveation` 为 Meta 扩展，YVR 运行时大概率无 | `client_openxr/src/stream.rs:171-198` |
| 服务端 FFE | 可用 | `foveated_encoding: platform != Platform::Unknown` 对 YVR 为真 |
| 10-bit HEVC | 可用 | `encoder_10_bits` 对 YVR 为真；XR2 Gen1 支持 HEVC Main10 硬解 |
| 编码器选择顺序 | **AMF 优先于 NVENC** | `CEncoder.cpp:42-70`：先 AMF → NVENC → VPL → 软件编码 |
| 适配器选择 | 由 `Video → Adapter index` 决定 | `settings.rs:767` → `CD3DRender::Initialize`（`HMD.cpp:121`） |

⚠️ 编码器隐患：由于 AMF 优先，且本机存在 AMD 核显，**必须确保适配器指向 RTX 4060**，否则编码会落在核显上（画质/延迟双输）。详见 [必改项](#二必改项不改会直接损失画质)。

---

## 二、必改项（不改会直接损失画质）

### 1. 编码器：确保落在 RTX 4060 的 NVENC 上

`CEncoder.cpp:42-70` 先试 AMF 再试 NVENC，而本机有 AMD 核显 + MuMu 虚拟显卡。若 `Video → Adapter index`（默认 0）指到 AMD，编码就落在核显上。

- **操作**：把 `Adapter index` 调到指向 RTX 4060 的索引（0 或 1，取决于 DXGI 枚举顺序；MuMu 虚拟显卡可能占用一个索引）。
- **验证**：连上头显后查看 `session_log.txt`，正常应看到 NVENC 成功；若实际启用了 AMF，则切换索引重试。Dashboard 的 Statistics 也可确认当前编码器。

### 2. 码率：默认只有 Constant 30Mbps，太低

默认值见 `settings.rs:1727`（`BitrateModeDefaultVariant::ConstantMbps`，30Mbps）。

- **改为 `Bitrate → Adaptive`**：
  - `min_throughput_mbps`：开启，**20**
  - `max_throughput_mbps`：开启，**120–150**
  - `saturation_multiplier`：保持 **0.95**
- 理由：注视点会把编码分辨率压到约 51% 像素（约 1170×1120/眼），90Hz HEVC 下 120Mbps 已很充裕；自适应能在 WiFi 波动时自动降档保平滑。

---

## 三、核心串流设置表

| 设置 | 默认 | 推荐 | 理由 |
|---|---|---|---|
| Codec（`preferred_codec`） | H264 | **HEVC** | 无 AV1 硬解；HEVC 同码率画质高约 40%，且支持 10-bit |
| FPS（`preferred_fps`） | 72 | **90** | YVR 2 屏幕原生 90Hz（`settings.rs:1724`） |
| 渲染分辨率（`emulated_headset_view_resolution`） | 1600/眼 | **Scale 1.25 ≈ 2000×2000/眼**（轻量游戏可 1.4–1.5） | 真正的超采样杠杆（游戏按此渲染）。pancake 透镜边缘清晰，超采样收益全屏可见；4060 Laptop 推荐 1.25x 起步 |
| 编码分辨率（`transcoding_view_resolution`） | 同渲染 | **与渲染相同**（Scale 1.25） | 编码器只编码游戏实际渲染的像素；编码 > 渲染 = 放大无中生有、纯浪费码率；< 渲染 = 丢掉超采样。**必须相等**（代码：`server_openvr/src/lib.rs:214-215`） |
| NVENC `quality_preset` | P1（最快最低质） | **P4–P5** | 1600 分辨率 90fps 对 NVENC 负荷很低，P1 浪费 4060 |
| NVENC `tuning_preset` | LowLatency | **LowLatency**（保持） | 串流延迟优先 |
| NVENC `multi_pass` | QuarterResolution | **FullResolution** | 画质更好，此分辨率下开销可忽略 |
| NVENC `adaptive_quantization_mode` | Spatial | **Temporal** | 动态场景更稳，减少色块/色带 |
| `rate_control_mode` | CBR | **CBR**（保持） | 配合自适应码率；VBR 会干扰 ABR 算法（`settings.rs:92-98`） |
| `use_10bit` | 关 | **开** | XR2 Gen1 支持 HEVC Main10 硬解，色带大幅减少，码率成本约 15% |
| `enforce_server_frame_pacing` | true | **true**（保持） | 服务端帧节奏稳定 |
| `max_buffering_frames` | 2.0 | 保持 | 缓冲深度，调小会加剧解码抖动 |

> **保持关闭**：HDR（YVR 2 是 Fast-LCD 非 HDR 屏）；`clientside_post_processing`（走 Meta 的 `fb_composition_layer_settings`，YVR 运行时大概率无效）。

### 分辨率模型（两个设置的职责）

`connection.rs` 协商 + `server_openvr/src/lib.rs:214-215` 明确分工：

| 设置 | 角色 |
|---|---|
| `emulated_headset_view_resolution` | **真正的超采样杠杆**——游戏真的按此分辨率渲染，细节实打实 |
| `transcoding_view_resolution` | 编码缓冲尺寸——游戏输出的帧被缩放进该缓冲再编码 |

**编码分辨率 ≠ 高于渲染分辨率**：

- 编码器只能编码游戏实际渲染的像素；
- 编码 > 渲染：ALVR 放大游戏输出再编码 → 放大不产生像素，纯浪费码率；
- 编码 < 渲染：降采样丢掉超采样细节，白超采样；
- **两者必须相等**（默认即同一 `view_resolution`，`settings.rs:1721-1722`）。

**超采样建议（渲染分辨率，`Scale` 相对设备默认 1600）：**

| 超采样 | 分辨率/眼 | 适用 |
|---|---|---|
| **1.25x** | **2000×2000** | **推荐基准**：多数游戏稳 90fps |
| 1.4–1.5x | 2240–2400 | 轻量游戏可上 |
| >1.5x | >2400 | 收益骤降（显示 1600 原生放不下细节，GPU 白烧） |

**注意**：超采样全屏烧 GPU，但 FFE 只保留中心细节 → 边缘的超采样像素白画。想更锐的中心且不加 GPU 负担：**降低 `edge_ratio` 比加超采样更划算**。

**YVR 运行时上限的坑**：`transcoding_view_resolution` 会被 clamp 到客户端上报的 `max_view_resolution`（`connection.rs:611-620`，超限会 warn）。若 YVR 运行时报的 `max_view_resolution` 偏低，设再高也无用——连接时检查 `session_log.txt` 是否出现 `exceeds client maximum supported resolution` 警告，有则该限制在 YVR 固件层，需改客户端上报逻辑才能突破。

---

## 四、静态注视点怎么设（核心问题）

### 原理

FFE 输出分辨率（每轴）近似为：

```
编码分辨率比例 = center + (1 - center) / edge_ratio
```

即**中心全清晰、边缘被压缩**（实现见 `FFR.cpp:CalculateFoveationVars()`）。对无眼追头显，核心权衡是：

- 中心越大 + 边缘压缩越小 → 外围不糊，但省带宽少；
- 中心越小 + 边缘压缩越大 → 中心极锐、带宽省得多，但**瞥向边缘就糊**。

YVR 2 原生分辨率低（1600/眼），边缘本来就偏软，所以注重点应是**中心够大、压缩别太狠**。

### 三档参考（按 1600 渲染分辨率计算）

| 配置 | center x/y | edge ratio | 编码分辨率/眼 | 像素占比 | 适用 |
|---|---|---|---|---|---|
| 激进（=默认） | 0.45 / 0.4 | 4 / 5 | ~940×832 | **31%** | 网络差、带宽紧张 |
| **推荐** | **0.55 / 0.5** | **2.5 / 2.5** | **~1170×1120** | **51%** | 无眼追的平衡点 |
| 保守 | 0.6 / 0.6 | 2 / 2 | ~1280×1280 | 64% | 介意边缘模糊 |
| 几乎不糊 | 0.65 / 0.6 | 1.5 / 1.5 | ~1400×1390 | 77% | 追求边缘清晰 |

### 推荐值

- `center_size_x`：**0.55**
- `center_size_y`：**0.5**
- `edge_ratio_x`：**2.5**
- `edge_ratio_y`：**2.5**
- `center_shift_x` / `center_shift_y`：**0 / 0**

理由：

1. 编码分辨率压到 51%，**固定码率下中心每像素分到的比特近乎翻倍** → 中心明显更锐，这才是无眼追时 FFE 的真正价值；
2. 边缘只压 2.5 倍，配合客户端超分，瞥向边缘时糊感可控；
3. 若玩着发现边缘 UI/文字糊得难受 → 往「保守档」走；若网络不稳想更稳 → 往「激进档」走。

> `center_shift` 默认 `center_size_x=0.4`（`settings.rs:1885`）是其他设备的光学校准值，YVR 2 用 0 起步；只在发现高清晰区域明显不在视野正中时再微调。

### 验证

静止不动，用余光看画面边缘的文字/UI——若明显模糊到影响观感，就加大 center 或减小 edge_ratio。

---

## 五、可选增强

| 项目 | 建议 | 说明 |
|---|---|---|
| **客户端超分 `upscaling`** | 按需开启；`edge_direction=true`，`upscale_factor` **1.3~1.5**（别超过 2，头显 GPU 扛不住） | 路径：**Dashboard → 设置 → 顶栏 `Video` → `Upscaling`**，开启开关后生效需重启 SteamVR/重连。SGSR v1（`graphics/resources/stream.wgsl:128-346`）是空间放大+锐化，**作用是把"编码分辨率设低 + 客户端高质量放大"的省带宽策略变可行，不能恢复 FFE 丢掉的边缘细节**。按推荐 transcode=2000（≥显示 1600）时收益边际，可不开；想省带宽则把 transcode 降到 ~1300-1400 再开 |
| **`avoid_video_glitching`** | 网络不稳时开 true | 丢包时丢弃损坏帧而非显示花屏，观感从"马赛克"变"轻微跳帧"（`settings.rs:1444-1446`） |

---

## 六、环境侧建议

1. **WiFi 用 5GHz（或 6GHz）专用 SSID**，头显连它，其他设备不蹭；PC 尽量有线接路由。
2. 路由与 PC 之间避开 USB 3.0 设备、微波炉等干扰源。
3. SteamVR 的自动分辨率缩放交给 ALVR 的 `emulated_headset_view_resolution` 统一管理。

---

## 七、验证方法

1. **编码器**：查看 `session_log.txt` 中 `Try to use VideoEncoder...` 系列日志，确认最终启用了 NVENC。
2. **延迟目标**（Dashboard Statistics，90Hz 下）：
   - 网络延迟（network）< 10ms
   - 编码延迟（encoder）< 6ms
   - 解码延迟（decoder）< 8ms
3. **画质**：中心文字/UI 应明显锐利；边缘用余光检查糊感是否符合预期。

---

## 预期效果

HEVC + 10-bit + P4~P5 编码 + 51% 注视点，在 120Mbps 自适应码率下：
- 中心清晰度明显好于默认配置（H264 + 30Mbps + 激进注视点）；
- 90Hz 帧率稳定，WiFi 波动时自动降码率保平滑。

---

## 附：相关代码位置速查

| 功能 | 位置 |
|---|---|
| 注视点参数定义/默认 | `alvr/session/src/settings.rs:458-492`、`:1878-1890` |
| FFE 数学实现 | `alvr/server_openvr/cpp/platform/win32/FFR.cpp:28-80` |
| 编码器选择顺序 | `alvr/server_openvr/cpp/platform/win32/CEncoder.cpp:42-70` |
| 适配器选择 | `alvr/server_openvr/cpp/platform/win32/shared/d3drender.cpp`、`HMD.cpp:121` |
| AV1 能力判定 | `alvr/client_openxr/src/lib.rs:357-361` |
| 客户端注视点扩展 | `alvr/client_openxr/src/stream.rs:171-198` |
| SGSR 超分实现 | `alvr/graphics/resources/stream.wgsl:128-346` |
| 丢包恢复逻辑 | `alvr/client_core/src/connection.rs:286-344` |
| 自适应码率算法 | `alvr/server_core/src/bitrate.rs:177-241` |
