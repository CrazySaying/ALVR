<p align="center"> <img width="500" src="resources/ALVR-Grey.svg"/> </p>

# ALVR - Air Light VR（中文版 Fork）

通过 Wi-Fi 将 PC 上的 VR 游戏串流到头显。

本项目是基于 [ALVR](https://github.com/alvr-org/ALVR) 的 **中文本地化 + YVR 设备支持** Fork，由 [CrazySaying](https://github.com/CrazySaying) 维护。

## Fork 特性

本 Fork 在保留上游 ALVR 全部功能的基础上，主要增加了以下内容：

- **YVR 1 / YVR 2（Play For Dream）设备支持**
  - 移植自 YVR 官方串流助手（PFDM Streaming Assistant）的 YVR1（D1）/ YVR2（D3）头显与手柄支持。
  - 内置 YVR 手柄渲染模型（D1 / D3 左右手柄）。
  - YVR 手柄相对 Oculus Touch 姿态的 25° 旋转偏移校准，握持方向与实际一致。
- **YVR 设备预设**
  - Dashboard 内一键应用 "YVR device preset"，自动设置 YVR1 / YVR2 模拟模式与手柄偏移，无需手动逐项配置。
- **中文本地化**
  - Dashboard 与 Launcher 界面全面汉化（zh-CN）。
  - 内置中文字体加载支持，解决中文乱码问题。
- **稳定性修复**
  - 默认 Web 服务端口改为 **8084**，避开腾讯 QQ（QQNT）占用 8082 导致的启动崩溃。
  - 修复 YVR 预设数组偏移写入失败（`invalid type: floating point 0.0, expected a boolean`）等问题。
  - 移植 YVR1 边缘延迟与加载器修复（来自 ALVR-shiroha）。
- **CI 手动构建工作流**
  - GitHub Actions 提供 `Manual build` 工作流，使用 **release** 构建产出 Windows / Linux / Android 安装包。

## 兼容性

|             VR 头显             |          支持情况          |
| :-----------------------------: | :------------------------: |
|  **YVR 1 / YVR 2 / Play For Dream MR** | :heavy_check_mark: **重点支持** |
|       Quest 1/2/3/3S/Pro        |   :heavy_check_mark: (\*)   |
|      Pico Neo 3/4/4 Ultra       |        :heavy_check_mark:        |
|   Vive Focus 3/Vision/XR Elite  |        :heavy_check_mark:        |
|        PhoneVR（手机）          | :heavy_check_mark: (\*\*) |
|        Android / Monado         |        :warning: (\*\*)       |

\* Quest 1 不在 Meta 商店上架。  
\*\* 部分手机可用，但未经广泛测试。

|         PC 操作系统          |     支持情况      |
| :-------------------------: | :---------------: |
|        Windows 10/11        |  :heavy_check_mark:  |
|      Windows XP/7/8         |         :x:        |
|            Linux            |  :heavy_check_mark: (\*\*\*) |
|            macOS            |         :x:        |

\*\*\* 详细兼容性请参考 wiki。

### 硬件要求

-   一台受支持的独立 VR 头显（见上方兼容表）。
-   SteamVR。
-   一台性能较好的 PC：
    -   操作系统见上方兼容表。
    -   NVIDIA GPU（NVENC，GTX 1000 系列或更新）、AMD GPU（AMF VCE）、或 Intel GPU（VPL，Arc / Tiger Lake 或更新），并安装最新驱动。
    -   笔记本同时有核显（Intel HD / AMD iGPU）和独显（NVIDIA GTX/RTX / AMD）时，请将 **ALVR 和 SteamVR 指定到独显**（"高性能图形适配器"）以获得最佳性能。
      （NVIDIA：NVIDIA 控制面板 → 管理 3D 设置 → 程序设置；AMD：类似方法）
-   网络：
    -   头显使用 **802.11ac 5GHz Wi-Fi**，PC 建议使用有线以太网。
    -   PC 与头显需连接到同一路由器（或使用[路由转发](https://github.com/alvr-org/ALVR/wiki/ALVR-v14-and-Above)连接）。

## 安装

参考[安装指南](https://github.com/alvr-org/ALVR/wiki/Installation-guide)。

## 故障排查

-   参考[故障排查](https://github.com/alvr-org/ALVR/wiki/Troubleshooting)页面（Linux 见 [Linux Troubleshooting](https://github.com/alvr-org/ALVR/wiki/Linux-Troubleshooting)）。
-   配置建议与更多信息见[这里](https://github.com/alvr-org/ALVR/wiki/Information-and-Recommendations)。
-   串流设置参考：见仓库 `doc/` 目录下的 [YVR 2 串流优化指南](doc/ALVR-YVR2-静态注视点串流优化指南.md)。

## 卸载

打开 `ALVR Dashboard.exe`，进入 `安装` 选项卡，点击 `移除防火墙规则`。  
关闭 ALVR 窗口并删除 ALVR 文件夹。

## 从源码构建

参考[构建指南](https://github.com/alvr-org/ALVR/wiki/Building-From-Source)。  
也可在 GitHub Actions 上手动触发 `Manual build` 工作流，用 release 模式构建各平台安装包。

## 许可证

ALVR 使用 [MIT 许可证](LICENSE)。

## 隐私政策

ALVR 应用不主动收集任何个人数据。

## 致谢

-   上游 [ALVR](https://github.com/alvr-org/ALVR) 项目及其社区。
-   [ALVR-shiroha](https://github.com/alvr-shiroha/ALVR)（YVR 相关修复的参考）。
-   YVR / 玩出梦想（Play For Dream）官方串流助手（设备支持移植来源）。
