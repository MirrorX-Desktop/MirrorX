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
            "--disable-cli",
            "--extra-cflags=-DNO_PREFIX"
            ]

    env = os.environ.copy()
    if osType == OSType.MSYS:
        oldEnv = env["CC"]
        env["CC"] = f"cl:{oldEnv}"

    try:
        subprocess.call(args, cwd=sourceDir, env=env, shell=True)
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
        print(f"✔️  [x265] compile product exists. Skipping build.")
        return

    args = ["cmake",
            "./source",
            f"-DCMAKE_INSTALL_PREFIX={outputPath.resolve()}",
            "-DENABLE_SHARED=OFF",
            "-DENABLE_CLI=OFF",
            "-DENABLE_PIC=ON",
            ]

    try:
        subprocess.call(args, cwd=sourceDir)

        if osType == OSType.MSYS:
            args.append("-DSTATIC_LINK_CRT=ON")

            subprocess.call(
                ["cmake", "--build", ".", "--config", "Release"], cwd=sourceDir)
            subprocess.call(
                ["cmake", "--build", ".", "--target", "install", "--config", "Release"], cwd=sourceDir)
        else:
            subprocess.call(args=["make", f"-j{getCPUCores()}"], cwd=sourceDir)
            subprocess.call(args=["make", "install"], cwd=sourceDir)
            subprocess.call(args=["make", "clean"], cwd=sourceDir)

        print("✔️ [x265] build completed.")
    except subprocess.CalledProcessError:
        print("❌ [x265] build failed")
        exit()


def buildOpus(osType: OSType, sourceDir: str, outputDir: str):
    print("💡 Building [opus]...")
    inputPath = Path(sourceDir)
    if not inputPath.exists():
        print("❌ [opus] source dir does not exist.")
        exit()

    outputPath = Path(outputDir)
    print(f"👉 [opus] output dir: {outputPath.resolve()}")
    if outputPath.exists() and os.listdir(outputPath.resolve()).__sizeof__() > 0:
        print(f"✔️  [opus] compile product exists. Skipping build.")
        return

    try:
        if osType == OSType.MSYS:
            args = ["cmake",
                    ".",
                    f"-DCMAKE_INSTALL_PREFIX={outputPath.resolve()}",
                    "-DOPUS_STACK_PROTECTOR=OFF",
                    "-DCMAKE_BUILD_TYPE=Release",
                    ]

            subprocess.call(args, cwd=sourceDir)
            subprocess.call(
                ["cmake", "--build", ".", "--config", "Release"], cwd=sourceDir)
            subprocess.call(
                ["cmake", "--build", ".", "--target", "install", "--config", "Release"], cwd=sourceDir)
        else:
            subprocess.call("./autogen.sh", cwd=sourceDir, shell=True)
            subprocess.call(["./configure", f"--prefix={outputPath.resolve()}",
                             "--enable-static",
                             "--disable-shared",
                             "--disable-doc",
                             "--disable-extra-programs"], cwd=sourceDir, shell=True)
            subprocess.call(args=["make", f"-j{getCPUCores()}"], cwd=sourceDir)
            subprocess.call(args=["make", "install"], cwd=sourceDir)
            subprocess.call(args=["make", "clean"], cwd=sourceDir)

        print("✔️ [opus] build completed.")
    except subprocess.CalledProcessError:
        print("❌ [opus] build failed")
        exit()


