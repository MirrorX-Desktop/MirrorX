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
		Files: 'File Transfer',
		EditPasswordTooltip: 'Edit Password',
		PasswordVisibleTooltip: 'Click to show plain password',
		PasswordInvisibleTooltip: 'Hide password when mouse leave',
		EditPasswordCancelTooltip: 'Cancel',
		ClickToCopyDeviceIDTooltip: 'Click to Copy',
		ClickToCopyDeviceIDCopiedTooltip: 'Copied',
		GenerateRandomPasswordTooltip: 'Generate Random Password',
		DomainActions: 'Domain Actions',
		DomainActionsEdit: 'Edit',
		SelectPrimaryDomain: 'Select Primary Domain'
	},
	LAN: {
		HostnameOrIP: 'Search Hostname or IP (Case Sensitive)',
		Discoverable: 'Discoverable',
		DiscoveredDevicesTip: 'List of LAN Discovered Devices'
	},
	History: {
		All: 'All',
		Day1: '1 Day',
		Days7: '7 Days',
		Days30: '30 Days',
		KeptTip: 'Records are kept for up to 60 days'
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
	FileTransfer: {
		WindowTitle: 'File Transfer',
		Local: 'Local',
		Remote: 'Remote',
		GotoInput: {
			RecentRecords: 'Recent 10 records',
			Empty: 'Empty'
		},
		View: {
			TableName: 'Name',
			TableModifiedTime: 'Modified Date',
			TableSize: 'Size',

			ContextMenu: {
				SendToRemote: 'Send to Remote',
				DownloadToLocal: 'Download to Local'
			}
		},
		Transfer: {
			Transferring: 'Transferring',
			TransferSucceed: 'Transfer Succeed',
			TransferFailed: 'Transfer Failed',
			TableLocalPath: 'Local Path',
			TableRemotePath: 'Remote Path',
			TableTransferredAndTotalSize: 'Transferred/Total Size',
			TableProgress: 'Progress',
			TableStatus: 'Status',
			TableTotalSize: 'Total Size',
			TableFinishAt: 'Finish At',
			TableTimeCost: 'Time Cost'
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
		},
		DomainList: {
			Current: 'Current:',
			Tooltips: {
				Add: 'Add new domain',
				SetPrimary: 'Set as primary domain',
				Edit: 'Edit',
				Delete: 'Delete'
			}
		},
		DomainEdit: {
			Title: 'Edit Domain',
			Name: 'Name',
			DeviceId: 'Device Id',
			FingerPrint: {
				Label: 'FingerPrint',
				Tooltip: `Finger print is a random string generated at local once you connected to a new domain. It is used to prove your device has authority to hold a Device Id that Domain allocated for you for a while and it can't be used to track your device.`
			},
			Remarks: 'Remarks',
			Delete: 'Delete',
			Edit: 'Edit'
		},
		DomainAdd: {
			Title: 'Add Domain',
			AddressInputPlaceHolder: 'Domain Address (IP:Port or URL)',
			RemarksInputPlaceHolder: 'Remarks'
		},
		DomainSwitch: {
			Title: 'Set Primary Domain',
			ContentPrefix: 'Do you want to set',
			ContentSuffix: 'as primary domain?'
		},
		DomainDelete: {
			Title: 'Delete Domain',
			ContentPrefix: 'Do you want to delete domain',
			ContentSuffix: `? Once you delete it and you can't recovery!`
		},
		HistoryConnect: {
			Tip: 'Automatically Switch Domain'
		}
	},
	FileType: {
		File: 'File',
		Application: 'Application',
		CompressArchive: 'Compress Archive'
	}
};

export default en;
