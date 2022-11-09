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
		Layout: {
			Domain: '域',
			Connect: '连接',
			LAN: '局域网',
			History: '历史',
			Menu: {
				Tooltip: '设置',
				Settings: '设置',
				Language: '语言'
			},
			Dialog: {
				SelectLanguage: {
					Title: '选择语言'
				}
			}
		},
		Pages: {
			Connect: {
				DeviceID: '设备ID',
				Password: '密码',
				RemoteDeviceIDPlaceHolder: '远程设备ID',
				Desktop: '桌面',
				Files: '文档',
				Tooltips: {
					EditPassword: '修改密码',
					PasswordVisible: '点击以显示密码',
					PasswordInvisible: '移开鼠标隐藏密码',
					EditPasswordCancel: '取消'
				},
				Dialog: {
					InputRemotePassword: {
						Title: '输入密码',
						ContentPrefix: '远程设备',
						ContentSuffix: '接受了你的访问请求，请在这里输入远程设备密码'
					},
					VisitRequest: {
						Title: '请求访问',
						ContentPrefix: '远程设备',
						ContentSuffix: '想要访问你的',
						ResourceType: {
							Desktop: '桌面',
							Files: '文档'
						}
					}
				}
			}
		}
	},
	Settings: {
		WindowTitle: '设置',
		Layout: {
			Domain: '域',
			About: '关于'
		},
		Pages: {
			Domain: {
				Current: '当前：',
				Tooltips: {
					Add: '增加新的域',
					SetPrimary: '设置为主域',
					Edit: '编辑',
					Delete: '删除'
				}
			},
			Dialog: {
				EditDomain: {
					Title: '修改域',
					Name: '名称',
					DeviceId: '设备ID',
					FingerPrint: {
						Label: '指纹',
						Tooltip: `Finger print is a random string generated at local once you connected to a new domain. It is used to prove your device has authority to hold a Device Id for a while and it can't be used to track your device.`
					},
					Remarks: '备注'
				},
				AddDomain: {
					Title: '增加域',
					AddressInputPlaceHolder: '域地址（IP:端口 或 链接）',
					RemarksInputPlaceHolder: '备注'
				},
				SetPrimaryDomain: {
					Title: '设为主域',
					ContentPrefix: '你想要设置',
					ContentSuffix: '为主域吗？'
				},
				DeleteDomain: {
					Title: '删除域',
					ContentPrefix: '你想要删除域',
					ContentSuffix: `吗？一旦你删除将无法恢复！`
				}
			}
		}
	}
};

export default zh_Hans;