def buildLibvpx(osType: OSType, sourceDir: str, outputDir: str):
    print("💡 Building [libvpx]...")
    inputPath = Path(sourceDir)
    if not inputPath.exists():
        print("❌ [libvpx] source dir does not exist.")
        exit()

    outputPath = Path(outputDir)
    print(f"👉 [libvpx] output dir: {outputPath.resolve()}")
    if outputPath.exists() and os.listdir(outputPath.resolve()).__sizeof__() > 0:
        print(f"✔️  [libvpx] compile product exists. Skipping build.")
        return

    args = ["./configure",
            f"--prefix=\"{outputPath.resolve()}\"",
            "--enable-vp9",
            "--enable-pic",
            "--enable-better-hw-compatibility",
            "--enable-realtime-only",
            "--disable-vp8",
            "--disable-examples",
            "--disable-docs",
            "--disable-tools",
            "--disable-unit-tests",
            "--disable-webm-io",
            "--disable-libyuv",
            ]

    try:
        if osType == OSType.MSYS:
            args.append("--target=x86_64-win64-vs17")
            args.append("--enable-static-msvcrt")

        subprocess.call(args, cwd=sourceDir)
        subprocess.call(args=["make", f"-j{getCPUCores()}"], cwd=sourceDir)
        subprocess.call(args=["make", "install"], cwd=sourceDir)
        subprocess.call(args=["make", "clean"], cwd=sourceDir)
        print("✔️\t[libvpx] build completed.")
    except subprocess.CalledProcessError:
        print("❌\t[libvpx] build failed")
        exit()


def buildNVHeaders(sourceDir: str):
    print("💡 Building [nv-headers]...")
    inputPath = Path(sourceDir)
    if not inputPath.exists():
        print("❌ [nv-headers] Source dir does not exist.")
        exit()

    # outputPath = Path(outputDir)
    # print(f"👉 [nv-headers] Output dir: {outputPath.resolve()}")
    # if outputPath.exists() and os.listdir(outputPath.resolve()).__sizeof__() > 0:
    #     print(f"✔️  [nv-headers] Compile product exists. Skipping build.")
    #     return

    try:
        subprocess.call(args=["make", f"-j{getCPUCores()}"], cwd=sourceDir)
        subprocess.call(args=["make", "install"], cwd=sourceDir)

        print("✔️  [nv-headers] build completed.")
    except subprocess.CalledProcessError:
        print("❌ [nv-headers] build failed.")
        exit()


def buildAMF(sourceDir: str):
    print("💡 Building [amf]...")
    inputPath = Path(sourceDir)
    if not inputPath.exists():
        print("❌ [amf] Source dir does not exist.")
        exit()

    # outputPath = Path(outputDir)
    # print(f"👉 [nv-headers] Output dir: {outputPath.resolve()}")
    # if outputPath.exists() and os.listdir(outputPath.resolve()).__sizeof__() > 0:
    #     print(f"✔️  [nv-headers] Compile product exists. Skipping build.")
    #     return

    try:
        srcIncludePath = Path(sourceDir).joinpath("amf").joinpath(
            "public").joinpath("include").joinpath("*")
        dstIncludePath = Path("/usr/local/include/AMF")
        subprocess.call(
            f"cp -r {srcIncludePath.resolve()} {dstIncludePath.resolve()}", cwd=sourceDir, shell=True)

        print("✔️  [amf] build completed.")
    except subprocess.CalledProcessError:
        print("❌ [amf] build failed.")
        exit()


def buildQSV(osType: OSType, sourceDir: str, outputDir: str):
    print("💡 Building [qsv]...")
    inputPath = Path(sourceDir)
    if not inputPath.exists():
        print("❌ [qsv] source dir does not exist.")
        exit()

    outputPath = Path(outputDir)
    print(f"👉 [qsv] output dir: {outputPath.resolve()}")
    if outputPath.exists() and os.listdir(outputPath.resolve()).__sizeof__() > 0:
        print(f"✔️  [qsv] compile product exists. Skipping build.")
        return

    try:
        if osType == OSType.MSYS:
            dispatchPath = Path(sourceDir).joinpath("SMP")
            projectPath = dispatchPath.joinpath("libmfx.sln")

            subprocess.call(
                f"devenv.exe {projectPath.resolve()} -upgrade", cwd=sourceDir, shell=True)

            subprocess.call(
                f"MSBuild.exe {projectPath.resolve()} -p:RuntimeLibrary=MultiThreaded -p:Platform=x64 -p:Configuration=Release -p:OutDir={outputPath.resolve()}/", cwd=sourceDir, shell=True)

        # else:
            # subprocess.call("./autogen.sh", cwd=sourceDir, shell=True)
            # subprocess.call(["./configure", f"--prefix={outputPath.resolve()}",
            #                  "--enable-static",
            #                  "--disable-shared",
            #                  "--disable-doc",
            #                  "--disable-extra-programs"], cwd=sourceDir, shell=True)
            # subprocess.call(args=["make", f"-j{getCPUCores()}"], cwd=sourceDir)
            # subprocess.call(args=["make", "install"], cwd=sourceDir)
            # subprocess.call(args=["make", "clean"], cwd=sourceDir)

        print("✔️ [qsv] build completed.")
    except subprocess.CalledProcessError:
        print("❌ [qsv] build failed")
        exit()


