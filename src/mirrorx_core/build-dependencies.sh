#!/bin/sh -e

export MACOSX_DEPLOYMENT_TARGET=10.11

LOCAL_PATH="$(pwd)"
CPU_CORES="$(grep -c ^processor /proc/cpuinfo 2>/dev/null || sysctl -n hw.ncpu)"
GIT_CLONE_PREFIX="${LOCAL_PATH}/dependencies"

#################################
## install build tools
#################################

echo '🛠  \033[1mInstall build tools.\033[0m'

if which nasm | grep -q 'nasm'; then
  echo 'Skip nasm'
else
    brew install nasm
fi

if which yasm | grep -q 'yasm'; then
  echo 'Skip yasm'
else
    brew install yasm
fi

if which pkg-config | grep -q 'pkg-config'; then
  echo 'Skip pkg-config'
else
    brew install pkg-config
fi

if which autoconf | grep -q 'autoconf'; then
  echo 'Skip autoconf'
else
    brew install autoconf
fi

echo '✅ \033[1mInstall build tools finished.\033[0m'

#################################
## build libx264
#################################

echo '🛠  \033[1mBuild libx264.\033[0m'

if [ ! -d "${GIT_CLONE_PREFIX}/x264" ]; then
    git clone -b stable --depth 1 https://code.videolan.org/videolan/x264.git "${GIT_CLONE_PREFIX}/x264"
fi

if [ ! -d "${GIT_CLONE_PREFIX}/x264_build" ]; then
    cd "${GIT_CLONE_PREFIX}/x264"
    ./configure \
        --prefix="${GIT_CLONE_PREFIX}/x264_build" \
        --disable-cli \
        --enable-shared \
        --enable-pic \

    make "-j${CPU_CORES}" && make install
    make clean
    cd "${LOCAL_PATH}"
fi

echo '✅ \033[1mBuild libx264 finished.\033[0m'

#################################
## build libx265
#################################

echo '🛠  \033[1mBuild libx265.\033[0m'

if [ ! -d "${GIT_CLONE_PREFIX}/x265" ]; then
    git clone -b stable --depth 1 https://bitbucket.org/multicoreware/x265_git.git "${GIT_CLONE_PREFIX}/x265"
fi

if [ ! -d "${GIT_CLONE_PREFIX}/x265_build" ]; then
cd "${GIT_CLONE_PREFIX}/x265"
cmake -G "Unix Makefiles" -DCMAKE_INSTALL_PREFIX="${GIT_CLONE_PREFIX}/x265_build" -DENABLE_STATIC=OFF -DENABLE_SHARED=ON -DENABLE_SHARED_LIBS=ON -DENABLE_CLI=OFF ./source
make "-j${CPU_CORES}" && make install
make clean
cd "${LOCAL_PATH}"
fi

echo '✅ \033[1mBuild libx265 finished.\033[0m'

#################################
## build libopus
#################################

echo '🛠  \033[1mBuild libopus finished.\033[0m'

if [ ! -d "${GIT_CLONE_PREFIX}/opus" ]; then
    git clone -b v1.3.1 --depth 1 https://gitlab.xiph.org/xiph/opus.git "${GIT_CLONE_PREFIX}/opus"
fi


if [ ! -d "${GIT_CLONE_PREFIX}/opus_build" ]; then
cd "${GIT_CLONE_PREFIX}/opus"
./autogen.sh
./configure \
    --prefix="${GIT_CLONE_PREFIX}/opus_build" \
    --enable-shared \
    --disable-static \
    --disable-doc \
    --disable-extra-programs \

make "-j${CPU_CORES}" && make install
make clean
cd "${LOCAL_PATH}"
fi

echo '✅ \033[1mBuild libopus finished.\033[0m'

#################################
## build libvpx
#################################

echo '🛠  \033[1mBuild libvpx.\033[0m'

if [ ! -d "${GIT_CLONE_PREFIX}/libvpx" ]; then
    git clone -b v1.11.0 --depth 1 https://github.com/webmproject/libvpx.git "${GIT_CLONE_PREFIX}/libvpx"
fi

if [ ! -d "${GIT_CLONE_PREFIX}/libvpx_build" ]; then
cd "${GIT_CLONE_PREFIX}/libvpx"
./configure \
    --prefix="${GIT_CLONE_PREFIX}/libvpx_build" \
    --target="x86_64-darwin20-gcc" \
    --as=yasm \
    --enable-shared \

    # --enable-shared \

# --enable-pic \
    # --enable-shared \
    # --enable-pic \
    # --enable-better-hw-compatibility \
    # --enable-vp9 \
    # --disable-vp8 \
    # --enable-realtime-only \
    # --disable-webm-io \
    # --disable-libyuv \
    # --disable-unit-tests \

make "-j${CPU_CORES}" && make install
make clean
cd "${LOCAL_PATH}"
fi

echo '✅ \033[1mBuild libvpx finished.\033[0m'

################################
# build ffmpeg
################################

echo '🛠  \033[1mBuild ffmpeg.\033[0m'

if [ ! -d "${GIT_CLONE_PREFIX}/ffmpeg" ]; then
    git clone -b n5.0 --depth 1 https://git.ffmpeg.org/ffmpeg.git "${GIT_CLONE_PREFIX}/ffmpeg"
fi

if [ ! -d "${GIT_CLONE_PREFIX}/ffmpeg_build" ]; then
cd "${GIT_CLONE_PREFIX}/ffmpeg"
./configure \
    --disable-all \
    --disable-autodetect \
    --arch=x86_64 \
    --cc=/usr/bin/clang \
    --cxx=/usr/bin/clang++ \
    --target-os=darwin \
    --arch=x86_64 \
    --enable-lto \
    --prefix="${GIT_CLONE_PREFIX}/ffmpeg_build" \
    --enable-pic \
    --enable-lto \
    --enable-hardcoded-tables \
    --enable-shared \
    --enable-gpl \
    --enable-nonfree \
    --enable-version3 \
    --enable-avdevice \
    --enable-avcodec \
    --enable-avformat \
    --enable-pthreads \
    --enable-libx264 \
    --enable-libx265 \
    --enable-libvpx \
    --enable-libopus \
    --enable-videotoolbox \
    --enable-audiotoolbox \
    --enable-encoder=libx264 \
    --enable-decoder=h264 \
    --enable-encoder=libx265 \
    --enable-decoder=hevc \
    --enable-encoder=libvpx_vp9 \
    --enable-decoder=libvpx_vp9 \
    --enable-encoder=libopus \
    --enable-decoder=libopus \
    --enable-hwaccel=h264_videotoolbox \
    --enable-hwaccel=hevc_videotoolbox \
    --enable-hwaccel=vp9_videotoolbox \

    # --extra-cflags="-I${GIT_CLONE_PREFIX}/x264_build/include" \ 
    # --extra-ldflags="-L${GIT_CLONE_PREFIX}/x264_build/lib" \
    # --extra-cflags="-I${GIT_CLONE_PREFIX}/opus_build/include/opus" \
    # --extra-ldflags="-L${GIT_CLONE_PREFIX}/opus_build/lib" \
    # --extra-cflags="-I${GIT_CLONE_PREFIX}/libvpx_build/include/vpx" \
    # --extra-ldflags="-L${GIT_CLONE_PREFIX}/libvpx_build/lib" \
    # --extra-cflags="-I${GIT_CLONE_PREFIX}/x265_build/include" \
    # --extra-ldflags="-L${GIT_CLONE_PREFIX}/x265_build/lib" \

make "-j${CPU_CORES}" && make install
make clean
fi

echo '✅ \033[1mBuild ffmpeg finished.\033[0m'