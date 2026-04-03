pub enum EffectObject {
    Triangle(usize, usize),
    Synergy(u32),
}

pub struct Effect {
    /// Источник эффекта
    pub source: EffectObject,
    /// Цель эффекта
    pub destination: EffectObject,

    /// Эффект на цену строительства
    pub build_price_effect: i16,
    /// Эффект на цену уничтожения
    pub destroy_price_effect: i16,
    /// Эффект на цену захвата ресурса
    pub grab_price_effect: i16,

    /// Эффект на выработку экономического ресурса
    pub economic_profit_effect: i16,
    /// Эффект на выработку политического ресурса
    pub politic_profit_effect: i16,
    /// Эффект на выработку влияния
    pub authority_profit_effect: i16,

    /// Эффект на количество захватываемого экономического ресурса
    pub economic_grab_n_effect: i16,
    /// Эффект на количество захватываемого политического ресурса
    pub politic_grab_n_effect: i16,
    /// Эффект на количество захватываемого влияния
    pub authority_grab_n_effect: i16,
}
