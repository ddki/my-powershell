# my-powershell

# Window Terminal 增强

## 💻 Window Terminal

🏠 https://github.com/microsoft/terminal

## 📦 scoop

Scoop 是 Windows 的命令行安装程序。

⬇ https://github.com/ScoopInstaller/Install

```powershell
# 获取安装文件
irm get.scoop.sh -outfile 'install.ps1'

# 执行安装（安装文件里有参数说明）
# ScoopDir 是 scoop 的安装目录
# ScoopGlobalDir 是 scoop 全局 app 的安装目录
# Proxy 安装时的代理设置
.\install.ps1 -ScoopDir 'D:\Applications\Scoop' -ScoopGlobalDir 'F:\GlobalScoopApps'
```

```powershell
# 使用
scoop install coreutils
```

## 🎨 美化

### 📝 文件图标

```sql
Install-Module -Name Terminal-Icons -Repository PSGallery
```

### 😮 oh-my-posh

```sql
Set-ExecutionPolicy Bypass -Scope LocalMachine -Force; Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://ohmyposh.dev/install.ps1'))
```

### 🖋 设置字体

https://ohmyposh.dev/docs/installation/fonts

```sql
oh-my-posh font install
```

## 🚀 补全

**相关项目：**

* https://github.com/abgox/PS-completions
* https://github.com/PowerShell-Completion

```sql
Install-Module PSReadLine -Scope AllUsers
Install-Module posh-git -Scope AllUsers
Install-Module yarn-completion -Scope AllUsers
Install-Module npm-completion -Scope AllUsers
```

### 🛠 配置

```sql
notepad $PROFILE
```

```sql
$PSDefaultParameterValues['*:Language'] = 'zh-CN'

Import-Module 'E:\development\vcpkg\scripts\posh-vcpkg'

# PSReadLine
Import-Module PSReadLine
# 设置预测性分析引擎,开启智能补全
Set-PSReadLineOption -PredictionSource History
# 设置补全热键,例如Tab
# Set-PSReadLineKeyHandler -Key Tab -Function Complete
# Tab键会出现自动补全菜单
Set-PSReadLineKeyHandler -Key Tab -Function MenuComplete 
# 上下方向键箭头，搜索历史中进行自动补全
Set-PSReadlineKeyHandler -Key UpArrow -Function HistorySearchBackward
Set-PSReadlineKeyHandler -Key DownArrow -Function HistorySearchForward

# oh-my-posh
# 主题设置
oh-my-posh init pwsh --config "$env:POSH_THEMES_PATH\M365Princess.omp.json" | Invoke-Expression
# （5.x语法）
Set-PoshPrompt -Theme M365Princess

# 文件图标
Import-Module -Name Terminal-Icons

# 自动补全
Import-Module npm-completion
Import-Module yarn-completion
Import-Module posh-git

```

## 🧰 coreutils

https://next.cyp0633.icu/post/%E7%94%A8-rust-uutils-%E6%9B%BF%E6%8D%A2-windows-powershell-%E5%86%85%E7%BD%AE-cmdlet/

https://github.com/uutils/coreutils

安装：
```sh
# use cargo 
cargo install coreutils --features windows
# use scoop
scoop install uutils-coreutils
```

