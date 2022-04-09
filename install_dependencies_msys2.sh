#!/bin/sh

CPU_CORES=$(grep </proc/cpuinfo -c "processor")

check_tool_installed() {
    printf "%-50s" "Check Tools: [$1]"
    tool_path=$(which "$1")
    if [ -z "$tool_path" ]; then
        echo "Failed"
        exit
    else
        printf "OK, found at: '%s'\r\n" "$tool_path"
    fi
}

check_already_built() {
    name=$1
    src_dir=$2
    dst_dir=$3

    printf "%-50s" "Check [$name] compile artificials"
    if [ ! "$(ls -A "$src_dir")" ]; then
        echo "Failed, source dir not exist"
        exit
    fi

    if [ -d "$dst_dir" ] && [ "$(ls -A "$dst_dir")" ]; then
        echo "OK, already exists, skip build"
        return 1
    else
        echo "Not exists, build it now"
        return 0
    fi
}

clone_source() {
    name=$1
    repo_url=$2
    branch=$3
    dst_dir=$4

    printf "%-50s" "Check Repository: [$name]"
    if [ -d "$dst_dir" ] && [ "$(ls -A "$dst_dir")" ]; then
        echo "OK, found at: $dst_dir"
    else
        echo "Not exists, clone repository"
        git clone -b "$branch" --depth=1 "$repo_url" "$dst_dir"
    fi
}

build_x264() {
    src_dir=$1
    dst_dir=$2
    absolute_dst_dir=$(readlink -f "$dst_dir")

    check_already_built "x264" "$src_dir" "$dst_dir"
    already_built=$?

    if [ "$already_built" = 1 ]; then
        return
    fi

    echo "Build x264..."
    cd "$src_dir" || exit

    CC=cl \
        ./configure \
        --prefix="$absolute_dst_dir" \
        --enable-pic \
        --enable-static \
        --enable-strip \
        --disable-cli \
        --disable-opencl

    make -j"$CPU_CORES" && make install && make clean

    cd ..
    echo "Build x264 success"
}

build_x265() {
    src_dir=$1
    dst_dir=$2
    absolute_dst_dir=$(readlink -f "$dst_dir")

    check_already_built "x265" "$src_dir" "$dst_dir"
    already_built=$?

    if [ "$already_built" = 1 ]; then
        return
    fi

    echo "Build x265..."
    cd "$src_dir" || exit

    cmake \
        ./source \
        -DCMAKE_INSTALL_PREFIX="$absolute_dst_dir" \
        -DENABLE_SHARED=OFF \
        -DENABLE_CLI=OFF \
        -DENABLE_PIC=ON \
        -DSTATIC_LINK_CRT=ON

    cmake --build . --config Release
    cmake --install .

    # modify name
    cp "$absolute_dst_dir"/lib/x265-static.lib "$absolute_dst_dir"/lib/x265.lib

    cd ..
    echo "Build x265 success"
}

build_opus() {
    src_dir=$1
    dst_dir=$2
    absolute_dst_dir=$(readlink -f "$dst_dir")

    check_already_built "opus" "$src_dir" "$dst_dir"
    already_built=$?

    if [ "$already_built" = 1 ]; then
        return
    fi

    echo "Build opus..."
    cd "$src_dir" || exit

    cmake \
        . \
        -DCMAKE_INSTALL_PREFIX="$absolute_dst_dir" \
        -DOPUS_STACK_PROTECTOR=OFF

    MSBuild.exe \
        -t:ReBuild \
        -p:Configuration=Release \
        -p:Platform=x64 \
        opus.vcxproj

    cmake --install .

    cd ..
    echo "Build opus success"
}

build_libvpx() {
    src_dir=$1
    dst_dir=$2
    absolute_dst_dir=$(readlink -f "$dst_dir")

    check_already_built "libvpx" "$src_dir" "$dst_dir"
    already_built=$?

    if [ "$already_built" = 1 ]; then
        return
    fi

    echo "Build libvpx..."
    cd "$src_dir" || exit

    ./configure \
        --prefix="$absolute_dst_dir" \
        --target=x86_64-win64-vs17 \
        --enable-static-msvcrt \
        --enable-vp9 \
        --enable-pic \
        --enable-better-hw-compatibility \
        --enable-realtime-only \
        --disable-vp8 \
        --disable-examples \
        --disable-docs \
        --disable-tools \
        --disable-unit-tests \
        --disable-webm-io \
        --disable-libyuv

    make -j"$CPU_CORES" && make install && make clean

    # gen pkg-config file
    gen_libvpx_pc "$absolute_dst_dir"

    # modify name
    cp "$absolute_dst_dir"/lib/x64/vpxmt.lib "$absolute_dst_dir"/lib/x64/vpx.lib

    cd ..
    echo "Build libvpx success"
}

