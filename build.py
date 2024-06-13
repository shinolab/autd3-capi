#!/usr/bin/env python3

import argparse
import contextlib
import glob
import os
import platform
import re
import shutil
import subprocess
import sys
from typing import Optional


def err(msg: str):
    print("\033[91mERR \033[0m: " + msg)


def warn(msg: str):
    print("\033[93mWARN\033[0m: " + msg)


def info(msg: str):
    print("\033[92mINFO\033[0m: " + msg)


def rm_f(path):
    try:
        os.remove(path)
    except FileNotFoundError:
        pass


def glob_norm(path, recursive):
    return list(
        map(lambda p: os.path.normpath(p), glob.glob(path, recursive=recursive))
    )


def rm_glob_f(path, exclude=None, recursive=True):
    if exclude is not None:
        for f in list(
            set(glob_norm(path, recursive=recursive))
            - set(glob_norm(exclude, recursive=recursive))
        ):
            rm_f(f)
    else:
        for f in glob.glob(path, recursive=recursive):
            rm_f(f)


@contextlib.contextmanager
def working_dir(path):
    cwd = os.getcwd()
    os.chdir(path)
    try:
        yield
    finally:
        os.chdir(cwd)


def env_exists(value):
    return value in os.environ and os.environ[value] != ""


class Config:
    _platform: str
    _all: bool
    release: bool
    target: Optional[str]
    no_examples: bool

    shaderc: bool
    cuda: bool

    def __init__(self, args):
        self._platform = platform.system()

        if not self.is_windows() and not self.is_macos() and not self.is_linux():
            err(f'platform "{platform.system()}" is not supported.')
            sys.exit(-1)

        self._all = hasattr(args, "all") and args.all
        self.release = hasattr(args, "release") and args.release
        self.no_examples = hasattr(args, "no_examples") and args.no_examples

        self.cuda = False if self.is_macos() else self.is_cuda_available()
        self.shaderc = self.is_shaderc_available()

        if hasattr(args, "arch") and args.arch is not None:
            if self.is_linux():
                self.shaderc = False
                self.cuda = False
                match args.arch:
                    case "arm32":
                        self.target = "armv7-unknown-linux-gnueabihf"
                    case "aarch64":
                        self.target = "aarch64-unknown-linux-gnu"
                    case _:
                        err(f'arch "{args.arch}" is not supported.')
                        sys.exit(-1)
            elif self.is_windows():
                self.shaderc = False
                self.cuda = False
                match args.arch:
                    case "aarch64":
                        self.target = "aarch64-pc-windows-msvc"
                    case _:
                        err(f'arch "{args.arch}" is not supported.')
                        sys.exit(-1)
        else:
            self.target = None

    def is_shaderc_available(self):
        shaderc_lib_name = (
            "shaderc_combined.lib" if self.is_windows() else "libshaderc_combined.a"
        )
        if env_exists("SHADERC_LIB_DIR"):
            if os.path.isfile(f"{os.environ['SHADERC_LIB_DIR']}/{shaderc_lib_name}"):
                return True
        if env_exists("VULKAN_SDK"):
            if os.path.isfile(f"{os.environ['VULKAN_SDK']}/lib/{shaderc_lib_name}"):
                return True
        if not self.is_windows():
            if os.path.isfile(f"/usr/local/lib/{shaderc_lib_name}"):
                return True
        if (
            shutil.which("git") is not None
            and shutil.which("cmake") is not None
            and shutil.which("python3") is not None
            and shutil.which("ninja")
        ):
            return True
        return False

    def is_cuda_available(self):
        return shutil.which("nvcc") is not None

    def cargo_command_base(self, subcommand):
        command = []
        if self.is_linux() and self.target:
            command.append("cross")
            command.append(subcommand)
        else:
            command.append("cargo")
            command.append(subcommand)
        if self.target:
            command.append("--target")
            command.append(self.target)
        if self.release:
            command.append("--release")
        return command

    def cargo_build_capi_command(self, extra_features=None):
        command = self.cargo_command_base("build")
        command.append("--all")
        command.append("--features")
        features = ""
        if extra_features is not None:
            features += extra_features
        command.append(features)
        if not self.cuda:
            command.append("--exclude")
            command.append("autd3capi-backend-cuda")
        if not self.shaderc:
            command.append("--exclude")
            command.append("autd3capi-link-visualizer")
        return command

    def cargo_clippy_capi_command(self, extra_features=None):
        command = self.cargo_build_capi_command(extra_features)
        command[1] = "clippy"
        command.append("--")
        command.append("-D")
        command.append("warnings")
        return command

    def is_windows(self):
        return self._platform == "Windows"

    def is_macos(self):
        return self._platform == "Darwin"

    def is_linux(self):
        return self._platform == "Linux"

    def setup_linker(self):
        if not self.is_linux() or self.target is None:
            return

        os.makedirs(".cargo", exist_ok=True)
        with open(".cargo/config", "w") as f:
            if self.target == "armv7-unknown-linux-gnueabihf":
                f.write("[target.armv7-unknown-linux-gnueabihf]\n")
                f.write('linker = "arm-linux-gnueabihf-gcc"\n')
            if self.target == "aarch64-unknown-linux-gnu":
                f.write("[target.aarch64-unknown-linux-gnu]\n")
                f.write('linker = "aarch64-linux-gnu-gcc"\n')