def buildFFmpeg(osType: OSType, sourceDir: str, outputDir: str):
    print("💡 Building [ffmpeg]...")
    inputPath = Path(sourceDir)
    if not inputPath.exists():
        print("❌ [ffmpeg] source dir does not exist.")
        exit()

    outputPath = Path(outputDir)
    print(f"👉 [ffmpeg] output dir: {outputPath.resolve()}")
    if outputPath.exists() and os.listdir(outputPath.resolve()).__sizeof__() > 0:
        print(f"✔️ [ffmpeg] compile product exists. Skipping build.")
        return

    args = ["./configure", f"--prefix={outputPath.resolve()}",
            "--disable-all",
            "--disable-autodetect",
            "--target-os=win64",
            "--arch=x86_64",
            "--enable-pic",
            "--enable-hardcoded-tables",
            "--enable-gpl",
            "--enable-nonfree",
            "--enable-version3",
            "--enable-avdevice",
            "--enable-avcodec",
            "--enable-avformat",
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

    env = os.environ.copy()

    if osType == OSType.macOS:
        args.append("--enable-videotoolbox")
        args.append("--enable-audiotoolbox")
        args.append("--enable-hwaccel=h264_videotoolbox")
        args.append("--enable-hwaccel=hevc_videotoolbox")
        args.append("--enable-hwaccel=vp9_videotoolbox")
    elif osType == OSType.MSYS:
        buildRootPath = outputPath.parent
        x264PCPath = buildRootPath.joinpath(
            "x264").joinpath("lib").joinpath("pkgconfig")
        x265PCPath = buildRootPath.joinpath(
            "x265").joinpath("lib").joinpath("pkgconfig")
        opusPCPath = buildRootPath.joinpath(
            "opus").joinpath("lib").joinpath("pkgconfig")
        qsvPCPath = buildRootPath.joinpath(
            "mfx_dispatch").joinpath("lib").joinpath("pkgconfig")

        args.append("--enable-cuvid")
        args.append("--enable-ffnvcodec")
        args.append("--enable-ffnvcodec")
        args.append("--enable-nvenc")
        args.append("--enable-nvdec")
        args.append("--enable-d3d11va")

        args.append("--enable-encoder=h264_amf")
        args.append("--enable-encoder=h264_nvenc")
        # args.append("--enable-encoder=h264_vaapi")
        args.append("--enable-encoder=h264_qsv")
        args.append("--enable-encoder=hevc_amf")
        args.append("--enable-encoder=hevc_nvenc")
        # args.append("--enable-encoder=hevc_vaapi")
        args.append("--enable-encoder=hevc_qsv")
        # args.append("--enable-encoder=vp9_vaapi")
        args.append("--enable-encoder=vp9_qsv")

        args.append("--enable-decoder=h264_cuvid")
        args.append("--enable-decoder=h264_qsv")
        args.append("--enable-decoder=hevc_cuvid")
        args.append("--enable-decoder=hevc_qsv")
        args.append("--enable-decoder=vp9_cuvid")
        args.append("--enable-decoder=vp9_qsv")

        args.append("--toolchain=msvc")
        args.append("--pkg-config-flags=--static")
        args.append(
            "--extra-cflags=-I/f/MirrorX/dependencies_build/libvpx/include")
        args.append(
            "--extra-ldflags=-libpath:/f/MirrorX/dependencies_build/libvpx/lib/x64")

        # AMF support
        args.append("--enable-amf")
        # todo copy amf/public/include/* to /usr/local/include/AMF/*
        args.append("--extra-cflags=-DAMF_CORE_STATIC")

        # QSV support
        args.append("--enable-libmfx")
        args.append(
            "--extra-cflags=-I/f/MirrorX/dependencies_build/mfx_dispatch/include")
        args.append(
            "--extra-ldflags=-libpath:/f/MirrorX/dependencies_build/mfx_dispatch/lib/x64 ole32.lib")

        oldEnv = env["PKG_CONFIG_PATH"]
        env["PKG_CONFIG_PATH"] = f"{x264PCPath.resolve()}:{x265PCPath.resolve()}:{opusPCPath.resolve()}:/usr/local/lib/pkgconfig:{oldEnv}"

    try:
        subprocess.call(args=args, cwd=sourceDir, env=env)
        subprocess.call(args=["make", f"-j{getCPUCores()}"], cwd=sourceDir)
        subprocess.call(args=["make", "install"], cwd=sourceDir)
        subprocess.call(args=["make", "clean"], cwd=sourceDir)

        print("✔️ [ffmpeg] build completed.")
    except subprocess.CalledProcessError:
        print("❌ [ffmpeg] build failed")
        exit()


# Build Process
# print platform
osType = getOSType()

if osType == OSType.Windows:
    print("❌ This script can't run in native Windows environment, please install MSYS2 and re-run it in MSYS2 environment!")
    exit()

print("📦 Building for: " + osType.name)

checkPackageManager(osType)

installBuildTools(osType, "git")
installBuildTools(osType, "make")
installBuildTools(osType, "cmake")
installBuildTools(osType, "automake")
installBuildTools(osType, "libtool")
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
          "v1.3.1", "./dependencies_repo/opus")

