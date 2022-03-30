import platform;
from enum import Enum
class OSType(Enum):
    Windows = 1,
    macOS = 2,
    Linux = 3

def getOSType():
    osType = platform.system()
    if osType == "Windows":
        return OSType.Windows
    elif osType == "macOS":
        return OSType.macOS
    else:
        return OSType.Linux


# Build Process

osType = getOSType()

