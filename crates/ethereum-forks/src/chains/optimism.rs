use crate::{ForkCondition, Hardfork};
use alloy_primitives::U256;

/// Optimism mainnet hardforks
pub const OP_MAINNET_HARDFORKS: [(Hardfork, ForkCondition); 21] = [
    (Hardfork::Frontier, ForkCondition::Block(0)),
    (Hardfork::Homestead, ForkCondition::Block(0)),
    (Hardfork::Tangerine, ForkCondition::Block(0)),
    (Hardfork::SpuriousDragon, ForkCondition::Block(0)),
    (Hardfork::Byzantium, ForkCondition::Block(0)),
    (Hardfork::Constantinople, ForkCondition::Block(0)),
    (Hardfork::Petersburg, ForkCondition::Block(0)),
    (Hardfork::Istanbul, ForkCondition::Block(0)),
    (Hardfork::MuirGlacier, ForkCondition::Block(0)),
    (Hardfork::Berlin, ForkCondition::Block(3950000)),
    (Hardfork::London, ForkCondition::Block(105235063)),
    (Hardfork::ArrowGlacier, ForkCondition::Block(105235063)),
    (Hardfork::GrayGlacier, ForkCondition::Block(105235063)),
    (
        Hardfork::Paris,
        ForkCondition::TTD { fork_block: Some(105235063), total_difficulty: U256::ZERO },
    ),
    (Hardfork::Bedrock, ForkCondition::Block(105235063)),
    (Hardfork::Regolith, ForkCondition::Timestamp(0)),
    (Hardfork::Shanghai, ForkCondition::Timestamp(1704992401)),
    (Hardfork::Canyon, ForkCondition::Timestamp(1704992401)),
    (Hardfork::Cancun, ForkCondition::Timestamp(1710374401)),
    (Hardfork::Ecotone, ForkCondition::Timestamp(1710374401)),
    (Hardfork::Fjord, ForkCondition::Timestamp(1720627201)),
];

/// Optimism Sepolia hardforks
pub const OP_SEPOLIA_HARDFORKS: [(Hardfork, ForkCondition); 21] = [
    (Hardfork::Frontier, ForkCondition::Block(0)),
    (Hardfork::Homestead, ForkCondition::Block(0)),
    (Hardfork::Tangerine, ForkCondition::Block(0)),
    (Hardfork::SpuriousDragon, ForkCondition::Block(0)),
    (Hardfork::Byzantium, ForkCondition::Block(0)),
    (Hardfork::Constantinople, ForkCondition::Block(0)),
    (Hardfork::Petersburg, ForkCondition::Block(0)),
    (Hardfork::Istanbul, ForkCondition::Block(0)),
    (Hardfork::MuirGlacier, ForkCondition::Block(0)),
    (Hardfork::Berlin, ForkCondition::Block(0)),
    (Hardfork::London, ForkCondition::Block(0)),
    (Hardfork::ArrowGlacier, ForkCondition::Block(0)),
    (Hardfork::GrayGlacier, ForkCondition::Block(0)),
    (Hardfork::Paris, ForkCondition::TTD { fork_block: Some(0), total_difficulty: U256::ZERO }),
    (Hardfork::Bedrock, ForkCondition::Block(0)),
    (Hardfork::Regolith, ForkCondition::Timestamp(0)),
    (Hardfork::Shanghai, ForkCondition::Timestamp(1699981200)),
    (Hardfork::Canyon, ForkCondition::Timestamp(1699981200)),
    (Hardfork::Cancun, ForkCondition::Timestamp(1708534800)),
    (Hardfork::Ecotone, ForkCondition::Timestamp(1708534800)),
    (Hardfork::Fjord, ForkCondition::Timestamp(1716998400)),
];

/// Base Sepolia hardforks
pub const BASE_SEPOLIA_HARDFORKS: [(Hardfork, ForkCondition); 21] = [
    (Hardfork::Frontier, ForkCondition::Block(0)),
    (Hardfork::Homestead, ForkCondition::Block(0)),
    (Hardfork::Tangerine, ForkCondition::Block(0)),
    (Hardfork::SpuriousDragon, ForkCondition::Block(0)),
    (Hardfork::Byzantium, ForkCondition::Block(0)),
    (Hardfork::Constantinople, ForkCondition::Block(0)),
    (Hardfork::Petersburg, ForkCondition::Block(0)),
    (Hardfork::Istanbul, ForkCondition::Block(0)),
    (Hardfork::MuirGlacier, ForkCondition::Block(0)),
    (Hardfork::Berlin, ForkCondition::Block(0)),
    (Hardfork::London, ForkCondition::Block(0)),
    (Hardfork::ArrowGlacier, ForkCondition::Block(0)),
    (Hardfork::GrayGlacier, ForkCondition::Block(0)),
    (Hardfork::Paris, ForkCondition::TTD { fork_block: Some(0), total_difficulty: U256::ZERO }),
    (Hardfork::Bedrock, ForkCondition::Block(0)),
    (Hardfork::Regolith, ForkCondition::Timestamp(0)),
    (Hardfork::Shanghai, ForkCondition::Timestamp(1699981200)),
    (Hardfork::Canyon, ForkCondition::Timestamp(1699981200)),
    (Hardfork::Cancun, ForkCondition::Timestamp(1708534800)),
    (Hardfork::Ecotone, ForkCondition::Timestamp(1708534800)),
    (Hardfork::Fjord, ForkCondition::Timestamp(1716998400)),
];

/// Base Mainnet hardforks
pub const BASE_MAINNET_HARDFORKS: [(Hardfork, ForkCondition); 21] = [
    (Hardfork::Frontier, ForkCondition::Block(0)),
    (Hardfork::Homestead, ForkCondition::Block(0)),
    (Hardfork::Tangerine, ForkCondition::Block(0)),
    (Hardfork::SpuriousDragon, ForkCondition::Block(0)),
    (Hardfork::Byzantium, ForkCondition::Block(0)),
    (Hardfork::Constantinople, ForkCondition::Block(0)),
    (Hardfork::Petersburg, ForkCondition::Block(0)),
    (Hardfork::Istanbul, ForkCondition::Block(0)),
    (Hardfork::MuirGlacier, ForkCondition::Block(0)),
    (Hardfork::Berlin, ForkCondition::Block(0)),
    (Hardfork::London, ForkCondition::Block(0)),
    (Hardfork::ArrowGlacier, ForkCondition::Block(0)),
    (Hardfork::GrayGlacier, ForkCondition::Block(0)),
    (Hardfork::Paris, ForkCondition::TTD { fork_block: Some(0), total_difficulty: U256::ZERO }),
    (Hardfork::Bedrock, ForkCondition::Block(0)),
    (Hardfork::Regolith, ForkCondition::Timestamp(0)),
    (Hardfork::Shanghai, ForkCondition::Timestamp(1704992401)),
    (Hardfork::Canyon, ForkCondition::Timestamp(1704992401)),
    (Hardfork::Cancun, ForkCondition::Timestamp(1710374401)),
    (Hardfork::Ecotone, ForkCondition::Timestamp(1710374401)),
    (Hardfork::Fjord, ForkCondition::Timestamp(1720627201)),
];
