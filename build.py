import os
from pathlib import Path
import subprocess
import platform
from enum import Enum
from tokenize import PlainToken


class OSType(Enum):
    Windows = 1,
    macOS = 2,
    Linux = 3,
    MSYS = 4


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
    elif osType.__contains__("MSYS_NT"):
        return OSType.MSYS
    else:
        return OSType.Linux


def checkPackageManager(osType: OSType):
    pm = ''
    if osType == OSType.Windows:
        print("❌ This script can't run in native Windows environment!")
        exit()
    elif osType == OSType.macOS:
        pm = "brew"
    elif osType == OSType.MSYS:
        pm = "pacman"
    elif osType == OSType.Linux:
        pm = "apt-get"

    try:
        pmPath = subprocess.check_output(["which", pm])
        pmPath = bytes.decode(pmPath).replace("\n", "")
        print(f"✔️ Package Manager [{pm}] found in: {pmPath}")
    except subprocess.CalledProcessError:
        print(f"❌ Package Manager [{pm}] not found")
        exit()


def buildPackageManagerInstallCommand(osType: OSType, toolName: str):
    if osType == OSType.Windows:
        print("❌ This script can't run in native Windows environment!")
        exit()
    elif osType == OSType.macOS:
        return ["brew", "install", toolName]
    elif osType == OSType.MSYS:
        return ["pacman", "-S", toolName]
    elif osType == OSType.Linux:
        return ["apt-get", "install", toolName]


def installBuildTools(osType: OSType, toolName: str):
    print(f"💡 Check [{toolName}] installation...")
    try:
        output = subprocess.check_output(["which", toolName])
        output = bytes.decode(output).replace('\n', '')
        print(f"✔️ [{toolName}] is available at: {output}")
    except subprocess.CalledProcessError:
        print(f"⚙️  Installing [{toolName}]...")
        try:
            args = buildPackageManagerInstallCommand(osType, toolName)
            output = subprocess.check_output(args=args)
            output = bytes.decode(output).replace('\n', '')
            print(f"✔️ [{toolName}] is available at: {output}")
        except subprocess.CalledProcessError:
            print(f"❌ [{toolName}] install failed.")
            exit()


def cloneRepo(repoName: str, repoURL: str, branch: str, targetDir: str):
    print(f"💡 Checking if repo [{repoName}] exists...")

    dir = Path(targetDir)
    if dir.exists() and os.listdir(dir.resolve()).__sizeof__() > 0:
        print(f"✔️ Repo [{repoName}] exists. Skipping clone.")
        return

    print(f"📥 Repo [{repoName}] not exists. Cloning...")

    try:
        subprocess.check_output(
            ["git", "clone", "-b", branch, "--depth=1", repoURL, targetDir])
        print(f"✔️  Repo [{repoName}] cloned successfully.")
    except subprocess.CalledProcessError:
        print(f"❌ Clone [{repoName}] failed")
        exit()

# def attachVisualStudioDeveloperTool(osType:OSType, args:list[str]):
#     if osType != OSType.MSYS:
#         return

#     path = Path("C:/Program Files (x86)/Microsoft Visual Studio/Installer/vswhere.exe")

#     if not path.exists():
#         print("❌ Visual Studio version must grater than or equals Visual Studio 2017 version 15.2 and later")
#         return

#     vsInstallationPath=bytes.decode(subprocess.check_output([path,"-prerelease", "-latest", "-property", "installationPath"])).replace("\r\n","")
#     vsDevCmdPath = Path(vsInstallationPath).joinpath("VC").joinpath("Auxiliary").joinpath("Build").joinpath("vcvarsall.bat")
#     args.insert(0,"&&")
#     args.insert(0,"x64")
#     args.insert(0,vsDevCmdPath.__str__())
#     return args


