   这是用户登录的默认进入的系统，主要功能：
     
	 - 酒馆
	   - 唠嗑（类似聊天室）
	 - 工具
	   - 记事本
	   - 密码本，支持pin码解锁
	 - 许愿墙  
	 - 切换皮肤，支持白天、黑夜、拿铁、玫瑰四种
	 - 进入系统，可以进入管理后台、轻松脚本、股票助手，需要系统权限。系统列表可扩展
	 - 问题与建议
	 - 联系我们
	
   技术架构：daisyUI+leptos+tauri+matchbox
	
	    - 唠嗑通讯使用webrtc协议，基于matchbox实现
		- 记事本、密码本由tauri项目封装的文件API实现，只本地存储

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## dev

trunk serve

## build

trunk build

##

输入命令：unzip 文件名.zip。 例如：unzip archive.zip。
输入命令：unzip 文件名.zip -d 目标目录路径。 例如：unzip archive.zip -d /path/to/directory。
