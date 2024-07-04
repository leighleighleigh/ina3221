/// Represents a register on the INA3221
#[allow(dead_code)]
#[derive(Debug)]
pub enum Register {
    Configuration = 0x00,
    ShuntVoltage1 = 0x01,
    BusVoltage1 = 0x02,
    ShuntVoltage2 = 0x03,
    BusVoltage2 = 0x04,
    ShuntVoltage3 = 0x05,
    BusVoltage3 = 0x06,
    CriticalAlertLimit1 = 0x07,
    WarningAlertLimit1 = 0x08,
    CriticalAlertLimit2 = 0x09,
    WarningAlertLimit2 = 0x0A,
    CriticalAlertLimit3 = 0x0B,
    WarningAlertLimit3 = 0x0C,
    ShuntVoltageSum = 0x0D,
    ShuntVoltageSumLimit = 0x0E,
    MaskEnable = 0x0F,
    PowerValidUpperLimit = 0x10,
    PowerValidLowerLimit = 0x11,
    ManufacturerId = 0xFE,
    DieId = 0xFF,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum AveragingMode{
    Samples1 = 0b000,
    Samples4 = 0b001,
    Samples16 = 0b010,
    Samples64 = 0b011,
    Samples128 = 0b100,
    Samples256 = 0b101,
    Samples512 = 0b110,
    Samples1024 = 0b111,
}