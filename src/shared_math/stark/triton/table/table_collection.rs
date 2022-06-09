use itertools::Itertools;

use super::aux_table::{AuxTable, ExtAuxTable};
use super::base_matrix::BaseMatrices;
use super::base_table::Table;
use super::instruction_table::{ExtInstructionTable, InstructionTable};
use super::io_table::{ExtIOTable, IOTable};
use super::jump_stack_table::{ExtJumpStackTable, JumpStackTable};
use super::op_stack_table::{ExtOpStackTable, OpStackTable};
use super::processor_table::{ExtProcessorTable, ProcessorTable};
use super::program_table::{ExtProgramTable, ProgramTable};
use super::ram_table::{ExtRAMTable, RAMTable};
use super::u32_op_table::{ExtU32OpTable, U32OpTable};
use crate::shared_math::b_field_element::BFieldElement;
use crate::shared_math::stark::triton::fri_domain::FriDomain;

type BWord = BFieldElement;

#[derive(Debug, Clone)]
pub struct BaseTableCollection {
    pub program_table: ProgramTable,
    pub instruction_table: InstructionTable,
    pub processor_table: ProcessorTable,
    pub input_table: IOTable,
    pub output_table: IOTable,
    pub op_stack_table: OpStackTable,
    pub ram_table: RAMTable,
    pub jump_stack_table: JumpStackTable,
    pub aux_table: AuxTable,
    pub u32_op_table: U32OpTable,
}

#[derive(Debug, Clone)]
pub struct ExtTableCollection {
    pub program_table: ExtProgramTable,
    pub instruction_table: ExtInstructionTable,
    pub processor_table: ExtProcessorTable,
    pub input_table: ExtIOTable,
    pub output_table: ExtIOTable,
    pub op_stack_table: ExtOpStackTable,
    pub ram_table: ExtRAMTable,
    pub jump_stack_table: ExtJumpStackTable,
    pub aux_table: ExtAuxTable,
    pub u32_op_table: ExtU32OpTable,
}

/// Convert vector-of-arrays to vector-of-vectors.
fn to_vec_vecs<T: Sized + Clone, const S: usize>(vector_of_arrays: &Vec<[T; S]>) -> Vec<Vec<T>> {
    vector_of_arrays
        .into_iter()
        .map(|arr| arr.to_vec())
        .collect_vec()
}

impl BaseTableCollection {
    pub fn empty() -> Self {
        // delete me when `from_base_matrices` works.
        todo!()
    }

    pub fn from_base_matrices(
        generator: BWord,
        order: usize,
        num_randomizers: usize,
        base_matrices: &BaseMatrices,
    ) -> Self {
        let program_table = ProgramTable::new_prover(
            generator,
            order,
            num_randomizers,
            to_vec_vecs(&base_matrices.program_matrix),
        );
        let processor_table = ProcessorTable::new_prover(
            generator,
            order,
            num_randomizers,
            to_vec_vecs(&base_matrices.processor_matrix),
        ); // ProcessorTable,
        let instruction_table = InstructionTable::new_prover(
            generator,
            order,
            num_randomizers,
            to_vec_vecs(&base_matrices.instruction_matrix),
        ); // InstructionTable
        let input_table = IOTable::new_prover(
            generator,
            order,
            num_randomizers,
            to_vec_vecs(&base_matrices.input_matrix),
        ); // InputTable,
        let output_table = IOTable::new_prover(
            generator,
            order,
            num_randomizers,
            to_vec_vecs(&base_matrices.input_matrix),
        ); // InputTable,
        let op_stack_table = OpStackTable::new_prover(
            generator,
            order,
            num_randomizers,
            to_vec_vecs(&base_matrices.op_stack_matrix),
        ); // OpStackTable,
        let ram_table = RAMTable::new_prover(
            generator,
            order,
            num_randomizers,
            to_vec_vecs(&base_matrices.ram_matrix),
        ); // RAMTable,
        let jump_stack_table = JumpStackTable::new_prover(
            generator,
            order,
            num_randomizers,
            to_vec_vecs(&base_matrices.jump_stack_matrix),
        ); // JumpStackTable,
        let aux_table = AuxTable::new_prover(
            generator,
            order,
            num_randomizers,
            to_vec_vecs(&base_matrices.aux_matrix),
        ); // Previously HashCoprocessorTable,
        let u32_op_table = U32OpTable::new_prover(
            generator,
            order,
            num_randomizers,
            to_vec_vecs(&base_matrices.u32_op_matrix),
        ); // U32OpTable,

        BaseTableCollection {
            program_table,
            instruction_table,
            processor_table,
            input_table,
            output_table,
            op_stack_table,
            ram_table,
            jump_stack_table,
            aux_table,
            u32_op_table,
        }
    }

    pub fn max_degree(&self) -> u64 {
        self.into_iter()
            .map(|table| table.max_degree())
            .max()
            .unwrap_or(1) as u64
    }

    pub fn codewords(&self, fri_domain: &FriDomain<BWord>) -> Vec<Vec<BWord>> {
        self.into_iter()
            .map(|table| table.low_degree_extension(fri_domain))
            .concat()
    }
}

impl<'a> IntoIterator for &'a BaseTableCollection {
    type Item = &'a dyn Table<BWord>;

    type IntoIter = std::array::IntoIter<&'a dyn Table<BWord>, 10>;

    fn into_iter(self) -> Self::IntoIter {
        [
            &self.program_table as &'a dyn Table<BWord>,
            &self.instruction_table as &'a dyn Table<BWord>,
            &self.processor_table as &'a dyn Table<BWord>,
            &self.input_table as &'a dyn Table<BWord>,
            &self.output_table as &'a dyn Table<BWord>,
            &self.op_stack_table as &'a dyn Table<BWord>,
            &self.ram_table as &'a dyn Table<BWord>,
            &self.jump_stack_table as &'a dyn Table<BWord>,
            &self.aux_table as &'a dyn Table<BWord>,
            &self.u32_op_table as &'a dyn Table<BWord>,
        ]
        .into_iter()
    }
}