cloneRepo("libvpx", "https://github.com/webmproject/libvpx.git",
          "main", "./dependencies_repo/libvpx")

if osType == OSType.MSYS:
    cloneRepo("AMF", "https://github.com/GPUOpen-LibrariesAndSDKs/AMF.git",
              "v1.4.24", "./dependencies_repo/AMF")
    cloneRepo("mfx_dispatch", "https://github.com/ShiftMediaProject/mfx_dispatch.git",
              "1.35.r89", "./dependencies_repo/mfx_dispatch")
    cloneRepo("nv-codec-headers", "https://github.com/FFmpeg/nv-codec-headers.git",
              "n11.1.5.1", "./dependencies_repo/nv_codec_headers")

cloneRepo("ffmpeg", "https://git.ffmpeg.org/ffmpeg.git",
          "n5.0", "./dependencies_repo/ffmpeg")

# build dependencies
print("📦 Building...")

buildX264(osType, "./dependencies_repo/x264", "./dependencies_build/x264")
buildX265(osType, "./dependencies_repo/x265", "./dependencies_build/x265")
buildOpus(osType, "./dependencies_repo/opus", "./dependencies_build/opus")
buildLibvpx(osType, "./dependencies_repo/libvpx",
            "./dependencies_build/libvpx")

if osType == OSType.MSYS:
    buildNVHeaders("./dependencies_repo/nv_codec_headers")
    buildAMF("./dependencies_repo/AMF")
    buildQSV(osType, "./dependencies_repo/mfx_dispatch",
             "./dependencies_build/mfx_dispatch")

buildFFmpeg(osType, "./dependencies_repo/ffmpeg",
            "./dependencies_build/ffmpeg")

print("✅ All dependencies has build successfully.")
