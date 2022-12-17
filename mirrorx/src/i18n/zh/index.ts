import type { Translation } from '../i18n-types';

const zh_Hans: Translation = {
	DialogActions: {
		Ok: '确定',
		Cancel: '取消',
		Yes: '是',
		No: '否',
		Allow: '允许',
		Reject: '拒绝'
	},
	Home: {
		Domain: '域',
		DeviceID: '设备ID',
		RemoteDeviceID: '远程设备ID',
		Password: '密码',
		Connect: '连接',
		Desktop: '桌面',
		Files: '文档',
		EditPasswordTooltip: '修改密码',
		PasswordVisibleTooltip: '点击以显示密码',
		PasswordInvisibleTooltip: '移开鼠标以隐藏密码',
		EditPasswordCancelTooltip: '取消',
		ClickToCopyDeviceIDTooltip: '点击以复制',
		ClickToCopyDeviceIDCopiedTooltip: '已复制',
		GenerateRandomPasswordTooltip: '生成随机密码'
	},
	LAN: {
		HostnameOrIP: '主机名 或 IP',
		Discoverable: '可被发现'
	},
	Dialogs: {
		About: {
			Copy: '复制'
		},
		VisitPrepare: {
			Content: '请输入该设备的密码'
		},
		LANConnect: {
			Content: '你想要连接这台设备吗？'
		},
		SelectLanguage: {
			Title: '选择语言'
		}
	},
	Settings: {
		Appearance: {
			Title: '外观',
			Theme: '主题',
			Light: '浅色',
			Dark: '深色',
			Auto: '自动'
		}
	}
	// Settings: {
	// 	WindowTitle: '设置',
	// 	Layout: {
	// 		Domain: '域',
	// 		About: '关于'
	// 	},
	// 	Pages: {
	// 		Domain: {
	// 			Current: '当前：',
	// 			Tooltips: {
	// 				Add: '增加新的域',
	// 				SetPrimary: '设置为主域',
	// 				Edit: '编辑',
	// 				Delete: '删除'
	// 			}
	// 		},
	// 		About: {
	// 			Version: '版本',
	// 			Official: '官方网站',
	// 			SourceRepository: '代码仓库',
	// 			SupportAndHelp: '帮助支持'
	// 		},
	// 		Dialog: {
	// 			EditDomain: {
	// 				Title: '修改域',
	// 				Name: '名称',
	// 				DeviceId: '设备ID',
	// 				FingerPrint: {
	// 					Label: '指纹',
	// 					Tooltip: `指纹是一串在你连接到新的域时在本地随机生成的字符串。它用来证明你的设备有权利持有域分配给你的设备ID一段时间并且不会被用来追踪你的设备。`
	// 				},
	// 				Remarks: '备注'
	// 			},
	// 			AddDomain: {
	// 				Title: '增加域',
	// 				AddressInputPlaceHolder: '域地址（IP:端口 或 链接）',
	// 				RemarksInputPlaceHolder: '备注'
	// 			},
	// 			SetPrimaryDomain: {
	// 				Title: '设为主域',
	// 				ContentPrefix: '你想要设置',
	// 				ContentSuffix: '为主域吗？'
	// 			},
	// 			DeleteDomain: {
	// 				Title: '删除域',
	// 				ContentPrefix: '你想要删除域',
	// 				ContentSuffix: `吗？一旦你删除将无法恢复！`
	// 			}
	// 		}
	// 	}
	// }
};

export default zh_Hans;
