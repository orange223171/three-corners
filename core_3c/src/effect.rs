//! Effect definitions

/// An effect object
pub enum EffectObject {
    /// Represents a [Triangle](crate::board::Triangle) like its coordinates on the board
    Triangle(usize, usize),
    /// Represents a [Synergy](crate::synergy::Synergy) like its id on the board
    Synergy(u32),
}

/// An effect
pub struct Effect {
    /// A source of the effect
    pub source: EffectObject,
    /// A destination of the effect
    pub destination: EffectObject,

    /// A build price effect
    pub build_price_effect: i16,
    /// A destroy price effect
    pub destroy_price_effect: i16,
    /// A grab price effect
    pub grab_price_effect: i16,

    /// An economic profit effect
    pub economic_profit_effect: i16,
    /// A politic profit effect
    pub politic_profit_effect: i16,
    /// An authority profit effect
    pub authority_profit_effect: i16,

    /// A grab number of economic effect
    pub economic_grab_n_effect: i16,
    /// A grab number of politic effect
    pub politic_grab_n_effect: i16,
    /// A grab number of authority effect
    pub authority_grab_n_effect: i16,
}