def copy_dll(config: Config, dst: str):
    if config.is_windows():
        target = ""
        if config.target is None:
            target = "target/release" if config.release else "target/debug"
        else:
            target = (
                f"target/{config.target}/release"
                if config.release
                else f"target/{config.target}/debug"
            )
        for dll in glob.glob(f"{target}/*.dll"):
            shutil.copy(dll, dst)
        for lib in glob.glob(f"{target}/*.dll.lib"):
            shutil.copy(lib, dst)
    elif config.is_macos():
        target = "target/release" if config.release else "target/debug"
        for lib in glob.glob(f"{target}/*.dylib"):
            shutil.copy(lib, dst)
    elif config.is_linux():
        target = ""
        if config.target is None:
            target = "target/release" if config.release else "target/debug"
        else:
            target = (
                f"target/{config.target}/release"
                if config.release
                else f"target/{config.target}/debug"
            )
        for lib in glob.glob(f"{target}/*.so"):
            shutil.copy(lib, dst)


def copy_lib(config: Config, dst: str):
    if config.is_windows():
        target = ""
        if config.target is None:
            target = "target/release" if config.release else "target/debug"
        else:
            target = (
                f"target/{config.target}/release"
                if config.release
                else f"target/{config.target}/debug"
            )
        for dll in glob.glob(f"{target}/*.lib"):
            shutil.copy(dll, dst)
        rm_glob_f(f"{dst}/*.dll.lib")
        if not config.release:
            for pdb in glob.glob(f"{target}/*.pdb"):
                shutil.copy(pdb, "lib")
    elif config.is_macos():
        target = "target/release" if config.release else "target/debug"
        for lib in glob.glob(f"{target}/*.a"):
            shutil.copy(lib, dst)
    elif config.is_linux():
        target = ""
        if config.target is None:
            target = "target/release" if config.release else "target/debug"
        else:
            target = (
                f"target/{config.target}/release"
                if config.release
                else f"target/{config.target}/debug"
            )
        for lib in glob.glob(f"{target}/*.a"):
            shutil.copy(lib, dst)


def capi_build(args):
    config = Config(args)

    with working_dir("."):
        config.setup_linker()
        subprocess.run(
            config.cargo_build_capi_command(args.features)
        ).check_returncode()

        os.makedirs("bin", exist_ok=True)
        copy_dll(config, "bin")
        os.makedirs("lib", exist_ok=True)
        copy_lib(config, "lib")


