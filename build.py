import os
from pathlib import Path
import subprocess
import platform
from enum import Enum


class OSType(Enum):
    Windows = 1,
    macOS = 2,
    Linux = 3


def getCPUCores():
    num = os.cpu_count()
    print(f"cpu num {num}")
    return num


def getOSType():
    osType = platform.system()
    if osType == "Windows":
        return OSType.Windows
    elif osType == "Darwin":
        return OSType.macOS
    else:
        return OSType.Linux


def checkBasicBuildToolsAvailable(osType: OSType, toolName: str):
    try:
        print(f"   💡 Checking for [{toolName}]...")
        output = ''
        if osType == OSType.Windows:
            output = subprocess.check_output(
                f"where.exe {toolName}",  shell=True)
        else:
            output = subprocess.check_output(f"which {toolName}",  shell=True)

        path = bytes.decode(output).replace('\n', '')
        print(f"   ✅ [{toolName}] is available at: {path}")
    except subprocess.CalledProcessError:
        print(
            f"   ❌ [{toolName}] cannot be found. Please install or add it's path to environment.")
        exit()


def vcpkgIntegration():
    subprocess.call(["vcpkg", "integrate", "install"])


def checkAndInstallBuildTools(osType: OSType, toolName: str):
    try:
        print(f"   💡 Checking for [{toolName}]...")
        output = ''
        if osType == OSType.Windows:
            output = subprocess.check_output(
                f"where.exe {toolName}",  shell=True)
        else:
            output = subprocess.check_output(f"which {toolName}",  shell=True)

        path = bytes.decode(output).replace('\n', '')
        print(f"   ✅ [{toolName}] is available at: {path}")
    except subprocess.CalledProcessError:
        print(f"   ⚙️ installing [{toolName}]...")
        try:
            if osType == OSType.macOS:
                output = subprocess.call(
                    f"brew install {toolName}", shell=True)
            else:
                output = subprocess.call(
                    f"vcpkg install {toolName}", shell=True)
                vcpkgIntegration()
        except subprocess.CalledProcessError:
            print(f"   ❌ [{toolName}] install failed.")
            print(bytes.decode(output).replace('\n', ''))


def cloneRepo(repoName: str, repoURL: str, branch: str, targetDir: str):
    print(f"   💡 Checking if repo [{repoName}] exists...")

    dir = Path(targetDir)
    if dir.exists() and os.listdir(dir.resolve()).__sizeof__() > 0:
        print(f"   ✅ Repo [{repoName}] exists. Skipping clone.")
        return

    print(f"   📥 Repo not exists. Cloning repo [{repoName}]...")
    try:
        subprocess.check_output(
            f"git clone -b {branch} --depth=1 {repoURL} {targetDir}", shell=True)
        print("   ✅ Repo cloned successfully.")
    except subprocess.CalledProcessError:
        print(f"   ❌ git clone [{repoName}] failed")
        exit()


def buildX264(sourceDir: str, outputDir: str):
    print("   💡 Building [x264]...")
    inputPath = Path(sourceDir)
    if not inputPath.exists():
        print("   ❌ [x264] source dir does not exist.")
        exit()

    outputPath = Path(outputDir)
    print(f"   👉 [x264] output dir: {outputPath.resolve()}")
    if outputPath.exists() and os.listdir(outputPath.resolve()).__sizeof__() > 0:
        print(f"   ✅ [x264] compile product exists. Skipping build.")
        return

    subprocess.call(args=["./configure", f"--prefix={outputPath.resolve()}",
                          "--enable-pic",
                          "--enable-static",
                          "--disable-cli"], cwd=sourceDir)
    subprocess.call(args=["make", f"-j{getCPUCores()}"], cwd=sourceDir)
    subprocess.call(args=["make", "install"], cwd=sourceDir)
    subprocess.call(args=["make", "clean"], cwd=sourceDir)

    print("   ✅ [x264] build completed.")


