// BP Core Library implementing LNP/BP specifications & standards related to
// bitcoin protocol
//
// Written in 2020-2022 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the Apache 2.0 License
// along with this software.
// If not, see <https://opensource.org/licenses/Apache-2.0>.

//! TxOut seals which are blinded with additional entropy.

// TODO: Re-implement when new single_use_seal API wii be done
// mod imp;
mod seal;

// pub use imp::{TxResolve, TxoutSeal, Witness};
pub use seal::{ConcealedSeal, ParseError, RevealedSeal};
