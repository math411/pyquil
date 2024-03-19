import pytest

from pyquil import Program


@pytest.fixture
def mock_calibration_program() -> Program:
    with open("test/bench/fixtures/ankaa-9q-1-calibrations.quil", "r") as file:
        return Program(file.read())


@pytest.fixture
def over_9000_line_program() -> Program:
    with open("test/bench/fixtures/over-9000.quil", "r") as file:
        return Program(file.read())


class TestInstructionIteration:
    def iterate(self, program: Program):
        for instruction in program:
            continue

    def test_calibration_program(self, benchmark, mock_calibration_program: Program):
        benchmark(self.iterate, mock_calibration_program)

    def test_large_program(self, benchmark, over_9000_line_program: Program):
        benchmark(self.iterate, over_9000_line_program)
