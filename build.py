#!/usr/bin/env python3

"""
File: build.py
Project: autd3
Created Date: 16/10/2023
Author: Shun Suzuki
-----
Last Modified: 19/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

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


@contextlib.contextmanager
def working_dir(path):
    cwd = os.getcwd()
    os.chdir(path)
    try:
        yield
    finally:
        os.chdir(cwd)


class Config:
    _platform: str
    _all: bool
    release: bool
    shaderc: bool
    target: Optional[str]
    universal: bool
    no_examples: bool
    cuda: bool

    def __init__(self, args):
        self._platform = platform.system()

        if not self.is_windows() and not self.is_macos() and not self.is_linux():
            err(f'platform "{platform.system()}" is not supported.')
            sys.exit(-1)

        self._all = hasattr(args, "all") and args.all
        self.release = hasattr(args, "release") and args.release
        self.universal = hasattr(args, "universal") and args.universal
        self.no_examples = hasattr(args, "no_examples") and args.no_examples

        self.cuda = False if self.is_macos() else True

        if self.is_linux() and hasattr(args, "arch") and args.arch is not None:
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
        else:
            self.target = None

    def cargo_command_base(self, subcommand):
        command = []
        if self.target is None:
            command.append("cargo")
            command.append(subcommand)
        else:
            command.append("cross")
            command.append(subcommand)
            command.append("--target")
            command.append(self.target)
        if self.release:
            command.append("--release")
        return command

    def cargo_build_capi_command(self, features=None):
        command = self.cargo_command_base("build")
        command.append("--all")
        if features is not None:
            command.append("--features")
            command.append(features)
        if not self.cuda:
            command.append("--exclude")
            command.append("autd3capi-backend-cuda")

        if self.is_macos() and self.universal:
            command_aarch64 = command.copy()
            command_aarch64.append("--target=aarch64-apple-darwin")
            command_x86 = command.copy()
            command_x86.append("--target=x86_64-apple-darwin")
            return [command_aarch64, command_x86]
        else:
            return [command]

    def cargo_clippy_capi_command(self):
        command = self.cargo_build_capi_command()[0]
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
        target = "target/release" if config.release else "target/debug"
        for dll in glob.glob(f"{target}/*.dll"):
            shutil.copy(dll, dst)
    elif config.is_macos():
        if config.universal:
            target = (
                "target/x86_64-apple-darwin/release"
                if config.release
                else "target/x86_64-apple-darwin/debug"
            )
            target_aarch64 = (
                "target/aarch64-apple-darwin/release"
                if config.release
                else "target/aarch64-apple-darwin/debug"
            )
            for x64_lib in glob.glob(f"{target}/*.dylib"):
                base_name = os.path.basename(x64_lib)
                subprocess.run(
                    [
                        "lipo",
                        "-create",
                        x64_lib,
                        f"./{target_aarch64}/{base_name}",
                        "-output",
                        f"./{dst}/{base_name}",
                    ]
                ).check_returncode()
        else:
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


def capi_build(args):
    config = Config(args)

    with working_dir("."):
        config.setup_linker()
        for command in config.cargo_build_capi_command(args.features):
            subprocess.run(command).check_returncode()

        os.makedirs("lib", exist_ok=True)
        os.makedirs("bin", exist_ok=True)
        copy_dll(config, "bin")
        if config.is_windows():
            target = "target/release" if config.release else "target/debug"
            for lib in glob.glob(f"{target}/*.dll.lib"):
                shutil.copy(lib, "lib")
            if not config.release:
                for pdb in glob.glob(f"{target}/*.pdb"):
                    shutil.copy(pdb, "lib")


def capi_lint(args):
    config = Config(args)

    with working_dir("."):
        config.setup_linker()
        subprocess.run(config.cargo_clippy_capi_command()).check_returncode()


def capi_clear(_):
    with working_dir("."):
        subprocess.run(["cargo", "clean"]).check_returncode()


def util_update_ver(args):
    version = args.version

    with working_dir("."):
        for toml in glob.glob("./**/*/Cargo.toml", recursive=True):
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
            "--arch", help="cross-compile for specific architecture (for Linux)"
        )
        parser_build.add_argument(
            "--universal", action="store_true", help="build universal binary"
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
