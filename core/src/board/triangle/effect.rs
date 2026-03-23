pub struct TriangleEffect {
    /// x координата треугольника
    x: u32,
    /// y координата треугольника
    y: u32,

    /// Эффект на цену строительства
    build_price_effect: i16,
    /// Эффект на цену уничтожения
    destroy_price_effect: i16,
    /// Эффект на цену захвата ресурса
    grab_price_effect: i16,

    /// Эффект на выработку экономического ресурса
    economic_profit_effect: i16,
    /// Эффект на выработку политического ресурса
    politic_profit_effect: i16,
    /// Эффект на выработку влияния
    authority_profit_effect: i16,

    /// Эффект на количество захватываемого экономического ресурса
    economic_grab_n_effect: i16,
    /// Эффект на количество захватываемого политического ресурса
    politic_grab_n_effect: i16,
    /// Эффект на количество захватываемого влияния
    authority_grab_n_effect: i16,
}