def buildX265(sourceDir: str, outputDir: str):
    print("   💡 Building [x265]...")
    inputPath = Path(sourceDir)
    if not inputPath.exists():
        print("   ❌ [x265] source dir does not exist.")
        exit()

    outputPath = Path(outputDir)
    print(f"   👉 [x265] output dir: {outputPath.resolve()}")
    if outputPath.exists() and os.listdir(outputPath.resolve()).__sizeof__() > 0:
        print(f"   ✅ [x264] compile product exists. Skipping build.")
        return

    subprocess.call(args=["cmake",
                          "-G",
                          "Unix Makefiles",
                          f"-DCMAKE_INSTALL_PREFIX={outputPath.resolve()}",
                          "-DENABLE_STATIC=ON",
                          "-DENABLE_SHARED=OFF",
                          "-DENABLE_SHARED_LIBS=OFF",
                          "-DENABLE_CLI=OFF",
                          "./source"], cwd=sourceDir)
    subprocess.call(args=["make", f"-j{getCPUCores()}"], cwd=sourceDir)
    subprocess.call(args=["make", "install"], cwd=sourceDir)
    subprocess.call(args=["make", "clean"], cwd=sourceDir)

    print("   ✅ [x265] build completed.")


def buildOpus(sourceDir: str, outputDir: str):
    print("   💡 Building [opus]...")
    inputPath = Path(sourceDir)
    if not inputPath.exists():
        print("   ❌ [opus] source dir does not exist.")
        exit()

    outputPath = Path(outputDir)
    print(f"   👉 [opus] output dir: {outputPath.resolve()}")
    if outputPath.exists() and os.listdir(outputPath.resolve()).__sizeof__() > 0:
        print(f"   ✅ [opus] compile product exists. Skipping build.")
        return

    subprocess.call(args=["./autogen.sh"], cwd=sourceDir)
    subprocess.call(args=["./configure", f"--prefix={outputPath.resolve()}",
                          "--enable-static",
                          "--disable-shared",
                          "--disable-doc",
                          "--disable-extra-programs"], cwd=sourceDir)
    subprocess.call(args=["make", f"-j{getCPUCores()}"], cwd=sourceDir)
    subprocess.call(args=["make", "install"], cwd=sourceDir)
    subprocess.call(args=["make", "clean"], cwd=sourceDir)

    print("   ✅ [opus] build completed.")


def buildLibvpx(sourceDir: str, outputDir: str):
    print("   💡 Building [libvpx]...")
    inputPath = Path(sourceDir)
    if not inputPath.exists():
        print("   ❌ [libvpx] source dir does not exist.")
        exit()

    outputPath = Path(outputDir)
    print(f"   👉 [libvpx] output dir: {outputPath.resolve()}")
    if outputPath.exists() and os.listdir(outputPath.resolve()).__sizeof__() > 0:
        print(f"   ✅ [libvpx] compile product exists. Skipping build.")
        return

    subprocess.call(args=["./configure", f"--prefix={outputPath.resolve()}",
                          "--enable-vp9",
                          "--enable-pic",
                          "--enable-better-hw-compatibility",
                          "--enable-realtime-only",
                          "--disable-vp8",
                          "--disable-examples",
                          "--disable-tools",
                          "--disable-docs"], cwd=sourceDir)
    subprocess.call(args=["make", f"-j{getCPUCores()}"], cwd=sourceDir)
    subprocess.call(args=["make", "install"], cwd=sourceDir)
    subprocess.call(args=["make", "clean"], cwd=sourceDir)

    print("   ✅ [libvpx] build completed.")


