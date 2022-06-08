#
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint mirrorx_core.podspec` to validate before publishing.
#
Pod::Spec.new do |s|
  s.name             = 'mirrorx_core'
  s.version          = '0.0.1'
  s.summary          = 'A new Flutter FFI plugin project.'
  s.description      = <<-DESC
A new Flutter FFI plugin project.
                       DESC
  s.homepage         = 'http://example.com'
  s.license          = { :file => '../LICENSE' }
  s.author           = { 'Your Company' => 'email@example.com' }

  # This will ensure the source files in Classes/ are included in the native
  # builds of apps using this FFI plugin. Podspec does not support relative
  # paths, so Classes contains a forwarder C file that relatively imports
  # `../src/*` so that the C sources can be shared among all target platforms.
  s.source           = { :path => '.' }
  s.source_files     = 'Classes/**/*'
  s.dependency 'FlutterMacOS'
  s.frameworks = 'CoreMedia', 'VideoToolbox'
  
  s.platform = :osx, '10.11'
  s.swift_version = '5.0'
  s.prepare_command = <<-CMD
    cd ../../../core && cargo build --release --target x86_64-apple-darwin
    CMD

  s.pod_target_xcconfig = { 'DEFINES_MODULE' => 'YES','OTHER_LDFLAGS' => ['-lcore', '-lavcodec','-lavutil','-lx264'], 'LIBRARY_SEARCH_PATHS' => ['${PODS_ROOT}/../../../core/target/x86_64-apple-darwin/release', '${PODS_ROOT}/../../../third/dependencies_build/ffmpeg/lib', '${PODS_ROOT}/../../../third/dependencies_build/x264/lib'] }
  s.vendored_libraries = 'core', 'avcodec', 'avutil', 'x264'
end
