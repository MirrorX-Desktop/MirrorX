Write-Output "Build Libyuv"
trap {
    echo "failed"
}
# CMakeLists line 41 yuvconvert -> ${ly_lib_shared}

# Invoke-Expression -Command "git clone -b stable https://chromium.googlesource.com/libyuv/libyuv ./dependencies/libyuv"
# Invoke-Expression -Command "mkdir ./dependencies/libyuv_build"
Invoke-Expression -Command "cd ./dependencies/libyuv"
Invoke-Expression -Command "cmake -DCMAKE_BUILD_TYPE=Release ."
Invoke-Expression -Command "cmake --build . --config Release"
# Invoke-Expression -Command "cmake --build . --target install --config Release"