def buildX264(osType: OSType, sourceDir: str, outputDir: str):
    print("💡 Building [x264]...")
    inputPath = Path(sourceDir)
    if not inputPath.exists():
        print("❌ [x264] Source dir does not exist.")
        exit()

    outputPath = Path(outputDir)
    print(f"👉 [x264] Output dir: {outputPath.resolve()}")
    if outputPath.exists() and os.listdir(outputPath.resolve()).__sizeof__() > 0:
        print(f"✔️  [x264] Compile product exists. Skipping build.")
        return

    args = ["./configure", f"--prefix={outputPath.resolve()}",
            "--enable-pic",
            "--enable-static",
            "--disable-cli"]

    if osType == OSType.MSYS:
        args.insert(0, "CC=cl")

    command = ' '.join(str(i) for i in args)
    print(f"CMD: {command}")

    try:
        subprocess.call(command, cwd=sourceDir, shell=True)
        subprocess.call(args=["make", f"-j{getCPUCores()}"], cwd=sourceDir)
        subprocess.call(args=["make", "install"], cwd=sourceDir)
        subprocess.call(args=["make", "clean"], cwd=sourceDir)

        print("✔️  [x264] build completed.")
    except subprocess.CalledProcessError:
        print("❌ [x264] build failed.")
        exit()


def buildX265(osType: OSType, sourceDir: str, outputDir: str):
    print("💡 Building [x265]...")
    inputPath = Path(sourceDir)
    if not inputPath.exists():
        print("❌ [x265] source dir does not exist.")
        exit()

    outputPath = Path(outputDir)
    print(f"👉 [x265] output dir: {outputPath.resolve()}")
    if outputPath.exists() and os.listdir(outputPath.resolve()).__sizeof__() > 0:
        print(f"✔️ [x265] compile product exists. Skipping build.")
        return

    makeFilesType = "Unix Makefiles"
    if osType == OSType.MSYS:
        makeFilesType = "Visual Studio 17 2022"

    args = ["cmake",
            "-G",
            makeFilesType,
            f"-DCMAKE_INSTALL_PREFIX={outputPath.resolve()}",
            "-DENABLE_SHARED=OFF",
            "-DENABLE_CLI=OFF",
            "./source",
            ]

    try:
        subprocess.call(args, cwd=sourceDir)
        subprocess.call(["MSBuild.exe", "INSTALL.vcxproj", "/property:Configuration=Release",
                        f"/maxCpuCount:{getCPUCores()}"], cwd=sourceDir)

        print("✔️ [x265] build completed.")
    except subprocess.CalledProcessError:
        print("❌ [x265] build failed")
        exit()


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

if osType == OSType.Windows:
    print("❌ This script can't run in native Windows environment, please install MSYS2 and re-run it in MSYS2 environment!")

print("📦 Building for: " + osType.name)

checkPackageManager(osType)

installBuildTools(osType, "git")
installBuildTools(osType, "make")
installBuildTools(osType, "cmake")
installBuildTools(osType, "nasm")
installBuildTools(osType, "autoconf")
installBuildTools(osType, "pkg-config")

# clone repo
print("📁 Cloning repo...")
cloneRepo("x264", "https://code.videolan.org/videolan/x264.git",
          "stable", "./dependencies_repo/x264")

cloneRepo("x265", "https://bitbucket.org/multicoreware/x265_git.git",
          "3.5", "./dependencies_repo/x265")

cloneRepo("opus", "https://gitlab.xiph.org/xiph/opus.git",
          "master", "./dependencies_repo/opus")

cloneRepo("libvpx", "https://chromium.googlesource.com/webm/libvpx",
          "v1.11.0", "./dependencies_repo/libvpx")

cloneRepo("ffmpeg", "https://git.ffmpeg.org/ffmpeg.git",
          "n5.0", "./dependencies_repo/ffmpeg")

# build dependencies
print("📦 Building...")


buildX264(osType, "./dependencies_repo/x264", "./dependencies_build/x264")
buildX265(osType, "./dependencies_repo/x265", "./dependencies_build/x265")
# buildOpus("./dependencies_repo/opus", "./dependencies_build/opus")
# buildLibvpx("./dependencies_repo/libvpx", "./dependencies_build/libvpx")
# buildFFmpeg(osType, "./dependencies_repo/ffmpeg",
#             "./dependencies_build/ffmpeg")

print("✅ All dependencies has build successfully.")
