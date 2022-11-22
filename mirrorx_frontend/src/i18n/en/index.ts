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
		Layout: {
			Domain: 'Domain',
			Connect: 'Connect',
			LAN: 'LAN',
			History: 'History',
			Menu: {
				Tooltip: 'Settings',
				Settings: 'Settings',
				Language: 'Language'
			},
			Dialog: {
				SelectLanguage: {
					Title: 'Select Language'
				}
			}
		},
		Pages: {
			Connect: {
				DeviceID: 'DeviceID',
				Password: 'Password',
				RemoteDeviceIDPlaceHolder: 'Remote Device ID',
				Desktop: 'Desktop',
				Files: 'Files',
				Tooltips: {
					EditPassword: 'Edit Password',
					PasswordVisible: 'Click to show plain password',
					PasswordInvisible: 'Hide password when mouse leave',
					EditPasswordCancel: 'Cancel',
					ClickToCopyDeviceID: 'Click to Copy',
					ClickToCopyDeviceIDCopied: 'Copied'
				},
				Dialog: {
					InputRemotePassword: {
						Title: 'Input Password',
						ContentPrefix: 'Remote Device',
						ContentSuffix: 'accept your visit request, please input remote device password here'
					},
					VisitRequest: {
						Title: 'VisitRequest',
						ContentPrefix: 'Remote Device',
						ContentSuffix: 'want to visit your',
						ResourceType: {
							Desktop: 'Desktop',
							Files: 'Files'
						}
					}
				}
			}
		}
	},
	Settings: {
		WindowTitle: 'Settings',
		Layout: {
			Domain: 'Domain',
			About: 'About'
		},
		Pages: {
			Domain: {
				Current: 'Current:',
				Tooltips: {
					Add: 'Add new domain',
					SetPrimary: 'Set as primary domain',
					Edit: 'Edit',
					Delete: 'Delete'
				}
			},
			Dialog: {
				EditDomain: {
					Title: 'Edit Domain',
					Name: 'Name',
					DeviceId: 'Device Id',
					FingerPrint: {
						Label: 'FingerPrint',
						Tooltip: `Finger print is a random string generated at local once you connected to a new domain. It is used to prove your device has authority to hold a Device Id that Domain allocated for you for a while and it can't be used to track your device.`
					},
					Remarks: 'Remarks'
				},
				AddDomain: {
					Title: 'Add Domain',
					AddressInputPlaceHolder: 'Domain Address (IP:Port or URL)',
					RemarksInputPlaceHolder: 'Remarks'
				},
				SetPrimaryDomain: {
					Title: 'Set Primary Domain',
					ContentPrefix: 'Do you want to set',
					ContentSuffix: 'as primary domain?'
				},
				DeleteDomain: {
					Title: 'Delete Domain',
					ContentPrefix: 'Do you want to delete domain',
					ContentSuffix: `? Once you delete it and you can't recovery!`
				}
			}
		}
	}
};

export default en;