build_amf() {
    src_dir=$1
    dst_dir=$2
    absolute_dst_dir=$(readlink -f "$dst_dir")

    check_already_built "amf" "$src_dir" "$dst_dir"
    already_built=$?

    if [ "$already_built" = 1 ]; then
        return
    fi

    echo "Build amf..."

    mkdir -p "$absolute_dst_dir"/include/AMF
    cp -r "$src_dir"/amf/public/include/* "$absolute_dst_dir"/include/AMF

    echo "Build amf success"
}

build_media_sdk() {
    src_dir=$1
    dst_dir=$2
    absolute_dst_dir=$(readlink -f "$dst_dir")

    check_already_built "media_sdk" "$src_dir" "$dst_dir"
    already_built=$?

    if [ "$already_built" = 1 ]; then
        return
    fi

    echo "Build opus..."
    cd "$src_dir" || exit

    # upgrade project
    devenv.exe ./api/mfx_dispatch/windows/libmfx_vs2015.vcxproj -upgrade

    MSBuild.exe \
        -t:ReBuild \
        -p:Configuration=Release \
        -p:Platform=x64 \
        -p:WindowsTargetPlatformVersion=10.0 \
        -p:OutDir="$absolute_dst_dir"/lib/ \
        ./api/mfx_dispatch/windows/libmfx_vs2015.vcxproj

    # copy include files
    mkdir -p "$absolute_dst_dir"/include/mfx
    cp -rf ./api/include/* "$absolute_dst_dir"/include/mfx

    # modify name
    cp "$absolute_dst_dir"/lib/libmfx_vs2015.lib "$absolute_dst_dir"/lib/mfx.lib

    # gen pkg-config file
    gen_libmfx_pc "$absolute_dst_dir"

    cd ..
    echo "Build media_sdk success"
}

build_nv_codec_headers() {
    src_dir=$1
    dst_dir=$2
    absolute_dst_dir=$(readlink -f "$dst_dir")

    check_already_built "nv_codec_headers" "$src_dir" "$dst_dir"
    already_built=$?

    if [ "$already_built" = 1 ]; then
        return
    fi

    echo "Build nv_codec_headers..."
    cd "$src_dir" || exit

    PREFIX="$absolute_dst_dir" make -j"$CPU_CORES" -e
    PREFIX="$absolute_dst_dir" make install -e

    echo "Build nv_codec_headers success"
}

build_ffmpeg() {
    artificials_root_dir=$(readlink -f "$1")
    src_dir=$2
    dst_dir=$3
    absolute_dst_dir=$(readlink -f "$dst_dir")

    check_already_built "ffmpeg" "$src_dir" "$dst_dir"
    already_built=$?

    if [ "$already_built" = 1 ]; then
        return
    fi

    echo "Build ffmpeg..."
    cd "$src_dir" || exit

    export PKG_CONFIG_PATH="$PKG_CONFIG_PATH":"$artificials_root_dir"/x264/lib/pkgconfig
    export PKG_CONFIG_PATH="$PKG_CONFIG_PATH":"$artificials_root_dir"/x265/lib/pkgconfig
    export PKG_CONFIG_PATH="$PKG_CONFIG_PATH":"$artificials_root_dir"/opus/lib/pkgconfig
    export PKG_CONFIG_PATH="$PKG_CONFIG_PATH":"$artificials_root_dir"/libvpx/lib/pkgconfig
    export PKG_CONFIG_PATH="$PKG_CONFIG_PATH":"$artificials_root_dir"/MediaSDK/lib/pkgconfig
    export PKG_CONFIG_PATH="$PKG_CONFIG_PATH":"$artificials_root_dir"/nv_codec_headers/lib/pkgconfig

    echo "List pkg-config libs"
    pkg-config --list-all

    echo "Configure ffmpeg compile configuration"
    set -x

    ./configure \
        --prefix="$absolute_dst_dir" \
        --disable-all \
        --disable-autodetect \
        --arch=x86_64 \
        --target-os=win64 \
        --toolchain=msvc \
        --pkg-config-flags=--static \
        --enable-stripping \
        --extra-cflags="-MT" \
        --disable-debug \
        --enable-d3d11va \
        --enable-w32threads \
        --enable-pic \
        --enable-hardcoded-tables \
        --enable-gpl \
        --enable-version3 \
        --enable-avdevice \
        --enable-avcodec \
        --enable-avformat \
        --enable-libx264 \
        --enable-libx265 \
        --enable-libvpx \
        --enable-libopus \
        --enable-encoder=libx264 \
        --enable-decoder=h264 \
        --enable-encoder=libx265 \
        --enable-decoder=hevc \
        --enable-encoder=libvpx_vp9 \
        --enable-decoder=libvpx_vp9 \
        --enable-encoder=libopus \
        --enable-decoder=libopus \
        --disable-doc \
        --disable-htmlpages \
        --disable-manpages \
        --disable-podpages \
        --disable-txtpages \
        --disable-network \
        --enable-cuvid \
        --enable-ffnvcodec \
        --enable-ffnvcodec \
        --enable-nvenc \
        --enable-nvdec \
        --enable-encoder=h264_nvenc \
        --enable-encoder=hevc_nvenc \
        --enable-decoder=h264_cuvid \
        --enable-decoder=hevc_cuvid \
        --enable-decoder=vp9_cuvid \
        --enable-amf \
        --enable-encoder=h264_amf \
        --enable-encoder=hevc_amf \
        --extra-cflags=-DAMF_CORE_STATIC \
        --extra-cflags=-I"$absolute_dst_dir"/../AMF/include \
        --enable-libmfx \
        --enable-encoder=h264_qsv \
        --enable-encoder=hevc_qsv \
        --enable-encoder=vp9_qsv \
        --enable-decoder=h264_qsv \
        --enable-decoder=hevc_qsv \
        --enable-decoder=vp9_qsv

    set +x

    make -j"$CPU_CORES" && make install && make clean

    cd ..
    echo "Build ffmpeg success"
}

gen_libvpx_pc() {
    build_dst_dir=$(readlink -f "$1")
    pc_path="$build_dst_dir"/lib/pkgconfig/vpx.pc

    rm -f "$pc_path"
    mkdir -p "$build_dst_dir"/lib/pkgconfig/
    touch "$pc_path"

    {
        echo "prefix=$build_dst_dir"
        echo '# pkg-config file from libvpx v1.11.0'
        echo "exec_prefix=\${prefix}"
        echo "libdir=\${prefix}/lib/x64"
        echo "includedir=\${prefix}/include"
        echo ""
        echo "Name: vpx"
        echo "Description: WebM Project VPx codec implementation"
        echo "Version: 1.11.0"
        echo "Libs: -L\"\${libdir}\" -lvpx"
        echo "Cflags: -I\"\${includedir}\""
    } >"$pc_path"
}

gen_libmfx_pc() {
    build_dst_dir=$(readlink -f "$1")
    pc_path="$build_dst_dir"/lib/pkgconfig/libmfx.pc

    rm -f "$pc_path"
    mkdir -p "$build_dst_dir"/lib/pkgconfig/
    touch "$pc_path"

    {
        echo "prefix=$build_dst_dir"
        echo "exec_prefix=\${prefix}"
        echo "libdir=\${prefix}/lib"
        echo "includedir=\${prefix}/include"
        echo ""
        echo "Name: libmfx"
        echo "Description: Intel Media SDK Dispatched static library"
        echo "Version: v1.3.5"
        echo "Libs: -L\"\${libdir}\" -lmfx -ladvapi32"
        echo "Cflags: -I\"\${includedir}\""
    } >"$pc_path"
}

echo "Environment:"
echo " - CPU CORES: $CPU_CORES"

check_tool_installed "git"
check_tool_installed "make"
check_tool_installed "libtool"
check_tool_installed "nasm"
check_tool_installed "autoconf"
check_tool_installed "pkg-config"

clone_source "x264" "https://code.videolan.org/videolan/x264.git" "stable" "./dependencies_repo/x264"
clone_source "x265" "https://bitbucket.org/multicoreware/x265_git.git" "3.5" "./dependencies_repo/x265"
clone_source "opus" "https://gitlab.xiph.org/xiph/opus.git" "v1.3.1" "./dependencies_repo/opus"
clone_source "libvpx" "https://github.com/webmproject/libvpx.git" "main" "./dependencies_repo/libvpx"
clone_source "AMF" "https://github.com/GPUOpen-LibrariesAndSDKs/AMF.git" "v1.4.24" "./dependencies_repo/AMF"
clone_source "MediaSDK" "https://github.com/Intel-Media-SDK/MediaSDK.git" "intel-mediasdk-22.3.0" "./dependencies_repo/MediaSDK"
clone_source "nv_codec_headers" "https://github.com/FFmpeg/nv-codec-headers.git" "n11.1.5.1" "./dependencies_repo/nv_codec_headers"
clone_source "ffmpeg" "https://git.ffmpeg.org/ffmpeg.git" "n5.0" "./dependencies_repo/ffmpeg"

build_x264 "./dependencies_repo/x264" "./dependencies_build/x264"
build_x265 "./dependencies_repo/x265" "./dependencies_build/x265"
build_opus "./dependencies_repo/opus" "./dependencies_build/opus"
build_libvpx "./dependencies_repo/libvpx" "./dependencies_build/libvpx"
build_amf "./dependencies_repo/AMF" "./dependencies_build/AMF"
build_media_sdk "./dependencies_repo/MediaSDK" "./dependencies_build/MediaSDK"
build_nv_codec_headers "./dependencies_repo/nv_codec_headers" "./dependencies_build/nv_codec_headers"
build_ffmpeg "./dependencies_build" "./dependencies_repo/ffmpeg" "./dependencies_build/ffmpeg"

echo "All dependencies has built successfully!"