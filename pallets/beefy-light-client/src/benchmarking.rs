// This file is part of the SORA network and Polkaswap app.

// Copyright (c) 2020, 2021, Polka Biome Ltd. All rights reserved.
// SPDX-License-Identifier: BSD-4-Clause

// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:

// Redistributions of source code must retain the above copyright notice, this list
// of conditions and the following disclaimer.
// Redistributions in binary form must reproduce the above copyright notice, this
// list of conditions and the following disclaimer in the documentation and/or other
// materials provided with the distribution.
//
// All advertising materials mentioning features or use of this software must display
// the following acknowledgement: This product includes software developed by Polka Biome
// Ltd., SORA, and Polkaswap.
//
// Neither the name of the Polka Biome Ltd. nor the names of its contributors may be used
// to endorse or promote products derived from this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY Polka Biome Ltd. AS IS AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL Polka Biome Ltd. BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
// BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
// OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use super::*;

use crate::test_helpers::*;
#[allow(unused)]
use crate::Pallet as BeefyLightClient;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use hex_literal::hex;

benchmarks! {
    initialize {
        let root = hex!("36ee7c9903f810b22f7e6fca82c1c0cd6a151eca01f087683d92333094d94dc1");
        let curr_val_set = ValidatorSet {
            id: 0,
            len: 3,
            root: root.into(),
        };
        let next_val_set = ValidatorSet {
            id: 1,
            len: 3,
            root: root.into(),
        };
    }: _(RawOrigin::Root, SubNetworkId::Mainnet, 1, curr_val_set, next_val_set)
    verify {
    }

    submit_signature_commitment {
        let validators = 200;
        let tree_size = 5000;

        let fixture = load_fixture(validators, tree_size);
        let validator_set = fixture.validator_set.clone().into();
        let next_validator_set = fixture.next_validator_set.clone().into();

        BeefyLightClient::<T>::initialize(
            RawOrigin::Root.into(),
            SubNetworkId::Mainnet,
            0,
            validator_set,
            next_validator_set
        );

        let signed_commitment: beefy_primitives::SignedCommitment<
            u32,
            beefy_primitives::crypto::Signature,
        > = Decode::decode(&mut &fixture.commitment[..]).unwrap();
        let commitment = signed_commitment.commitment.clone();
        let validator_proof = validator_proof::<T>(&fixture, signed_commitment.signatures, validators);
        let leaf: BeefyMMRLeaf = Decode::decode(&mut &fixture.leaf[..]).unwrap();

    }: _(RawOrigin::Signed(alice::<T>()), SubNetworkId::Mainnet, commitment, validator_proof, leaf, fixture.leaf_proof.into())
    verify {
    }
}
