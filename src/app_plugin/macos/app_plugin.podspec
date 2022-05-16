#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint app_plugin.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
  s.name             = 'app_plugin'
  s.version          = '0.0.1'
  s.summary          = 'A new flutter plugin project.'
  s.description      = <<-DESC
  A new flutter plugin project.
  DESC
  s.homepage         = 'http://example.com'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'Your Company' => 'email@example.com' }
  s.source           = { :path => '.' }
  s.source_files     = 'Classes/**/*'
  s.public_header_files = 'Classes/bridge_generated.h'

  s.dependency 'FlutterMacOS'
  s.frameworks = 'AVFoundation', 'VideoToolbox'
  
  s.platform = :osx, '10.15'
  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES' } #
  s.swift_version = '5.0'
  s.static_framework = true # the metal shader library can't be founded if this enabled.
  s.vendored_libraries = "**.a"
end
