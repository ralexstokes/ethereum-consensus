use crate::{bellatrix, capella, deneb};

#[derive(Debug)]
pub enum ExecutionEngine<
    const BYTES_PER_LOGS_BLOOM: usize,
    const MAX_EXTRA_DATA_BYTES: usize,
    const MAX_BYTES_PER_TRANSACTION: usize,
    const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
    const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
    B: bellatrix::ExecutionEngine<
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
    >,
    C: capella::ExecutionEngine<
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
    >,
    D: deneb::ExecutionEngine<
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
    >,
> {
    Bellatrix(B),
    Capella(C),
    Deneb(D),
}

impl<
        const BYTES_PER_LOGS_BLOOM: usize,
        const MAX_EXTRA_DATA_BYTES: usize,
        const MAX_BYTES_PER_TRANSACTION: usize,
        const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
        const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
        B: bellatrix::ExecutionEngine<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
        >,
        C: capella::ExecutionEngine<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
        D: deneb::ExecutionEngine<
            BYTES_PER_LOGS_BLOOM,
            MAX_EXTRA_DATA_BYTES,
            MAX_BYTES_PER_TRANSACTION,
            MAX_TRANSACTIONS_PER_PAYLOAD,
            MAX_WITHDRAWALS_PER_PAYLOAD,
        >,
    >
    ExecutionEngine<
        BYTES_PER_LOGS_BLOOM,
        MAX_EXTRA_DATA_BYTES,
        MAX_BYTES_PER_TRANSACTION,
        MAX_TRANSACTIONS_PER_PAYLOAD,
        MAX_WITHDRAWALS_PER_PAYLOAD,
        B,
        C,
        D,
    >
{
    pub fn bellatrix(&self) -> Option<&B> {
        match self {
            Self::Bellatrix(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn capella(&self) -> Option<&C> {
        match self {
            Self::Capella(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn deneb(&self) -> Option<&D> {
        match self {
            Self::Deneb(inner) => Some(inner),
            _ => None,
        }
    }
}

// impl<
//         const BYTES_PER_LOGS_BLOOM: usize,
//         const MAX_EXTRA_DATA_BYTES: usize,
//         const MAX_BYTES_PER_TRANSACTION: usize,
//         const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
//         const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
//         B: bellatrix::ExecutionEngine<
//             BYTES_PER_LOGS_BLOOM,
//             MAX_EXTRA_DATA_BYTES,
//             MAX_BYTES_PER_TRANSACTION,
//             MAX_TRANSACTIONS_PER_PAYLOAD,
//         >,
//         C: capella::ExecutionEngine<
//             BYTES_PER_LOGS_BLOOM,
//             MAX_EXTRA_DATA_BYTES,
//             MAX_BYTES_PER_TRANSACTION,
//             MAX_TRANSACTIONS_PER_PAYLOAD,
//             MAX_WITHDRAWALS_PER_PAYLOAD,
//         >,
//         D: deneb::ExecutionEngine<
//             BYTES_PER_LOGS_BLOOM,
//             MAX_EXTRA_DATA_BYTES,
//             MAX_BYTES_PER_TRANSACTION,
//             MAX_TRANSACTIONS_PER_PAYLOAD,
//             MAX_WITHDRAWALS_PER_PAYLOAD,
//         >,
//     > From<B>
//     for ExecutionEngine<
//         BYTES_PER_LOGS_BLOOM,
//         MAX_EXTRA_DATA_BYTES,
//         MAX_BYTES_PER_TRANSACTION,
//         MAX_TRANSACTIONS_PER_PAYLOAD,
//         MAX_WITHDRAWALS_PER_PAYLOAD,
//         B,
//         C,
//         D,
//     >
// {
//     fn from(value: B) -> Self {
//         Self::Bellatrix(value)
//     }
// }

// impl<
//         const BYTES_PER_LOGS_BLOOM: usize,
//         const MAX_EXTRA_DATA_BYTES: usize,
//         const MAX_BYTES_PER_TRANSACTION: usize,
//         const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
//         const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
//         B: bellatrix::ExecutionEngine<
//             BYTES_PER_LOGS_BLOOM,
//             MAX_EXTRA_DATA_BYTES,
//             MAX_BYTES_PER_TRANSACTION,
//             MAX_TRANSACTIONS_PER_PAYLOAD,
//         >,
//         C: capella::ExecutionEngine<
//             BYTES_PER_LOGS_BLOOM,
//             MAX_EXTRA_DATA_BYTES,
//             MAX_BYTES_PER_TRANSACTION,
//             MAX_TRANSACTIONS_PER_PAYLOAD,
//             MAX_WITHDRAWALS_PER_PAYLOAD,
//         >,
//         D: deneb::ExecutionEngine<
//             BYTES_PER_LOGS_BLOOM,
//             MAX_EXTRA_DATA_BYTES,
//             MAX_BYTES_PER_TRANSACTION,
//             MAX_TRANSACTIONS_PER_PAYLOAD,
//             MAX_WITHDRAWALS_PER_PAYLOAD,
//         >,
//     > From<C>
//     for ExecutionEngine<
//         BYTES_PER_LOGS_BLOOM,
//         MAX_EXTRA_DATA_BYTES,
//         MAX_BYTES_PER_TRANSACTION,
//         MAX_TRANSACTIONS_PER_PAYLOAD,
//         MAX_WITHDRAWALS_PER_PAYLOAD,
//         B,
//         C,
//         D,
//     >
// {
//     fn from(value: C) -> Self {
//         Self::Capella(value)
//     }
// }

// impl<
//         const BYTES_PER_LOGS_BLOOM: usize,
//         const MAX_EXTRA_DATA_BYTES: usize,
//         const MAX_BYTES_PER_TRANSACTION: usize,
//         const MAX_TRANSACTIONS_PER_PAYLOAD: usize,
//         const MAX_WITHDRAWALS_PER_PAYLOAD: usize,
//         B: bellatrix::ExecutionEngine<
//             BYTES_PER_LOGS_BLOOM,
//             MAX_EXTRA_DATA_BYTES,
//             MAX_BYTES_PER_TRANSACTION,
//             MAX_TRANSACTIONS_PER_PAYLOAD,
//         >,
//         C: capella::ExecutionEngine<
//             BYTES_PER_LOGS_BLOOM,
//             MAX_EXTRA_DATA_BYTES,
//             MAX_BYTES_PER_TRANSACTION,
//             MAX_TRANSACTIONS_PER_PAYLOAD,
//             MAX_WITHDRAWALS_PER_PAYLOAD,
//         >,
//         D: deneb::ExecutionEngine<
//             BYTES_PER_LOGS_BLOOM,
//             MAX_EXTRA_DATA_BYTES,
//             MAX_BYTES_PER_TRANSACTION,
//             MAX_TRANSACTIONS_PER_PAYLOAD,
//             MAX_WITHDRAWALS_PER_PAYLOAD,
//         >,
//     > From<D>
//     for ExecutionEngine<
//         BYTES_PER_LOGS_BLOOM,
//         MAX_EXTRA_DATA_BYTES,
//         MAX_BYTES_PER_TRANSACTION,
//         MAX_TRANSACTIONS_PER_PAYLOAD,
//         MAX_WITHDRAWALS_PER_PAYLOAD,
//         B,
//         C,
//         D,
//     >
// {
//     fn from(value: D) -> Self {
//         Self::Deneb(value)
//     }
// }
