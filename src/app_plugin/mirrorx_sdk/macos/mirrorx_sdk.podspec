#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint mirrorx_sdk.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
  s.name             = 'mirrorx_sdk'
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
  s.dependency 'FlutterMacOS'

  s.platform = :osx, '10.11'
  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES' }
  s.swift_version = '5.0'

  # s.library = 'c++'
  # s.xcconfig={
  #   'CLANG_CXX_LANGUAGE_STANDARD' => 'c++11',
  #   'CLANG_CXX_LIBRARY' => 'libc++'
  # }
  
  # s.framework = 'VideoToolBox', 'AVFoundation'
  s.public_header_files = 'Classes/**/*.h'
  # s.static_framework = true
  s.vendored_libraries = '**/*.{a,dylib}'
end
