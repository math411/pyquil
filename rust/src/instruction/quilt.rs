//! [TODO:description]

use pyo3::{
    exceptions::PyValueError, prelude::*, pyclass_init::PyObjectInit, type_object::PyTypeInfo,
};

use crate::{
    expression::{self, Expression, MemoryReference},
    impl_from_quil_rs,
    instruction::Instruction,
    primitive::QubitDesignator,
};

use super::GateModifier;

#[pyclass(extends=Instruction)]
#[derive(Clone, Debug)]
pub struct DefCalibration {}
impl_from_quil_rs!(
    DefCalibration,
    quil_rs::instruction::Calibration,
    CalibrationDefinition
);

#[derive(FromPyObject, Clone, Debug)]
pub enum Parameter {
    Expression(expression::Expression),
    MemoryReference(expression::MemoryReference),
    I64(i64),
    F64(f64),
    Complex(crate::Complex),
    U64(u64),
}

impl ToPyObject for Parameter {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self {
            Parameter::Expression(expression) => expression.clone().into_py(py),
            Parameter::MemoryReference(memory_reference) => memory_reference.clone().into_py(py),
            Parameter::I64(number) => number.to_object(py),
            Parameter::F64(number) => number.to_object(py),
            Parameter::Complex(number) => number.to_object(py),
            Parameter::U64(number) => number.to_object(py),
        }
    }
}

impl From<Parameter> for quil_rs::expression::Expression {
    fn from(value: Parameter) -> Self {
        match value {
            Parameter::Expression(expression) => expression.into(),
            Parameter::MemoryReference(memory_reference) => {
                quil_rs::expression::Expression::Address(memory_reference.into())
            }
            Parameter::I64(number) => quil_rs::expression::Expression::Number(crate::Complex {
                re: number as f64,
                im: 0.0,
            }),
            Parameter::U64(number) => quil_rs::expression::Expression::Number(crate::Complex {
                re: number as f64,
                im: 0.0,
            }),
            Parameter::F64(number) => quil_rs::expression::Expression::Number(number.into()),
            Parameter::Complex(number) => quil_rs::expression::Expression::Number(number),
        }
    }
}

impl From<quil_rs::expression::Expression> for Parameter {
    fn from(value: quil_rs::expression::Expression) -> Self {
        match value {
            quil_rs::expression::Expression::Number(number) => {
                if number.im == 0.0 {
                    if number.re as i64 as f64 == number.re {
                        Parameter::I64(number.re as i64)
                    } else if number.re as u64 as f64 == number.re {
                        Parameter::U64(number.re as u64)
                    } else {
                        Parameter::F64(number.re)
                    }
                } else {
                    Parameter::Complex(number)
                }
            }
            quil_rs::expression::Expression::Address(address) => {
                Parameter::MemoryReference(address.into())
            }
            quil_rs::expression::Expression::Prefix(_)
            | quil_rs::expression::Expression::Infix(_)
            | quil_rs::expression::Expression::FunctionCall(_)
            | quil_rs::expression::Expression::Variable(_)
            | quil_rs::expression::Expression::PiConstant => {
                Parameter::Expression(Expression::from(value))
            }
        }
    }
}

