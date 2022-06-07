//
//  Generated file. Do not edit.
//

import FlutterMacOS
import Foundation

import app_plugin
import device_info_plus_macos
import path_provider_macos
import screen_retriever
import window_manager

func RegisterGeneratedPlugins(registry: FlutterPluginRegistry) {
  AppPlugin.register(with: registry.registrar(forPlugin: "AppPlugin"))
  DeviceInfoPlusMacosPlugin.register(with: registry.registrar(forPlugin: "DeviceInfoPlusMacosPlugin"))
  PathProviderPlugin.register(with: registry.registrar(forPlugin: "PathProviderPlugin"))
  ScreenRetrieverPlugin.register(with: registry.registrar(forPlugin: "ScreenRetrieverPlugin"))
  WindowManagerPlugin.register(with: registry.registrar(forPlugin: "WindowManagerPlugin"))
}
