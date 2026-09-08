<p align="center">
  <img src="./assets/readme/hero.svg" width="100%" alt="A3S GUI 将 Rust RSX 与 TypeScript TSX 汇入同一条自绘语义、布局、交互、无障碍与 Graphics 管线">
</p>

<p align="center">
  <strong>Language / 语言:</strong>
  <a href="README.md">English</a> ·
  <a href="README.zh-CN.md">中文</a>
</p>

<p align="center">
  <a href="https://github.com/A3S-Lab/GUI/actions/workflows/ci.yml"><img alt="CI 状态" src="https://github.com/A3S-Lab/GUI/actions/workflows/ci.yml/badge.svg"></a>
  <img alt="Rust 1.95.0" src="https://img.shields.io/badge/Rust-1.95.0-2F3945?style=flat-square&logo=rust&logoColor=white">
  <img alt="仅自绘" src="https://img.shields.io/badge/renderer-self--drawn%20only-0067C0?style=flat-square">
  <img alt="TSX T3 Windows 切片" src="https://img.shields.io/badge/TSX-T3%20Windows%20slice-1687D9?style=flat-square">
  <a href="LICENSE"><img alt="MIT 许可证" src="https://img.shields.io/badge/license-MIT-2F3945?style=flat-square"></a>
</p>

**A3S GUI** 是面向 Rust 的原生语义 UI 与自绘渲染运行时。
Rust RSX 现已可用；私有包 `@a3s/gui` 的自动 JSX 运行时会将标准
TSX 降级为同一套版本化帧协议。两条编写路径都不使用 DOM、CSSOM、
WebView，也不依赖平台内容控件工具包。

> [!IMPORTANT]
> A3S 拥有每一个应用内容像素。原先的 AppKit、GTK4 与 WinUI
> 内容后端、特性、依赖、示例、打包脚本与 CI
> 通道均已删除。`platform-host` 定义零控件边界，
> `platform-runtime` 实现共享的原子自绘帧运行时，
> `host-windows` 提供首个原始 Win32 窗口生命周期、自有表面
> 租约，以及规范化的鼠标/键盘/滚轮/触摸/笔输入。由 Graphics 拥有的
> presenter 负责提交、呈现，并可捕获精确准备好的 DX12
> 交换链帧。原始 macOS/Linux 宿主仍在路线图中。

## 一棵语义树，一条自有像素管线

```text
Rust ComponentCx / .rsx           TypeScript / TSX
            |                           |
            |                  @a3s/gui/jsx-runtime
            +-------------+-------------+
                          |
                          v
                 versioned UI frame
                          |
                          v
                   NativeElement tree
                 /          |           \
                v           v            v
       layout snapshot  accessibility  interaction + hit regions
                |                         |
                +------------+------------+
                             v
                    A3S Graphics Scene
                             |
                 +-----------+-----------+
                 v                       v
       deterministic software          wgpu
                                         |
                              Metal / DX12 / Vulkan
                                         |
                              zero-widget OS surface
```

语义身份在布局、命中测试、无障碍、焦点、
选择与渲染之间共享。绘制命令绝不会变成无障碍树，
Graphics 也绝不会从像素推断行为。

