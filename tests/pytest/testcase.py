import dataclasses
import os
import subprocess
import unittest
from copy import deepcopy
from pathlib import Path
from typing import List, Optional, Dict


# These are environment variable names used by the application
CT_API_KEY = "CLOUDTRUTH_API_KEY"
CT_ENV = "CLOUDTRUTH_ENVIRONMENT"
CT_PROFILE = "CLOUDTRUTH_PROFILE"
CT_PROJ = "CLOUDTRUTH_PROJECT"
CT_URL = "CLOUDTRUTH_SERVER_URL"

DEFAULT_SERVER_URL = "https://api.cloudtruth.com/graphql"
DEFAULT_PROJ_NAME = "default"
DEFAULT_ENV_NAME = "default"

AUTO_DESCRIPTION = "Automated testing via `live_test`"

CT_TEST_LOG_COMMANDS = "CT_LIVE_TEST_LOG_COMMANDS"
CT_TEST_LOG_OUTPUT = "CT_LIVE_TEST_LOG_OUTPUT"


@dataclasses.dataclass
class Result:
    return_value: int = 0,
    stdout: List = dataclasses.field(default_factory=list),
    stderr: List = dataclasses.field(default_factory=list),

    @staticmethod
    def _first_line_contains(stream: List[str], value: str) -> Optional[str]:
        for line in stream:
            if value in line:
                return line
        return None

    def _contains_value(self, stream: List[str], value: str) -> bool:
        return self._first_line_contains(stream, value) is not None

    def _contains_both(self, stream: List[str], one: str, two: str) -> bool:
        line = self._first_line_contains(stream, one)
        if line:
            return two in line
        return False

    @staticmethod
    def _equals(stream: List[str], value: str) -> bool:
        total = "\n".join(stream)
        return total == value

    def out_contains_both(self, one: str, two: str) -> bool:
        return self._contains_both(self.stdout, one, two)

    def out_contains_value(self, one: str) -> bool:
        return self._contains_value(self.stdout, one)

    def out(self) -> str:
        return "\n".join(self.stdout)

    def err_contains_value(self, one: str) -> bool:
        return self._contains_value(self.stderr, one)

    def err(self) -> str:
        return "\n".join(self.stderr)


class TestCase(unittest.TestCase):
    """
    This extends the unittest.TestCase to add some basic functions
    """
    def __init__(self, *args, **kwargs):
        self._base_cmd = self.get_cli_base_cmd()
        self.log_commands = int(os.environ.get(CT_TEST_LOG_COMMANDS, "0"))
        self.log_output = int(os.environ.get(CT_TEST_LOG_OUTPUT, "0"))
        super().__init__(*args, **kwargs)

    def get_cli_base_cmd(self) -> str:
        """
        Finds where to get the executable image from.
        The result includes an extra space, and whatever other args may be necessary (e.g. api_key)
        """
        if os.environ.get("CI"):
            return "cloudtruth "

        # walk back up looking for top of projects, and goto `target/debug/cloudtruth`
        curr = Path(__file__)
        subdir = Path("target") / "debug"
        match = False
        while not match and curr:
            possible = curr.parent / subdir
            match = possible.exists()
            curr = curr.parent

        if not match:
            return "cloudtruth "

        for fname in ("cloudtruth", "cloudtruth.exe"):
            file = possible / fname
            if file.exists():
                return str(file) + " "

        # this is a little odd... no executable found
        return "cloudtruth "

    def get_cmd_env(self):
        return deepcopy(os.environ)

    def run_cli(self, env: Dict[str, str], cmd) -> Result:
        if self.log_commands:
            print(cmd)

        process = subprocess.run(
            cmd, env=env, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE
        )
        result = Result(
            return_value=process.returncode,
            stdout=process.stdout.decode("utf-8").split("\n"),
            stderr=process.stderr.decode("utf-8").split("\n"),
        )

        if self.log_output:
            if result.stdout:
                print("\n".join(result.stdout))
            if result.stderr:
                print("\n".join(result.stderr))

        return result

    def create_project(self, cmd_env, proj_name: str) -> None:
        result = self.run_cli(cmd_env,
                              self._base_cmd + f"proj set '{proj_name}' -d '{AUTO_DESCRIPTION}'")
        self.assertEqual(result.return_value, 0)

    def delete_project(self, cmd_env, proj_name: str) -> None:
        result = self.run_cli(cmd_env, self._base_cmd + f" proj delete '{proj_name}' --confirm")
        self.assertEqual(result.return_value, 0)

    def create_environment(self, cmd_env, env_name: str) -> None:
        result = self.run_cli(cmd_env,
                              self._base_cmd + f"env set '{env_name}' -d '{AUTO_DESCRIPTION}'")
        self.assertEqual(result.return_value, 0)

    def delete_environment(self, cmd_env, env_name: str) -> None:
        result = self.run_cli(cmd_env, self._base_cmd + f"env del '{env_name}' --confirm")
        self.assertEqual(result.return_value, 0)

    def set_param(self, cmd_env, proj: str, name: str, value: str, secret: bool = False):
        result = self.run_cli(cmd_env,
                              self._base_cmd + f"--project '{proj}' param set '{name}' " +
                              f"--value '{value}' --secret '{str(secret).lower()}'")
        self.assertEqual(result.return_value, 0)

    def delete_param(self, cmd_env, proj: str, name: str):
        result = self.run_cli(cmd_env, self._base_cmd + f"--project '{proj}' param delete '{name}'")
        self.assertEqual(result.return_value, 0)

