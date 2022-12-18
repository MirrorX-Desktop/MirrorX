import type { BaseTranslation } from '../i18n-types';

const en: BaseTranslation = {
	DialogActions: {
		Ok: 'OK',
		Cancel: 'CANCEL',
		Yes: 'YES',
		No: 'NO',
		Allow: 'ALLOW',
		Reject: 'REJECT'
	},
	Home: {
		Domain: 'Domain',
		DeviceID: 'DeviceID',
		RemoteDeviceID: 'Remote Device ID',
		Password: 'Password',
		Connect: 'Connect',
		Desktop: 'Desktop',
		Files: 'Files',
		EditPasswordTooltip: 'Edit Password',
		PasswordVisibleTooltip: 'Click to show plain password',
		PasswordInvisibleTooltip: 'Hide password when mouse leave',
		EditPasswordCancelTooltip: 'Cancel',
		ClickToCopyDeviceIDTooltip: 'Click to Copy',
		ClickToCopyDeviceIDCopiedTooltip: 'Copied',
		GenerateRandomPasswordTooltip: 'Generate Random Password'
		// Layout: {
		// 	Domain: 'Domain',
		// 	Connect: 'Connect',
		// 	LAN: 'LAN',
		// 	History: 'History',
		// 	Menu: {
		// 		Tooltip: 'Settings',
		// 		Settings: 'Settings',
		// 		Language: 'Language'
		// 	},
		// 	Dialog: {
		// 		SelectLanguage: {
		// 			Title: 'Select Language'
		// 		}
		// 	}
		// },
		// Pages: {
		// 	Connect: {
		// 		DeviceID: 'DeviceID',
		// 		Password: 'Password',
		// 		RemoteDeviceIDPlaceHolder: 'Remote Device ID',
		// 		Desktop: 'Desktop',
		// 		Files: 'Files',
		// 		Tooltips: {
		// 			EditPassword: 'Edit Password',
		// 			PasswordVisible: 'Click to show plain password',
		// 			PasswordInvisible: 'Hide password when mouse leave',
		// 			EditPasswordCancel: 'Cancel',
		// 			ClickToCopyDeviceID: 'Click to Copy',
		// 			ClickToCopyDeviceIDCopied: 'Copied'
		// 		},
		// 		Dialog: {
		// 			InputRemotePassword: {
		// 				Title: 'Input Password',
		// 				ContentPrefix: 'Remote Device',
		// 				ContentSuffix: 'accept your visit request, please input remote device password here'
		// 			},
		// 			VisitRequest: {
		// 				Title: 'VisitRequest',
		// 				ContentPrefix: 'Remote Device',
		// 				ContentSuffix: 'want to visit your',
		// 				ResourceType: {
		// 					Desktop: 'Desktop',
		// 					Files: 'Files'
		// 				}
		// 			}
		// 		}
		// 	}
		// }
	},
	LAN: {
		HostnameOrIP: 'Hostname or IP',
		Discoverable: 'Discoverable'
	},
	Settings: {
		Appearance: {
			Title: 'Appearance',
			Theme: 'Theme',
			Light: 'Light',
			Dark: 'Dark',
			Auto: 'Auto'
		}
	},
	Dialogs: {
		About: {
			Copy: 'Copy',
			Version: 'Version'
		},
		VisitPrepare: {
			Content: "Please input this device's password"
		},
		LANConnect: {
			Content: 'Do you want to connect this device?'
		},
		SelectLanguage: {
			Title: 'Select Language'
		}
	}
	// Settings: {
	// 	WindowTitle: 'Settings',
	// 	Layout: {
	// 		Domain: 'Domain',
	// 		About: 'About'
	// 	},
	// 	Pages: {
	// 		Domain: {
	// 			Current: 'Current:',
	// 			Tooltips: {
	// 				Add: 'Add new domain',
	// 				SetPrimary: 'Set as primary domain',
	// 				Edit: 'Edit',
	// 				Delete: 'Delete'
	// 			}
	// 		},
	// 		About: {
	// 			Version: 'Version',
	// 			Official: 'Official',
	// 			SourceRepository: 'Source Repository',
	// 			SupportAndHelp: 'Support&Help'
	// 		},
	// 		Dialog: {
	// 			EditDomain: {
	// 				Title: 'Edit Domain',
	// 				Name: 'Name',
	// 				DeviceId: 'Device Id',
	// 				FingerPrint: {
	// 					Label: 'FingerPrint',
	// 					Tooltip: `Finger print is a random string generated at local once you connected to a new domain. It is used to prove your device has authority to hold a Device Id that Domain allocated for you for a while and it can't be used to track your device.`
	// 				},
	// 				Remarks: 'Remarks'
	// 			},
	// 			AddDomain: {
	// 				Title: 'Add Domain',
	// 				AddressInputPlaceHolder: 'Domain Address (IP:Port or URL)',
	// 				RemarksInputPlaceHolder: 'Remarks'
	// 			},
	// 			SetPrimaryDomain: {
	// 				Title: 'Set Primary Domain',
	// 				ContentPrefix: 'Do you want to set',
	// 				ContentSuffix: 'as primary domain?'
	// 			},
	// 			DeleteDomain: {
	// 				Title: 'Delete Domain',
	// 				ContentPrefix: 'Do you want to delete domain',
	// 				ContentSuffix: `? Once you delete it and you can't recovery!`
	// 			}
	// 		}
	// 	}
	// }
};

export default en;