def capi_lint(args):
    config = Config(args)

    with working_dir("."):
        config.setup_linker()
        subprocess.run(
            config.cargo_clippy_capi_command(args.features)
        ).check_returncode()


def capi_clear(_):
    with working_dir("."):
        subprocess.run(["cargo", "clean"]).check_returncode()


def util_update_ver(args):
    version = args.version

    with working_dir("."):
        for toml in glob.glob("./**/*/Cargo.toml", recursive=True):
            if "tools" in toml:
                continue
            with open(toml, "r") as f:
                content = f.read()
                content = re.sub(
                    r'^version = "(.*?)"',
                    f'version = "{version}"',
                    content,
                    flags=re.MULTILINE,
                )
                content = re.sub(
                    r'^autd3(.*)version = "(.*?)"',
                    f'autd3\\1version = "{version}"',
                    content,
                    flags=re.MULTILINE,
                )
            with open(toml, "w") as f:
                f.write(content)

        with open("Cargo.toml", "r") as f:
            content = f.read()
            content = re.sub(
                r'^autd3(.*)version = "(.*?)"',
                f'autd3\\1version = "{version}"',
                content,
                flags=re.MULTILINE,
            )
        with open("Cargo.toml", "w") as f:
            f.write(content)

        with open("ThirdPartyNotice.txt", "r") as f:
            content = f.read()
            content = re.sub(
                r"^autd3(.*) (.*) \((.*)\)",
                f"autd3\\1 {version} (MIT)",
                content,
                flags=re.MULTILINE,
            )
            content = re.sub(
                r"^autd3-link-soem (.*)",
                f"autd3-link-soem {version}",
                content,
                flags=re.MULTILINE,
            )
            content = re.sub(
                r"^autd3-link-twincat (.*)",
                f"autd3-link-twincat {version}",
                content,
                flags=re.MULTILINE,
            )
        with open("ThirdPartyNotice.txt", "w") as f:
            f.write(content)

        subprocess.run(["cargo", "update"]).check_returncode()


def command_help(args):
    print(parser.parse_args([args.command, "--help"]))


if __name__ == "__main__":
    with working_dir(os.path.dirname(os.path.abspath(__file__))):
        parser = argparse.ArgumentParser(description="autd3capi library build script")
        subparsers = parser.add_subparsers()

        # build
        parser_build = subparsers.add_parser("build", help="see build -h`")
        parser_build.add_argument(
            "--release", action="store_true", help="release build"
        )
        parser_build.add_argument(
            "--arch", help="cross-compile for specific architecture"
        )
        parser_build.add_argument(
            "--features",
            help="features to build",
            default=None,
        )
        parser_build.set_defaults(handler=capi_build)

        # lint
        parser_lint = subparsers.add_parser("lint", help="see lint -h`")
        parser_lint.add_argument("--release", action="store_true", help="release build")
        parser_lint.add_argument(
            "--features",
            help="features to build",
            default=None,
        )
        parser_lint.set_defaults(handler=capi_lint)

        # clear
        parser_capi_clear = subparsers.add_parser("clear", help="see `clear -h`")
        parser_capi_clear.set_defaults(handler=capi_clear)

        # util
        parser_util = subparsers.add_parser("util", help="see `util -h`")
        subparsers_util = parser_util.add_subparsers()

        # util update version
        parser_util_upver = subparsers_util.add_parser(
            "upver", help="see `util upver -h`"
        )
        parser_util_upver.add_argument("version", help="version")
        parser_util_upver.set_defaults(handler=util_update_ver)

        # help
        parser_help = subparsers.add_parser("help", help="see `help -h`")
        parser_help.add_argument("command", help="command name which help is shown")
        parser_help.set_defaults(handler=command_help)

        args = parser.parse_args()
        if hasattr(args, "handler"):
            args.handler(args)
        else:
            parser.print_help()