| 层 | 负责 | 明确不负责 |
| --- | --- | --- |
| 编写 | Rust 组件/RSX；自动 TSX 元素与回调 | OS 句柄、GPU 资源、布局真值 |
| 语义运行时 | 角色、属性、动作、焦点、选择、浮层、i18n、拖放、无障碍 | 平台控件、工具包布局、产品 I/O |
| 布局与场景 | 可移植样式、量化盒子、稳定路径、命中区域、场景提取 | 产品状态或 OS 几何 |
| [A3S Graphics](https://github.com/A3S-Lab/Graphics) | 保留场景、损伤、软件参考输出、GPU 表面、准备、渲染与呈现 | 组件、窗口、IME、无障碍 |
| 平台边界 | 原生窗口、自有表面生命周期目标、提交协调、规范化输入、文本/IME、无障碍桥接、剪贴板与系统服务 | 应用内容控件、样式、布局或绘制 |

请阅读[架构](docs/architecture.md)与
[自绘平台宿主](docs/platform-hosts.md)了解契约。

## 当前 DX12 证据

<p align="center">
  <img src="./docs/assets/calculator-tsx-dx12.png" width="410" alt="当前自绘计算器几何与颜色的精确 DX12 交换链捕获">
</p>

这是规范 TSX 计算器的精确已准备交换链图像，
并非浏览器或工具包的 mock。同一模型会从 Rust RSX 产出字节级一致的
软件输出；经审阅的 DX12 运行在 254,200 个像素中有 940 个不同
（0.370%），最大通道差分为 91。
[捕获清单](docs/assets/calculator-tsx-dx12.json)固化了该证据。
刻意省略标签，因为生产级字体/塑形/字形
后端是渲染器的下一个里程碑。

## 当前实现

本仓库已提供：

- 稳定的 `NativeElement` 语义 IR，以及严格的版本化帧协议；
- Rust RSX 解析、组件、hooks、reducers、effects，以及内置
  语义组件目录；
- 焦点、交互、浮层、选择、集合导航、i18n、
  拖放策略、live regions，以及无障碍快照；
- 覆盖全部 504 个顶层 `PortableStyle` 字段的可移植样式解析；
- 确定性布局快照、稳定场景身份、软件参考
  输出、保留损伤、规范 RSX/TSX 计算器对等，以及经审阅的
  离屏与精确原生表面 DX12 证据；
- 明确的生产文本边界：有界 `TextShaper` 产出
  唯一的固有度量与保留字形记录；口令值在
  塑形前被掩码；有状态的 `TextSceneEncoder` 消费同一
  记录，既不接收源文本，也不越出其墨迹边界；
- 带依赖防火墙的有界 `PlatformHost` 事务/事件契约；
- 目标门控的 `WindowsPlatformHost`，具备真实 Win32 `HWND` 生命周期、
  per-monitor-v2 DPI 客户区尺寸、消息泵、焦点/关闭/遮挡
  事件、DPI 正确的遗留鼠标/键盘/滚轮转译、捕获与
  失焦取消、带压力与命名空间身份的有界并发 `WM_POINTER` 触摸/笔
  转译、真实 HWND 触摸与
  User32 合成笔注入证据、兼容性鼠标抑制、
  笔杆按钮规范化、暴露前几何的最小化/恢复
  恢复、原生缩放事务对账、隐藏首帧
  暂存、防止过早销毁 HWND 的自有原始表面租约、
  原子
  prepare/commit/rollback，以及 Windows 原生 CI 证据；
- `SelfDrawnWindowRuntime`，具备原子 prepare/commit/reject、恢复、
  类型化呈现结果、规范化输入、命中测试、拖放、
  无障碍动作、保留重绘重试，以及参考/录制/GPU
  presenter；TSX 事件泵每轮最多尝试一次待处理恢复；
- 真实的 Windows 呈现门：在暂存 HWND 上准备 Graphics 交换链帧、
  提交并显示原始窗口、经 DX12 呈现、
  校验已提交场景指纹、通过显式故障注入特性销毁已附着设备、
  重建设备与表面，
  并在销毁 HWND 之前释放 GPU 表面；
- 一次性 presenter 捕获契约：在呈现前复制精确已准备的
  交换链纹理，将 BGRA 规范化为 RGBA，并在目标原生 CI 中
  将可见 TSX 计算器与共享的确定性 RSX/TSX 场景进行比对；
- Rust 生成的 TypeScript 协议声明、规范跨语言
  夹具、自动 JSX 降级、严格帧规范化，以及
  按 revision 作用域排序的回调；
- 传输无关的 TypeScript `createApp` 生命周期，含键控组件
  实例、类型化 context、渲染错误边界、state/reducer/memo/ref/
  effect hooks、批处理重渲染、提交后清理，以及严格的握手后
  会话/消息身份；
- 已定稿的 TSX 身份契约：Node 本地组件实例使用
  规范 `a3s:c1:` 身份；自动动作使用仅由原生键路径加事件名派生的
  规范 `a3s:a1:`
  id；生成的
  `a3s:` 命名空间不能被显式动作认领；Rust 在 Host 边界拥有并
  校验这些常量，并生成其 TypeScript
  声明；
- 无依赖的 TypeScript 客户端握手，以及与 Rust 协议边界对齐的
  增量小端序 JSON 帧编解码器；
- 有序成帧连接，以及显式无 shell 的 Node 子进程字节
  传输，带有界 stderr、关闭超时与真实进程夹具。
- `a3s-gui-tsx-host`：严格成帧的 stdin/stdout 进程，协商单个
  TSX 会话并将完整帧降级为 `NativeElement`。Windows 产品
  构建选择 `WindowsPlatformHost + ReverseScenePresenter`，打开可见原始
  HWND，经 Graphics/DX12 呈现，持续泵送规范化输入，
  返回有序 TSX 动作（含窗口关闭），并在重绘/缩放后接受单调的
  host revision。软件参考构建仍是确定性进程测试设施，
  而非替代产品宿主。
- `A3sFramedApplicationHostV1`：有序单读应用泵，
  与 `createApp` 共享协商后的客户端会话，限制未完成的
  事件工作与 host 消息重排，在待处理
  commit/event 应用期间应答 host ping，执行有超时边界的客户端 ping/pong 与
  close/ack 控制，传播致命/流失败，并由
  真实 Node 经 Rust 软件宿主演练。
- 无宿主的 `createApp(App).run()` 路径：选择已校验的
  平台宿主制品，创建唯一协议会话，在首次渲染前绑定原生事件，
  并在失败时关闭部分启动的宿主。
- 可选有界 Host 监督：观察异常终止、
  协商新会话、事务性重放上次已提交的完整
  帧与回调作用域、在重放提交前门控事件，并在配置的重启预算耗尽后关闭
  应用；
- 真实三代子进程门：使第一个 Host 崩溃、
  在重放后接受键盘激活、拒绝更旧的 render revision
  且不调用其回调，并再次重放到新会话。

生产原生应用前仍需：

- Windows 硬件设备笔捕获以及倾斜/旋转/橡皮擦一致性、
  TSF、UI Automation 与系统服务；
- 真实的零控件 macOS 与 Wayland/X11 宿主；
- 生产级字体数据库/塑形器与字形光栅/图集编码器，随后是
  文本编辑、IME 与辅助技术桥接；
- 打包、签名、安装程序工作，以及三平台视觉证据；
- 每个 React Aria 家族的完整自绘一致性证据。

非可视化的 `HeadlessAdapter` 仅作为协议与事务测试
基础设施保留。它不是渲染器，且始终发出诊断类
`a3s_gui::HeadlessNode`。

## 完整 React Aria 范围

目标是官方
[React Aria](https://react-aria.adobe.com/) 目录中的每一个语义家族。已检入的
[组件矩阵](docs/react-aria-component-matrix.json) 固化
`react-aria-components` 1.19.0，并在 CI 中做 schema 测试。

- 全部 51 个官方顶层家族均有显式 A3S 组件映射。
- `Button` 已有首个场景/软件像素冒烟证据。
- 尚无任何家族被标记为自绘一致。
- 一个家族只有在编写、行为、布局/命中、
  场景、确定性像素、无障碍，以及真实 macOS/Windows/Linux
  宿主证据全部通过后，才算一致。

一致性契约与已知 API 缺口见
[React Aria 自绘方向](docs/react-aria-native.md)。

## 无需浏览器的 TSX

TypeScript 设计采纳了
[Nub](https://github.com/nubjs/nub) 的有用边界：标准 Node 加载普通
TSX，而狭窄的 Rust 进程拥有语义对账、布局、
渲染与 OS 资源。

```tsx
import { Button, Text, View, Window, createApp, useState } from "@a3s/gui";

function Counter() {
  const [count, setCount] = useState(0);
  return (
    <Window title="Counter" width={360} height={220}>
      <View className="flex-col gap-4 p-6">
        <Text>Count: {count}</Text>
        <Button onPress={() => setCount((value) => value + 1)}>
          Increment
        </Button>
      </View>
    </Window>
  );
}

await createApp(Counter).run();
```

该零参数启动 API 已实现，并从 Node 对
确定性 Rust 软件/自绘测试进程进行演练。它会解析精确的
平台制品包，并在 spawn 前校验其清单、大小、目标、协议范围与
SHA-256 校验和；仓库测试路径使用显式
绝对路径开发制品覆盖，以及未校验 Host 的可选启用。
Windows 目标通道现已通过可见原始 HWND、
Graphics/DX12 呈现、Win32 鼠标输入与语义窗口关闭动作驱动同一协议。
已发布的原生包、经审阅的视觉对等、无障碍/IME 完成，
以及 macOS/Linux 宿主尚不存在，因此这仍不是面向最终用户的原生
包。底层调度器也
仍可与类型化宿主对象一起使用。它拥有键控函数组件
实例、类型化嵌套 context、渲染错误边界、state/reducer/memo/
ref/effect hooks、单微任务重渲染批处理、按 revision 作用域的回调、
仅在 `committed` 之后的 effect 提升、完整渲染信封、独立的
客户端/宿主消息排序、客户端 `hello`/`welcome` 协商、有界
增量成帧、显式 Node 子进程传输、严格的
`a3s-gui-tsx-host` 自绘进程，以及与 `createApp` 共享的有序成帧
应用宿主，包括双向 ping/pong、
固定宿主存活期限、协议级优雅关闭，以及可选的
跨新协议会话的有界重启/重放监督。完成
原生服务与对等门、平台制品发布以及 npm
发布仍属 T3–T5 工作。

协议与交付门详见
[TSX 原生运行时](docs/tsx-native-runtime.md)。

## Rust RSX 快速开始

从 Git 消费该 crate：

```toml
[dependencies]
a3s-gui = { git = "https://github.com/A3S-Lab/GUI" }
```

定义状态、注册 reducer，并返回语义 RSX：

```rust
use a3s_gui::{rsx, ComponentCx, GuiResult, RSX};

#[derive(Default)]
struct CounterState {
    count: u32,
}

fn counter(cx: &mut ComponentCx<CounterState>) -> RSX {
    let count = cx.use_state("count", |state: &CounterState| state.count);
    let increment = cx.use_reducer("increment", |state: &mut CounterState, _| {
        state.count += 1;
        Ok(())
    });

    rsx!(
        <Button key="increment" onPress={increment}>
          Count {count}
        </Button>
    )
}

fn main() -> GuiResult<()> {
    let component = ComponentCx::compile("counter", counter)?;
    let frame = component.render(&CounterState::default())?;
    println!("{} action(s)", frame.actions.len());
    Ok(())
}
```

运行维护中的示例：

```sh
cargo run --locked --example component_playground
cargo run --locked --example dogfood_session
cargo run --locked --no-default-features \
  --features authoring,platform-runtime,software-reference \
  --example self_drawn_calculator
```

## Cargo 特性

| 特性 | 用途 |
| --- | --- |
| `default` | `authoring + design-system + software-reference` |
| `authoring` | 基于 SWC 的 Rust RSX 解析与编译 |
| `design-system` | 内置语义组件注册 |
| `graphics` | GUI 到 A3S Graphics 的场景边界 |
| `software-reference` | 确定性软件 presenter |
| `gpu` | Graphics 拥有的离屏与原生表面 GPU 路径 |
| `gpu-fault-injection` | 显式仅用于一致性的设备销毁；默认永不启用 |
| `platform-host` | 零控件 OS 边界契约与录制宿主 |
| `platform-runtime` | 共享自绘帧/输入/无障碍运行时 |
| `host-windows` | 原始 Win32 顶层宿主、自有 `HWND` 表面目标，以及规范化遗留输入；无 WinUI/XAML |
| `host-macos`、`host-linux-*` | 零控件宿主能力标记；具体宿主尚未实现 |
| `typescript-schema` | Rust 到 TypeScript 的协议声明生成 |

刻意不提供 AppKit、GTK4、WinUI 或对应的
内容工具包特性标志。

## 开发

完整的 CI 等价门是：

```sh
just verify
```

常用聚焦门：

```sh
just check-core
just check-platform-host
just check-platform-runtime
just test-platform-host
just test-platform-runtime
just test-windows-host # 仅 Windows
just test-graphics
just check-tsx-protocol
just test-typescript
just test-tsx-host
```

`just verify` 还会检查依赖防火墙、格式化、Clippy、rustdoc、
全部 Rust 测试与示例、React Aria 目录、TypeScript 夹具，以及
空白。CI 额外加入 Windows 原生生命周期与最小化/恢复顺序、
触摸/笔系统注入、输入/取消、H1 事务、真实 DX12
呈现、精确计算器交换链捕获对等，以及设备丢失
重建证据。
不存在工具包专用内容宿主与遗留捆绑通道。

## 仓库地图

```text
src/
|- compiler/              RSX/intrinsic lowering into semantic IR
|- runtime/               reconciliation, focus, interaction, selection
|- layout/                deterministic layout snapshots and hit regions
|- drawing/               semantic/layout to A3S Graphics scenes
|- platform_host/         zero-widget contracts, recording host, raw Win32 host
|- platform_runtime/      shared atomic self-drawn window runtime
|- tsx_protocol/          strict Node/Rust wire protocol
|- bin/tsx_host.rs        TSX process/session entry point
|- bin/tsx_host/          native/test backend selection and event pump
|- semantic_ui/           React Aria-aligned semantic components and hooks
`- platform/              nonvisual planning/transaction test IR
packages/typescript/      private automatic JSX runtime and protocol SDK
tests/                    dependency firewalls and catalog gates
docs/                     architecture, roadmap, protocol, and conformance
```

## 路线图

下一条关键路径是：

1. 在已落地的通用文本契约上实现字体发现/塑形与字形光栅/图集后端，
   然后加入编辑与 IME；
2. 在 Windows 上加入硬件设备笔捕获与倾斜/旋转/橡皮擦语义、TSF、
   UI Automation 与系统服务；
3. 将同一零控件捕获契约移植到 macOS 与 Wayland/X11；
4. 按里程碑关闭 React Aria 家族，并提供三平台证据；
5. 仅在自绘宿主制品存在后再恢复打包。

版本化计划见 [ROADMAP](docs/roadmap.md)。

## 文档

- [架构](docs/architecture.md)
- [路线图](docs/roadmap.md)
- [自绘平台宿主](docs/platform-hosts.md)
- [TSX 原生运行时](docs/tsx-native-runtime.md)
- [React Aria 自绘方向](docs/react-aria-native.md)
- [React Aria 组件矩阵](docs/react-aria-component-matrix.json)
- [RSX 指南](docs/rsx.md)
- [RSX 框架](docs/rsx-framework.md)
- [布局与场景](docs/layout-scene.md)
- [应用外壳](docs/app-shell.md)
- [样式契约](docs/style-contract.md)
- [渲染器字段清单](docs/renderer-field-inventory.md)
- [打包门](docs/packaging.md)

## 许可证

[MIT](LICENSE)