#[pymethods]
impl DefCalibration {
    #[new]
    fn new(
        name: &str,
        parameters: Vec<Parameter>,
        qubits: Vec<QubitDesignator>,
        instructions: Vec<Instruction>,
        modifiers: Option<Vec<GateModifier>>,
    ) -> PyResult<(Self, Instruction)> {
        let calibration = quil_rs::instruction::Calibration::new(
            name,
            parameters
                .into_iter()
                .map(quil_rs::expression::Expression::from)
                .collect(),
            qubits
                .into_iter()
                .map(quil_rs::instruction::Qubit::from)
                .collect(),
            instructions
                .into_iter()
                .map(quil_rs::instruction::Instruction::from)
                .collect(),
            modifiers
                .unwrap_or_default()
                .into_iter()
                .map(quil_rs::instruction::GateModifier::from)
                .collect(),
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok((
            Self {},
            Instruction::from_quil_rs(quil_rs::instruction::Instruction::CalibrationDefinition(
                calibration,
            )),
        ))
    }

    #[getter]
    fn parameters(self_: PyRef<'_, Self>, py: Python<'_>) -> Vec<PyObject> {
        let instruction = self_.into_super();
        if let quil_rs::instruction::Instruction::CalibrationDefinition(calibration) =
            &(*instruction).inner
        {
            calibration
                .parameters
                .iter()
                .cloned()
                .map(|p| Parameter::from(p).to_object(py))
                .collect()
        } else {
            unreachable!()
        }
    }

    fn set_parameters(self_: PyRefMut<'_, Self>, parameters: Vec<Parameter>) {
        let mut instruction = self_.into_super();
        if let quil_rs::instruction::Instruction::CalibrationDefinition(calibration) =
            &mut instruction.inner
        {
            calibration.parameters = parameters
                .into_iter()
                .map(quil_rs::expression::Expression::from)
                .collect()
        }
    }

    #[getter]
    fn qubits(self_: PyRef<'_, Self>, py: Python) -> Vec<PyObject> {
        let instruction = self_.into_super();
        if let quil_rs::instruction::Instruction::CalibrationDefinition(calibration) =
            &(*instruction).inner
        {
            calibration
                .qubits
                .iter()
                .cloned()
                .map(|p| QubitDesignator::from(p).to_object(py))
                .collect()
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_qubits(self_: PyRefMut<'_, Self>, qubits: Vec<QubitDesignator>) {
        let mut instruction = self_.into_super();
        if let quil_rs::instruction::Instruction::CalibrationDefinition(calibration) =
            &mut instruction.inner
        {
            calibration.qubits = qubits
                .into_iter()
                .map(quil_rs::instruction::Qubit::from)
                .collect()
        }
    }

    #[getter]
    fn instrs(self_: PyRef<'_, Self>, py: Python) -> Vec<PyObject> {
        let instruction = self_.into_super();
        if let quil_rs::instruction::Instruction::CalibrationDefinition(calibration) =
            &(*instruction).inner
        {
            calibration
                .instructions
                .iter()
                .cloned()
                .map(|p| Instruction::from(p).into_py(py))
                .collect()
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_instrs(self_: PyRefMut<'_, Self>, instructions: Vec<Instruction>) {
        let mut instruction = self_.into_super();
        if let quil_rs::instruction::Instruction::CalibrationDefinition(calibration) =
            &mut instruction.inner
        {
            calibration.instructions = instructions
                .into_iter()
                .map(quil_rs::instruction::Instruction::from)
                .collect()
        } else {
            unreachable!()
        }
    }
}

#[pyclass(extends=Instruction)]
#[derive(Clone, Debug)]
pub struct DefMeasureCalibration {}
impl_from_quil_rs!(
    DefMeasureCalibration,
    quil_rs::instruction::MeasureCalibrationDefinition,
    MeasureCalibrationDefinition
);

#[pymethods]
impl DefMeasureCalibration {
    #[new]
    #[pyo3(signature = (qubit, memory_reference, instrs))]
    fn __new__(
        qubit: Option<QubitDesignator>,
        memory_reference: MemoryReference,
        instrs: Vec<Instruction>,
    ) -> (Self, Instruction) {
        (
            Self {},
            Instruction {
                inner: quil_rs::instruction::Instruction::MeasureCalibrationDefinition(
                    quil_rs::instruction::MeasureCalibrationDefinition::new(
                        qubit.map(quil_rs::instruction::Qubit::from),
                        memory_reference.to_string(),
                        instrs
                            .into_iter()
                            .map(quil_rs::instruction::Instruction::from)
                            .collect(),
                    ),
                ),
            },
        )
    }

    #[getter]
    fn qubit(self_: PyRef<'_, Self>, py: Python) -> Option<PyObject> {
        let instruction = self_.into_super();
        if let quil_rs::instruction::Instruction::MeasureCalibrationDefinition(calibration) =
            &instruction.inner
        {
            calibration
                .qubit
                .clone()
                .map(|q| QubitDesignator::from(q).to_object(py))
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_qubit(self_: PyRefMut<'_, Self>, qubit: Option<QubitDesignator>) {
        let mut instruction = self_.into_super();
        if let quil_rs::instruction::Instruction::MeasureCalibrationDefinition(calibration) =
            &mut instruction.inner
        {
            calibration.qubit = qubit.map(quil_rs::instruction::Qubit::from)
        }
    }

    #[getter]
    fn instrs(self_: PyRef<'_, Self>, py: Python<'_>) -> Vec<PyObject> {
        let instruction = self_.into_super();
        if let quil_rs::instruction::Instruction::MeasureCalibrationDefinition(calibration) =
            &(*instruction).inner
        {
            calibration
                .instructions
                .iter()
                .cloned()
                .map(|p| Instruction::from(p).into_py(py))
                .collect()
        } else {
            unreachable!()
        }
    }

    #[setter]
    fn set_instrs(self_: PyRefMut<'_, Self>, instructions: Vec<Instruction>) {
        let mut instruction = self_.into_super();
        if let quil_rs::instruction::Instruction::MeasureCalibrationDefinition(calibration) =
            &mut instruction.inner
        {
            calibration.instructions = instructions
                .into_iter()
                .map(quil_rs::instruction::Instruction::from)
                .collect()
        } else {
            unreachable!()
        }
    }
}