def buildFFmpeg(osType: OSType, sourceDir: str, outputDir: str):
    print("   💡 Building [ffmpeg]...")
    inputPath = Path(sourceDir)
    if not inputPath.exists():
        print("   ❌ [ffmpeg] source dir does not exist.")
        exit()

    outputPath = Path(outputDir)
    print(f"   👉 [ffmpeg] output dir: {outputPath.resolve()}")
    if outputPath.exists() and os.listdir(outputPath.resolve()).__sizeof__() > 0:
        print(f"   ✅ [ffmpeg] compile product exists. Skipping build.")
        return

    args = ["./configure", f"--prefix={outputPath.resolve()}",
            "--disable-all",
            "--disable-autodetect",
            "--arch=x86_64",
            "--enable-lto",
            "--enable-pic",
            "--enable-hardcoded-tables",
            "--enable-gpl",
            "--enable-nonfree",
            "--enable-version3",
            "--enable-avdevice",
            "--enable-avcodec",
            "--enable-avformat",
            "--enable-pthreads",
            "--enable-libx264",
            "--enable-libx265",
            "--enable-libvpx",
            "--enable-libopus",
            "--enable-encoder=libx264",
            "--enable-decoder=h264",
            "--enable-encoder=libx265",
            "--enable-decoder=hevc",
            "--enable-encoder=libvpx_vp9",
            "--enable-decoder=libvpx_vp9",
            "--enable-encoder=libopus",
            "--enable-decoder=libopus",
            "--disable-doc",
            "--disable-htmlpages",
            "--disable-manpages",
            "--disable-podpages",
            "--disable-txtpages",
            ]

    if osType == OSType.macOS:
        args.append("--enable-videotoolbox")
        args.append("--enable-audiotoolbox")
        args.append("--enable-hwaccel=h264_videotoolbox")
        args.append("--enable-hwaccel=hevc_videotoolbox")
        args.append("--enable-hwaccel=vp9_videotoolbox")

    subprocess.call(args=args, cwd=sourceDir)
    subprocess.call(args=["make", f"-j{getCPUCores()}"], cwd=sourceDir)
    subprocess.call(args=["make", "install"], cwd=sourceDir)
    subprocess.call(args=["make", "clean"], cwd=sourceDir)

    print("   ✅ [ffmpeg] build completed.")


# Build Process
# print platform
osType = getOSType()
print("📦 Building for: " + osType.name)


# check build tool dependencies
print("📋 Checking build tools...")

if osType == OSType.macOS:
    checkBasicBuildToolsAvailable(osType, "brew")

checkBasicBuildToolsAvailable(osType, "git")
checkBasicBuildToolsAvailable(osType, "vcpkg")

checkAndInstallBuildTools(osType, "yasm")
checkAndInstallBuildTools(osType, "autoconf")

if osType == OSType.macOS:
    checkBasicBuildToolsAvailable(osType, "pkg-config")
else:
    checkBasicBuildToolsAvailable(osType, "pkgconf")


# clone repo
print("📁 Cloning repo...")
cloneRepo("x264", "https://code.videolan.org/videolan/x264.git",
          "stable", "./dependencies_repo/x264")

cloneRepo("x265", "https://bitbucket.org/multicoreware/x265_git.git",
          "stable", "./dependencies_repo/x265")

cloneRepo("opus", "https://gitlab.xiph.org/xiph/opus.git",
          "master", "./dependencies_repo/opus")

cloneRepo("libvpx", "https://chromium.googlesource.com/webm/libvpx",
          "v1.11.0", "./dependencies_repo/libvpx")

cloneRepo("ffmpeg", "https://git.ffmpeg.org/ffmpeg.git",
          "n5.0", "./dependencies_repo/ffmpeg")

# build dependencies
print("📦 Building...")

buildX264("./dependencies_repo/x264", "./dependencies_build/x264")
buildX265("./dependencies_repo/x265", "./dependencies_build/x265")
buildOpus("./dependencies_repo/opus", "./dependencies_build/opus")
buildLibvpx("./dependencies_repo/libvpx", "./dependencies_build/libvpx")
buildFFmpeg(osType, "./dependencies_repo/ffmpeg",
            "./dependencies_build/ffmpeg")

print("✅ All dependencies has build successfully.")
