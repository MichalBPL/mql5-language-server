//! MQL5 built-in functions, enums, structs, constants, globals, keywords, and types.
//! Auto-generated from MQL5 stub files for language server support.

#[derive(Clone, Debug)]
pub struct BuiltinFunction {
    pub name: &'static str,
    pub signature: &'static str,
    pub doc: Option<&'static str>,
}

#[derive(Clone, Debug)]
pub struct BuiltinEnum {
    pub name: &'static str,
    pub values: &'static [&'static str],
}

#[derive(Clone, Debug)]
pub struct BuiltinStruct {
    pub name: &'static str,
    pub fields: &'static [(&'static str, &'static str)], // (field_name, field_type)
}

#[derive(Clone, Debug)]
pub struct BuiltinConstant {
    pub name: &'static str,
    pub value: &'static str,
    pub doc: Option<&'static str>,
}

// =============================================================================
// BUILTIN FUNCTIONS
// =============================================================================
pub static BUILTIN_FUNCTIONS: &[BuiltinFunction] = &[
    // === Account ===
    BuiltinFunction {
        name: "AccountInfoInteger",
        signature: "long AccountInfoInteger(ENUM_ACCOUNT_INFO_INTEGER property_id)",
        doc: Some("Returns the value of an integer property of the account."),
    },
    BuiltinFunction {
        name: "AccountInfoDouble",
        signature: "double AccountInfoDouble(ENUM_ACCOUNT_INFO_DOUBLE property_id)",
        doc: Some("Returns the value of a double property of the account."),
    },
    BuiltinFunction {
        name: "AccountInfoString",
        signature: "string AccountInfoString(ENUM_ACCOUNT_INFO_STRING property_id)",
        doc: Some("Returns the value of a string property of the account."),
    },
    // === Trading ===
    BuiltinFunction {
        name: "OrderSend",
        signature: "bool OrderSend(const MqlTradeRequest& request, MqlTradeResult& result)",
        doc: Some("Sends a trade request to the server. Returns true if the request was sent successfully."),
    },
    BuiltinFunction {
        name: "OrderCheck",
        signature: "bool OrderCheck(const MqlTradeRequest& request, MqlTradeCheckResult& result)",
        doc: Some("Checks if there are enough funds to execute the trade request. Does not send the order."),
    },
    BuiltinFunction {
        name: "OrderCalcMargin",
        signature: "double OrderCalcMargin(ENUM_ORDER_TYPE action, string symbol, double volume, double price)",
        doc: Some("Calculates the margin required for the specified order type."),
    },
    BuiltinFunction {
        name: "OrderCalcProfit",
        signature: "double OrderCalcProfit(ENUM_ORDER_TYPE action, string symbol, double volume, double price_open, double price_close)",
        doc: Some("Calculates the profit for the specified order parameters."),
    },
    BuiltinFunction {
        name: "OrderSelect",
        signature: "bool OrderSelect(ulong ticket)",
        doc: Some("Selects an order by ticket for further processing."),
    },
    BuiltinFunction {
        name: "OrdersTotal",
        signature: "int OrdersTotal()",
        doc: Some("Returns the number of current pending orders."),
    },
    BuiltinFunction {
        name: "OrderGetTicket",
        signature: "ulong OrderGetTicket(int index)",
        doc: Some("Returns ticket of a pending order by its index."),
    },
    BuiltinFunction {
        name: "OrderGetInteger",
        signature: "long OrderGetInteger(ENUM_ORDER_PROPERTY_INTEGER property_id)",
        doc: Some("Returns an integer property of the selected order."),
    },
    BuiltinFunction {
        name: "OrderGetDouble",
        signature: "double OrderGetDouble(ENUM_ORDER_PROPERTY_DOUBLE property_id)",
        doc: Some("Returns a double property of the selected order."),
    },
    BuiltinFunction {
        name: "OrderGetString",
        signature: "string OrderGetString(ENUM_ORDER_PROPERTY_STRING property_id)",
        doc: Some("Returns a string property of the selected order."),
    },
    BuiltinFunction {
        name: "PositionSelect",
        signature: "bool PositionSelect(string symbol)",
        doc: Some("Selects an open position by symbol for further processing."),
    },
    BuiltinFunction {
        name: "PositionSelectByTicket",
        signature: "bool PositionSelectByTicket(ulong ticket)",
        doc: Some("Selects an open position by ticket for further processing."),
    },
    BuiltinFunction {
        name: "PositionsTotal",
        signature: "int PositionsTotal()",
        doc: Some("Returns the number of open positions."),
    },
    BuiltinFunction {
        name: "PositionGetSymbol",
        signature: "string PositionGetSymbol(int index)",
        doc: Some("Returns the symbol of an open position by its index."),
    },
    BuiltinFunction {
        name: "PositionGetTicket",
        signature: "ulong PositionGetTicket(int index)",
        doc: Some("Returns the ticket of an open position by its index."),
    },
    BuiltinFunction {
        name: "PositionGetInteger",
        signature: "long PositionGetInteger(ENUM_POSITION_PROPERTY_INTEGER property_id)",
        doc: Some("Returns an integer property of the selected position."),
    },
    BuiltinFunction {
        name: "PositionGetDouble",
        signature: "double PositionGetDouble(ENUM_POSITION_PROPERTY_DOUBLE property_id)",
        doc: Some("Returns a double property of the selected position."),
    },
    BuiltinFunction {
        name: "PositionGetString",
        signature: "string PositionGetString(ENUM_POSITION_PROPERTY_STRING property_id)",
        doc: Some("Returns a string property of the selected position."),
    },
    BuiltinFunction {
        name: "HistorySelect",
        signature: "bool HistorySelect(datetime from_date, datetime to_date)",
        doc: Some("Requests the history of deals and orders for the specified period."),
    },
    BuiltinFunction {
        name: "HistorySelectByPosition",
        signature: "bool HistorySelectByPosition(long position_id)",
        doc: Some("Requests the history of deals and orders with the specified position identifier."),
    },
    BuiltinFunction {
        name: "HistoryOrderSelect",
        signature: "bool HistoryOrderSelect(ulong ticket)",
        doc: Some("Selects an order in the history by its ticket."),
    },
    BuiltinFunction {
        name: "HistoryOrdersTotal",
        signature: "int HistoryOrdersTotal()",
        doc: Some("Returns the number of orders in the history (after HistorySelect)."),
    },
    BuiltinFunction {
        name: "HistoryOrderGetTicket",
        signature: "ulong HistoryOrderGetTicket(int index)",
        doc: Some("Returns the ticket of a history order by index."),
    },
    BuiltinFunction {
        name: "HistoryOrderGetInteger",
        signature: "long HistoryOrderGetInteger(ulong ticket_number, ENUM_ORDER_PROPERTY_INTEGER property_id)",
        doc: Some("Returns an integer property of a history order."),
    },
    BuiltinFunction {
        name: "HistoryOrderGetDouble",
        signature: "double HistoryOrderGetDouble(ulong ticket_number, ENUM_ORDER_PROPERTY_DOUBLE property_id)",
        doc: Some("Returns a double property of a history order."),
    },
    BuiltinFunction {
        name: "HistoryOrderGetString",
        signature: "string HistoryOrderGetString(ulong ticket_number, ENUM_ORDER_PROPERTY_STRING property_id)",
        doc: Some("Returns a string property of a history order."),
    },
    BuiltinFunction {
        name: "HistoryDealSelect",
        signature: "bool HistoryDealSelect(ulong ticket)",
        doc: Some("Selects a deal in the history by its ticket."),
    },
    BuiltinFunction {
        name: "HistoryDealsTotal",
        signature: "int HistoryDealsTotal()",
        doc: Some("Returns the number of deals in the history (after HistorySelect)."),
    },
    BuiltinFunction {
        name: "HistoryDealGetTicket",
        signature: "ulong HistoryDealGetTicket(int index)",
        doc: Some("Returns the ticket of a history deal by index."),
    },
    BuiltinFunction {
        name: "HistoryDealGetInteger",
        signature: "long HistoryDealGetInteger(ulong ticket_number, ENUM_DEAL_PROPERTY_INTEGER property_id)",
        doc: Some("Returns an integer property of a history deal."),
    },
    BuiltinFunction {
        name: "HistoryDealGetDouble",
        signature: "double HistoryDealGetDouble(ulong ticket_number, ENUM_DEAL_PROPERTY_DOUBLE property_id)",
        doc: Some("Returns a double property of a history deal."),
    },
    BuiltinFunction {
        name: "HistoryDealGetString",
        signature: "string HistoryDealGetString(ulong ticket_number, ENUM_DEAL_PROPERTY_STRING property_id)",
        doc: Some("Returns a string property of a history deal."),
    },
    // === Technical Indicators ===
    BuiltinFunction {
        name: "iAC",
        signature: "int iAC(string symbol, ENUM_TIMEFRAMES period)",
        doc: Some("Creates Accelerator Oscillator indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iAD",
        signature: "int iAD(string symbol, ENUM_TIMEFRAMES period, ENUM_APPLIED_VOLUME applied_volume)",
        doc: Some("Creates Accumulation/Distribution indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iADX",
        signature: "int iADX(string symbol, ENUM_TIMEFRAMES period, int adx_period)",
        doc: Some("Creates Average Directional Index indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iADXWilder",
        signature: "int iADXWilder(string symbol, ENUM_TIMEFRAMES period, int adx_period)",
        doc: Some("Creates ADX by Welles Wilder indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iAlligator",
        signature: "int iAlligator(string symbol, ENUM_TIMEFRAMES period, int jaw_period, int jaw_shift, int teeth_period, int teeth_shift, int lips_period, int lips_shift, ENUM_MA_METHOD ma_method, ENUM_APPLIED_PRICE applied_price)",
        doc: Some("Creates Alligator indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iAMA",
        signature: "int iAMA(string symbol, ENUM_TIMEFRAMES period, int ama_period, int fast_ma_period, int slow_ma_period, int ama_shift, ENUM_APPLIED_PRICE applied_price)",
        doc: Some("Creates Adaptive Moving Average indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iAO",
        signature: "int iAO(string symbol, ENUM_TIMEFRAMES period)",
        doc: Some("Creates Awesome Oscillator indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iATR",
        signature: "int iATR(string symbol, ENUM_TIMEFRAMES period, int ma_period)",
        doc: Some("Creates Average True Range indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iBands",
        signature: "int iBands(string symbol, ENUM_TIMEFRAMES period, int bands_period, int bands_shift, double deviation, ENUM_APPLIED_PRICE applied_price)",
        doc: Some("Creates Bollinger Bands indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iBearsPower",
        signature: "int iBearsPower(string symbol, ENUM_TIMEFRAMES period, int ma_period)",
        doc: Some("Creates Bears Power indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iBullsPower",
        signature: "int iBullsPower(string symbol, ENUM_TIMEFRAMES period, int ma_period)",
        doc: Some("Creates Bulls Power indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iBWMFI",
        signature: "int iBWMFI(string symbol, ENUM_TIMEFRAMES period, ENUM_APPLIED_VOLUME applied_volume)",
        doc: Some("Creates Market Facilitation Index by Bill Williams and returns its handle."),
    },
    BuiltinFunction {
        name: "iCCI",
        signature: "int iCCI(string symbol, ENUM_TIMEFRAMES period, int ma_period, ENUM_APPLIED_PRICE applied_price)",
        doc: Some("Creates Commodity Channel Index indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iChaikin",
        signature: "int iChaikin(string symbol, ENUM_TIMEFRAMES period, int fast_ma_period, int slow_ma_period, ENUM_MA_METHOD ma_method, ENUM_APPLIED_VOLUME applied_volume)",
        doc: Some("Creates Chaikin Oscillator indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iCustom",
        signature: "int iCustom(string symbol, ENUM_TIMEFRAMES period, string name, ...)",
        doc: Some("Creates a custom indicator and returns its handle. Additional parameters are passed to the indicator."),
    },
    BuiltinFunction {
        name: "iDEMA",
        signature: "int iDEMA(string symbol, ENUM_TIMEFRAMES period, int ma_period, int ma_shift, ENUM_APPLIED_PRICE applied_price)",
        doc: Some("Creates Double Exponential Moving Average indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iDeMarker",
        signature: "int iDeMarker(string symbol, ENUM_TIMEFRAMES period, int ma_period)",
        doc: Some("Creates DeMarker indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iEnvelopes",
        signature: "int iEnvelopes(string symbol, ENUM_TIMEFRAMES period, int ma_period, int ma_shift, ENUM_MA_METHOD ma_method, ENUM_APPLIED_PRICE applied_price, double deviation)",
        doc: Some("Creates Envelopes indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iForce",
        signature: "int iForce(string symbol, ENUM_TIMEFRAMES period, int ma_period, ENUM_MA_METHOD ma_method, ENUM_APPLIED_VOLUME applied_volume)",
        doc: Some("Creates Force Index indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iFractals",
        signature: "int iFractals(string symbol, ENUM_TIMEFRAMES period)",
        doc: Some("Creates Fractals indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iFrAMA",
        signature: "int iFrAMA(string symbol, ENUM_TIMEFRAMES period, int ma_period, int ma_shift, ENUM_APPLIED_PRICE applied_price)",
        doc: Some("Creates Fractal Adaptive Moving Average indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iGator",
        signature: "int iGator(string symbol, ENUM_TIMEFRAMES period, int jaw_period, int jaw_shift, int teeth_period, int teeth_shift, int lips_period, int lips_shift, ENUM_MA_METHOD ma_method, ENUM_APPLIED_PRICE applied_price)",
        doc: Some("Creates Gator Oscillator indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iIchimoku",
        signature: "int iIchimoku(string symbol, ENUM_TIMEFRAMES period, int tenkan_sen, int kijun_sen, int senkou_span_b)",
        doc: Some("Creates Ichimoku Kinko Hyo indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iMA",
        signature: "int iMA(string symbol, ENUM_TIMEFRAMES period, int ma_period, int ma_shift, ENUM_MA_METHOD ma_method, ENUM_APPLIED_PRICE applied_price)",
        doc: Some("Creates Moving Average indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iMACD",
        signature: "int iMACD(string symbol, ENUM_TIMEFRAMES period, int fast_ema_period, int slow_ema_period, int signal_period, ENUM_APPLIED_PRICE applied_price)",
        doc: Some("Creates MACD indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iMFI",
        signature: "int iMFI(string symbol, ENUM_TIMEFRAMES period, int ma_period, ENUM_APPLIED_VOLUME applied_volume)",
        doc: Some("Creates Money Flow Index indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iMomentum",
        signature: "int iMomentum(string symbol, ENUM_TIMEFRAMES period, int mom_period, ENUM_APPLIED_PRICE applied_price)",
        doc: Some("Creates Momentum indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iOBV",
        signature: "int iOBV(string symbol, ENUM_TIMEFRAMES period, ENUM_APPLIED_VOLUME applied_volume)",
        doc: Some("Creates On Balance Volume indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iOsMA",
        signature: "int iOsMA(string symbol, ENUM_TIMEFRAMES period, int fast_ema_period, int slow_ema_period, int signal_period, ENUM_APPLIED_PRICE applied_price)",
        doc: Some("Creates OsMA (Moving Average of Oscillator) indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iRSI",
        signature: "int iRSI(string symbol, ENUM_TIMEFRAMES period, int ma_period, ENUM_APPLIED_PRICE applied_price)",
        doc: Some("Creates Relative Strength Index indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iRVI",
        signature: "int iRVI(string symbol, ENUM_TIMEFRAMES period, int ma_period)",
        doc: Some("Creates Relative Vigor Index indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iSAR",
        signature: "int iSAR(string symbol, ENUM_TIMEFRAMES period, double step, double maximum)",
        doc: Some("Creates Parabolic SAR indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iStdDev",
        signature: "int iStdDev(string symbol, ENUM_TIMEFRAMES period, int ma_period, int ma_shift, ENUM_MA_METHOD ma_method, ENUM_APPLIED_PRICE applied_price)",
        doc: Some("Creates Standard Deviation indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iStochastic",
        signature: "int iStochastic(string symbol, ENUM_TIMEFRAMES period, int Kperiod, int Dperiod, int slowing, ENUM_MA_METHOD ma_method, ENUM_STO_PRICE price_field)",
        doc: Some("Creates Stochastic Oscillator indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iTEMA",
        signature: "int iTEMA(string symbol, ENUM_TIMEFRAMES period, int ma_period, int ma_shift, ENUM_APPLIED_PRICE applied_price)",
        doc: Some("Creates Triple Exponential Moving Average indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iTriX",
        signature: "int iTriX(string symbol, ENUM_TIMEFRAMES period, int ma_period, ENUM_APPLIED_PRICE applied_price)",
        doc: Some("Creates Triple Exponential Moving Averages Oscillator and returns its handle."),
    },
    BuiltinFunction {
        name: "iVIDyA",
        signature: "int iVIDyA(string symbol, ENUM_TIMEFRAMES period, int cmo_period, int ema_period, int ma_shift, ENUM_APPLIED_PRICE applied_price)",
        doc: Some("Creates Variable Index Dynamic Average indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iVolumes",
        signature: "int iVolumes(string symbol, ENUM_TIMEFRAMES period, ENUM_APPLIED_VOLUME applied_volume)",
        doc: Some("Creates Volumes indicator and returns its handle."),
    },
    BuiltinFunction {
        name: "iWPR",
        signature: "int iWPR(string symbol, ENUM_TIMEFRAMES period, int calc_period)",
        doc: Some("Creates Williams' Percent Range indicator and returns its handle."),
    },
    // === Timeseries & Indicator Access ===
    BuiltinFunction {
        name: "CopyBuffer",
        signature: "int CopyBuffer(int indicator_handle, int buffer_num, int start_pos, int count, double& buffer[])",
        doc: Some("Copies indicator buffer data. Returns the number of copied elements or -1 on error."),
    },
    BuiltinFunction {
        name: "CopyRates",
        signature: "int CopyRates(string symbol, ENUM_TIMEFRAMES timeframe, int start_pos, int count, MqlRates& rates_array[])",
        doc: Some("Copies rates (OHLCV) data into an MqlRates array. Returns count or -1 on error."),
    },
    BuiltinFunction {
        name: "CopyTime",
        signature: "int CopyTime(string symbol, ENUM_TIMEFRAMES timeframe, int start_pos, int count, datetime& time_array[])",
        doc: Some("Copies bar open times. Returns count or -1 on error."),
    },
    BuiltinFunction {
        name: "CopyOpen",
        signature: "int CopyOpen(string symbol, ENUM_TIMEFRAMES timeframe, int start_pos, int count, double& open_array[])",
        doc: Some("Copies bar open prices. Returns count or -1 on error."),
    },
    BuiltinFunction {
        name: "CopyHigh",
        signature: "int CopyHigh(string symbol, ENUM_TIMEFRAMES timeframe, int start_pos, int count, double& high_array[])",
        doc: Some("Copies bar high prices. Returns count or -1 on error."),
    },
    BuiltinFunction {
        name: "CopyLow",
        signature: "int CopyLow(string symbol, ENUM_TIMEFRAMES timeframe, int start_pos, int count, double& low_array[])",
        doc: Some("Copies bar low prices. Returns count or -1 on error."),
    },
    BuiltinFunction {
        name: "CopyClose",
        signature: "int CopyClose(string symbol, ENUM_TIMEFRAMES timeframe, int start_pos, int count, double& close_array[])",
        doc: Some("Copies bar close prices. Returns count or -1 on error."),
    },
    BuiltinFunction {
        name: "CopyTickVolume",
        signature: "int CopyTickVolume(string symbol, ENUM_TIMEFRAMES timeframe, int start_pos, int count, long& volume_array[])",
        doc: Some("Copies bar tick volumes. Returns count or -1 on error."),
    },
    BuiltinFunction {
        name: "CopyRealVolume",
        signature: "int CopyRealVolume(string symbol, ENUM_TIMEFRAMES timeframe, int start_pos, int count, long& volume_array[])",
        doc: Some("Copies bar real volumes. Returns count or -1 on error."),
    },
    BuiltinFunction {
        name: "CopySpread",
        signature: "int CopySpread(string symbol, ENUM_TIMEFRAMES timeframe, int start_pos, int count, int& spread_array[])",
        doc: Some("Copies bar spreads. Returns count or -1 on error."),
    },
    BuiltinFunction {
        name: "CopyTicks",
        signature: "int CopyTicks(string symbol, MqlTick& ticks_array[], uint flags = 0, ulong from = 0, uint count = 0)",
        doc: Some("Copies ticks into an MqlTick array. Returns count or -1 on error."),
    },
    BuiltinFunction {
        name: "CopyTicksRange",
        signature: "int CopyTicksRange(string symbol, MqlTick& ticks_array[], uint flags, ulong from, ulong to)",
        doc: Some("Copies ticks within a time range into an MqlTick array."),
    },
    BuiltinFunction {
        name: "Bars",
        signature: "int Bars(string symbol, ENUM_TIMEFRAMES timeframe)",
        doc: Some("Returns the number of bars for the specified symbol and timeframe."),
    },
    BuiltinFunction {
        name: "iBars",
        signature: "int iBars(string symbol, ENUM_TIMEFRAMES timeframe)",
        doc: Some("Returns the number of bars for the specified symbol and timeframe."),
    },
    BuiltinFunction {
        name: "iBarShift",
        signature: "int iBarShift(string symbol, ENUM_TIMEFRAMES timeframe, datetime time, bool exact = false)",
        doc: Some("Returns the bar index (shift) for the specified time. If exact=true and no bar matches, returns -1."),
    },
    BuiltinFunction {
        name: "iTime",
        signature: "datetime iTime(string symbol, ENUM_TIMEFRAMES timeframe, int shift)",
        doc: Some("Returns the open time of the bar at the specified shift."),
    },
    BuiltinFunction {
        name: "iOpen",
        signature: "double iOpen(string symbol, ENUM_TIMEFRAMES timeframe, int shift)",
        doc: Some("Returns the open price of the bar at the specified shift."),
    },
    BuiltinFunction {
        name: "iHigh",
        signature: "double iHigh(string symbol, ENUM_TIMEFRAMES timeframe, int shift)",
        doc: Some("Returns the high price of the bar at the specified shift."),
    },
    BuiltinFunction {
        name: "iLow",
        signature: "double iLow(string symbol, ENUM_TIMEFRAMES timeframe, int shift)",
        doc: Some("Returns the low price of the bar at the specified shift."),
    },
    BuiltinFunction {
        name: "iClose",
        signature: "double iClose(string symbol, ENUM_TIMEFRAMES timeframe, int shift)",
        doc: Some("Returns the close price of the bar at the specified shift."),
    },
    BuiltinFunction {
        name: "iVolume",
        signature: "long iVolume(string symbol, ENUM_TIMEFRAMES timeframe, int shift)",
        doc: Some("Returns the tick volume of the bar at the specified shift."),
    },
    BuiltinFunction {
        name: "iSpread",
        signature: "int iSpread(string symbol, ENUM_TIMEFRAMES timeframe, int shift)",
        doc: Some("Returns the spread of the bar at the specified shift."),
    },
    BuiltinFunction {
        name: "iTickVolume",
        signature: "long iTickVolume(string symbol, ENUM_TIMEFRAMES timeframe, int shift)",
        doc: Some("Returns the tick volume of the bar at the specified shift."),
    },
    BuiltinFunction {
        name: "iRealVolume",
        signature: "long iRealVolume(string symbol, ENUM_TIMEFRAMES timeframe, int shift)",
        doc: Some("Returns the real volume of the bar at the specified shift."),
    },
    BuiltinFunction {
        name: "SeriesInfoInteger",
        signature: "long SeriesInfoInteger(string symbol, ENUM_TIMEFRAMES timeframe, ENUM_SERIES_INFO_INTEGER property_id)",
        doc: Some("Returns information about the state of historical data for a symbol/timeframe."),
    },
    BuiltinFunction {
        name: "IndicatorCreate",
        signature: "int IndicatorCreate(string symbol, ENUM_TIMEFRAMES period, int indicator_type, int parameters_cnt = 0, const MqlParam& parameters_array[] = {})",
        doc: Some("Creates an indicator by type and returns its handle."),
    },
    BuiltinFunction {
        name: "IndicatorRelease",
        signature: "int IndicatorRelease(int indicator_handle)",
        doc: Some("Releases an indicator handle. Returns 1 on success."),
    },
    // === Chart Functions ===
    BuiltinFunction {
        name: "ChartOpen",
        signature: "long ChartOpen(string symbol, ENUM_TIMEFRAMES period)",
        doc: Some("Opens a new chart for the specified symbol and timeframe. Returns the chart ID."),
    },
    BuiltinFunction {
        name: "ChartFirst",
        signature: "long ChartFirst()",
        doc: Some("Returns the ID of the first chart in the terminal."),
    },
    BuiltinFunction {
        name: "ChartNext",
        signature: "long ChartNext(long chart_id)",
        doc: Some("Returns the ID of the chart next to the specified one, or -1 if none."),
    },
    BuiltinFunction {
        name: "ChartClose",
        signature: "bool ChartClose(long chart_id = 0)",
        doc: Some("Closes the specified chart. 0 means the current chart."),
    },
    BuiltinFunction {
        name: "ChartSymbol",
        signature: "string ChartSymbol(long chart_id = 0)",
        doc: Some("Returns the symbol name of the specified chart."),
    },
    BuiltinFunction {
        name: "ChartPeriod",
        signature: "ENUM_TIMEFRAMES ChartPeriod(long chart_id = 0)",
        doc: Some("Returns the timeframe of the specified chart."),
    },
    BuiltinFunction {
        name: "ChartRedraw",
        signature: "void ChartRedraw(long chart_id = 0)",
        doc: Some("Forces a redraw of the specified chart."),
    },
    BuiltinFunction {
        name: "ChartSetInteger",
        signature: "bool ChartSetInteger(long chart_id, ENUM_CHART_PROPERTY_INTEGER property_id, long value)",
        doc: Some("Sets an integer property of the chart."),
    },
    BuiltinFunction {
        name: "ChartSetDouble",
        signature: "bool ChartSetDouble(long chart_id, ENUM_CHART_PROPERTY_DOUBLE property_id, double value)",
        doc: Some("Sets a double property of the chart."),
    },
    BuiltinFunction {
        name: "ChartSetString",
        signature: "bool ChartSetString(long chart_id, ENUM_CHART_PROPERTY_STRING property_id, string value)",
        doc: Some("Sets a string property of the chart."),
    },
    BuiltinFunction {
        name: "ChartGetInteger",
        signature: "long ChartGetInteger(long chart_id, ENUM_CHART_PROPERTY_INTEGER property_id, int sub_window = 0)",
        doc: Some("Returns an integer property of the chart."),
    },
    BuiltinFunction {
        name: "ChartGetDouble",
        signature: "double ChartGetDouble(long chart_id, ENUM_CHART_PROPERTY_DOUBLE property_id, int sub_window = 0)",
        doc: Some("Returns a double property of the chart."),
    },
    BuiltinFunction {
        name: "ChartGetString",
        signature: "string ChartGetString(long chart_id, ENUM_CHART_PROPERTY_STRING property_id)",
        doc: Some("Returns a string property of the chart."),
    },
    BuiltinFunction {
        name: "ChartNavigate",
        signature: "bool ChartNavigate(long chart_id, int position, int shift = 0)",
        doc: Some("Navigates the chart. Position: CHART_BEGIN, CHART_CURRENT_POS, or CHART_END."),
    },
    BuiltinFunction {
        name: "ChartID",
        signature: "long ChartID()",
        doc: Some("Returns the ID of the current chart."),
    },
    BuiltinFunction {
        name: "ChartWindowFind",
        signature: "int ChartWindowFind(long chart_id, string indicator_shortname)",
        doc: Some("Returns the subwindow number containing the specified indicator, or -1."),
    },
    BuiltinFunction {
        name: "ChartTimePriceToXY",
        signature: "bool ChartTimePriceToXY(long chart_id, int sub_window, datetime time, double price, int& x, int& y)",
        doc: Some("Converts chart time/price coordinates to screen X/Y pixels."),
    },
    BuiltinFunction {
        name: "ChartXYToTimePrice",
        signature: "bool ChartXYToTimePrice(long chart_id, int x, int y, int& sub_window, datetime& time, double& price)",
        doc: Some("Converts screen X/Y coordinates to chart time/price."),
    },
    BuiltinFunction {
        name: "ChartWindowOnDropped",
        signature: "int ChartWindowOnDropped()",
        doc: Some("Returns the subwindow number where the EA/script was dropped."),
    },
    BuiltinFunction {
        name: "ChartPriceOnDropped",
        signature: "double ChartPriceOnDropped()",
        doc: Some("Returns the price where the EA/script was dropped."),
    },
    BuiltinFunction {
        name: "ChartTimeOnDropped",
        signature: "datetime ChartTimeOnDropped()",
        doc: Some("Returns the time where the EA/script was dropped."),
    },
    BuiltinFunction {
        name: "ChartIndicatorName",
        signature: "string ChartIndicatorName(long chart_id, int sub_window, int index)",
        doc: Some("Returns the short name of an indicator in the specified subwindow."),
    },
    BuiltinFunction {
        name: "ChartIndicatorsTotal",
        signature: "int ChartIndicatorsTotal(long chart_id, int sub_window)",
        doc: Some("Returns the number of indicators in the specified subwindow."),
    },
    BuiltinFunction {
        name: "ChartIndicatorAdd",
        signature: "bool ChartIndicatorAdd(long chart_id, int sub_window, int indicator_handle)",
        doc: Some("Adds an indicator to the specified chart subwindow."),
    },
    BuiltinFunction {
        name: "ChartIndicatorDelete",
        signature: "bool ChartIndicatorDelete(long chart_id, int sub_window, string indicator_shortname)",
        doc: Some("Removes an indicator from the chart by its short name."),
    },
    BuiltinFunction {
        name: "ChartIndicatorGet",
        signature: "int ChartIndicatorGet(long chart_id, int sub_window, string indicator_shortname)",
        doc: Some("Returns the handle of an indicator by its short name, or INVALID_HANDLE."),
    },
    BuiltinFunction {
        name: "ChartSetSymbolPeriod",
        signature: "bool ChartSetSymbolPeriod(long chart_id, string symbol, ENUM_TIMEFRAMES period)",
        doc: Some("Changes the chart symbol and/or timeframe."),
    },
    BuiltinFunction {
        name: "ChartScreenShot",
        signature: "bool ChartScreenShot(long chart_id, string filename, int width, int height, ENUM_ALIGN_MODE align_mode = ALIGN_RIGHT)",
        doc: Some("Saves a screenshot of the chart to a BMP file."),
    },
    BuiltinFunction {
        name: "ChartApplyTemplate",
        signature: "bool ChartApplyTemplate(long chart_id, string filename)",
        doc: Some("Applies a template to the chart."),
    },
    BuiltinFunction {
        name: "ChartSaveTemplate",
        signature: "bool ChartSaveTemplate(long chart_id, string filename)",
        doc: Some("Saves the chart template to a file."),
    },
    // === Object Functions ===
    BuiltinFunction {
        name: "ObjectCreate",
        signature: "bool ObjectCreate(long chart_id, string name, ENUM_OBJECT type, int sub_window, datetime time1, double price1, datetime time2 = 0, double price2 = 0, datetime time3 = 0, double price3 = 0)",
        doc: Some("Creates a graphical object on the chart. Returns true on success."),
    },
    BuiltinFunction {
        name: "ObjectDelete",
        signature: "bool ObjectDelete(long chart_id, string name)",
        doc: Some("Deletes a graphical object from the chart by name."),
    },
    BuiltinFunction {
        name: "ObjectMove",
        signature: "bool ObjectMove(long chart_id, string name, int point_index, datetime time, double price)",
        doc: Some("Moves an anchor point of a graphical object."),
    },
    BuiltinFunction {
        name: "ObjectsDeleteAll",
        signature: "int ObjectsDeleteAll(long chart_id, int sub_window = -1, int type = -1)",
        doc: Some("Deletes all objects from the chart. Returns the number of deleted objects."),
    },
    BuiltinFunction {
        name: "ObjectFind",
        signature: "int ObjectFind(long chart_id, string name)",
        doc: Some("Searches for an object by name. Returns the subwindow index or -1 if not found."),
    },
    BuiltinFunction {
        name: "ObjectsTotal",
        signature: "int ObjectsTotal(long chart_id, int sub_window = -1, int type = -1)",
        doc: Some("Returns the total number of objects on the chart."),
    },
    BuiltinFunction {
        name: "ObjectName",
        signature: "string ObjectName(long chart_id, int pos, int sub_window = -1, int type = -1)",
        doc: Some("Returns the name of a graphical object by its index."),
    },
    BuiltinFunction {
        name: "ObjectSetInteger",
        signature: "bool ObjectSetInteger(long chart_id, string name, ENUM_OBJECT_PROPERTY_INTEGER property_id, long value)",
        doc: Some("Sets an integer property of a graphical object."),
    },
    BuiltinFunction {
        name: "ObjectSetDouble",
        signature: "bool ObjectSetDouble(long chart_id, string name, ENUM_OBJECT_PROPERTY_DOUBLE property_id, double value)",
        doc: Some("Sets a double property of a graphical object."),
    },
    BuiltinFunction {
        name: "ObjectSetString",
        signature: "bool ObjectSetString(long chart_id, string name, ENUM_OBJECT_PROPERTY_STRING property_id, string value)",
        doc: Some("Sets a string property of a graphical object."),
    },
    BuiltinFunction {
        name: "ObjectGetInteger",
        signature: "long ObjectGetInteger(long chart_id, string name, ENUM_OBJECT_PROPERTY_INTEGER property_id, int modifier = 0)",
        doc: Some("Returns an integer property of a graphical object."),
    },
    BuiltinFunction {
        name: "ObjectGetDouble",
        signature: "double ObjectGetDouble(long chart_id, string name, ENUM_OBJECT_PROPERTY_DOUBLE property_id, int modifier = 0)",
        doc: Some("Returns a double property of a graphical object."),
    },
    BuiltinFunction {
        name: "ObjectGetString",
        signature: "string ObjectGetString(long chart_id, string name, ENUM_OBJECT_PROPERTY_STRING property_id, int modifier = 0)",
        doc: Some("Returns a string property of a graphical object."),
    },
    BuiltinFunction {
        name: "ObjectGetTimeByValue",
        signature: "double ObjectGetTimeByValue(long chart_id, string name, double value, int line_id = 0)",
        doc: Some("Returns the time value for the specified price on a trend line."),
    },
    BuiltinFunction {
        name: "ObjectGetValueByTime",
        signature: "double ObjectGetValueByTime(long chart_id, string name, datetime time, int line_id = 0)",
        doc: Some("Returns the price value for the specified time on a trend line."),
    },
    // === Array Functions ===
    BuiltinFunction {
        name: "ArraySize",
        signature: "int ArraySize(const void& array[])",
        doc: Some("Returns the number of elements in the array."),
    },
    BuiltinFunction {
        name: "ArrayResize",
        signature: "int ArrayResize(void& array[], int new_size, int reserve_size = 0)",
        doc: Some("Resizes the first dimension of a dynamic array. Returns the new size or -1 on error."),
    },
    BuiltinFunction {
        name: "ArrayFree",
        signature: "void ArrayFree(void& array[])",
        doc: Some("Frees the memory of a dynamic array, setting its size to 0."),
    },
    BuiltinFunction {
        name: "ArrayCopy",
        signature: "int ArrayCopy(void& dst_array[], const void& src_array[], int dst_start = 0, int src_start = 0, int count = WHOLE_ARRAY)",
        doc: Some("Copies elements from one array to another. Returns the number of copied elements."),
    },
    BuiltinFunction {
        name: "ArraySort",
        signature: "bool ArraySort(void& array[])",
        doc: Some("Sorts the array in ascending order. Returns true on success."),
    },
    BuiltinFunction {
        name: "ArrayBsearch",
        signature: "int ArrayBsearch(const void& array[], double value)",
        doc: Some("Binary search in a sorted array. Returns the index of the found element."),
    },
    BuiltinFunction {
        name: "ArrayMinimum",
        signature: "int ArrayMinimum(const void& array[], int start = 0, int count = WHOLE_ARRAY)",
        doc: Some("Returns the index of the minimum value in the array."),
    },
    BuiltinFunction {
        name: "ArrayMaximum",
        signature: "int ArrayMaximum(const void& array[], int start = 0, int count = WHOLE_ARRAY)",
        doc: Some("Returns the index of the maximum value in the array."),
    },
    BuiltinFunction {
        name: "ArrayRange",
        signature: "double ArrayRange(const void& array[], int rank_index)",
        doc: None,
    },
    BuiltinFunction {
        name: "ArrayReverse",
        signature: "bool ArrayReverse(void& array[], int start = 0, int count = WHOLE_ARRAY)",
        doc: Some("Reverses the order of elements in the array."),
    },
    BuiltinFunction {
        name: "ArrayInitialize",
        signature: "void ArrayInitialize(void& array[], int value)",
        doc: Some("Initializes all elements of a numeric array with a given value."),
    },
    BuiltinFunction {
        name: "ArrayFill",
        signature: "void ArrayFill(void& array[], int start, int count, int value)",
        doc: Some("Fills a range of array elements with a given value."),
    },
    BuiltinFunction {
        name: "ArraySetAsSeries",
        signature: "bool ArraySetAsSeries(void& array[], bool flag)",
        doc: Some("Sets the array indexing direction (true = reverse, like timeseries)."),
    },
    BuiltinFunction {
        name: "ArrayIsSeries",
        signature: "bool ArrayIsSeries(const void& array[])",
        doc: Some("Returns true if the array is indexed as a timeseries (newest first)."),
    },
    BuiltinFunction {
        name: "ArrayIsDynamic",
        signature: "bool ArrayIsDynamic(const void& array[])",
        doc: Some("Returns true if the array is dynamic."),
    },
    BuiltinFunction {
        name: "ArrayPrint",
        signature: "void ArrayPrint(const void& array[], int digits = -1, string separator = \" \", ulong start = 0, ulong count = WHOLE_ARRAY, ulong flags = 0)",
        doc: Some("Prints array contents to the Experts tab."),
    },
    BuiltinFunction {
        name: "ArraySwap",
        signature: "void ArraySwap(void& array1[], void& array2[])",
        doc: Some("Swaps the contents of two dynamic arrays."),
    },
    BuiltinFunction {
        name: "ArrayRemove",
        signature: "bool ArrayRemove(void& array[], int start, int count)",
        doc: Some("Removes elements from a dynamic array. Returns true on success."),
    },
    BuiltinFunction {
        name: "ArrayInsert",
        signature: "bool ArrayInsert(void& dst_array[], const void& src_array[], int dst_start, int src_start = 0, int count = WHOLE_ARRAY)",
        doc: Some("Inserts elements from one array into another at the specified position."),
    },
    // === String Functions ===
    BuiltinFunction {
        name: "StringAdd",
        signature: "bool StringAdd(string& string_var, string add_substring)",
        doc: Some("Appends a substring to a string. More efficient than + operator for repeated concatenation."),
    },
    BuiltinFunction {
        name: "StringBufferLen",
        signature: "int StringBufferLen(string string_var)",
        doc: Some("Returns the allocated buffer size of the string."),
    },
    BuiltinFunction {
        name: "StringCompare",
        signature: "int StringCompare(string string1, string string2, bool case_sensitive = true)",
        doc: Some("Compares two strings. Returns 0 if equal, -1 or 1 otherwise."),
    },
    BuiltinFunction {
        name: "StringConcatenate",
        signature: "int StringConcatenate(string& string_var, ...)",
        doc: Some("Concatenates multiple values into a string. Returns the length."),
    },
    BuiltinFunction {
        name: "StringFill",
        signature: "int StringFill(string& string_var, ushort character)",
        doc: Some("Fills the string with the specified character."),
    },
    BuiltinFunction {
        name: "StringFind",
        signature: "int StringFind(string string_value, string match_substring, int start_pos = 0)",
        doc: Some("Searches for a substring. Returns the position or -1 if not found."),
    },
    BuiltinFunction {
        name: "StringFormat",
        signature: "string StringFormat(string format, ...)",
        doc: Some("Formats a string with printf-style format specifiers."),
    },
    BuiltinFunction {
        name: "StringGetCharacter",
        signature: "ushort StringGetCharacter(string string_value, int pos)",
        doc: Some("Returns the character (Unicode) at the specified position."),
    },
    BuiltinFunction {
        name: "StringInit",
        signature: "bool StringInit(string& string_var, int new_len = 0, ushort character = 0)",
        doc: Some("Initializes a string with a specified length and fill character."),
    },
    BuiltinFunction {
        name: "StringLen",
        signature: "int StringLen(string string_value)",
        doc: Some("Returns the number of characters in the string."),
    },
    BuiltinFunction {
        name: "StringReplace",
        signature: "bool StringReplace(string& str, string find, string replacement)",
        doc: Some("Replaces all occurrences of a substring."),
    },
    BuiltinFunction {
        name: "StringSetCharacter",
        signature: "bool StringSetCharacter(string& string_var, int pos, ushort character)",
        doc: Some("Sets a character at the specified position."),
    },
    BuiltinFunction {
        name: "StringSplit",
        signature: "int StringSplit(string string_value, ushort separator, string& result[])",
        doc: Some("Splits a string by the separator character. Returns the number of parts."),
    },
    BuiltinFunction {
        name: "StringSubstr",
        signature: "string StringSubstr(string string_value, int start_pos, int length = -1)",
        doc: Some("Extracts a substring. If length=-1, returns from start_pos to end."),
    },
    BuiltinFunction {
        name: "StringToLower",
        signature: "string StringToLower(string string_value)",
        doc: Some("Converts string to lowercase."),
    },
    BuiltinFunction {
        name: "StringToUpper",
        signature: "string StringToUpper(string string_value)",
        doc: Some("Converts string to uppercase."),
    },
    BuiltinFunction {
        name: "StringTrimLeft",
        signature: "string StringTrimLeft(string string_value)",
        doc: Some("Removes whitespace from the left side of the string."),
    },
    BuiltinFunction {
        name: "StringTrimRight",
        signature: "string StringTrimRight(string string_value)",
        doc: Some("Removes whitespace from the right side of the string."),
    },
    BuiltinFunction {
        name: "StringToCharArray",
        signature: "int StringToCharArray(string text_string, uchar& array[], int start = 0, int count = -1, uint codepage = 0)",
        doc: Some("Copies a string to a uchar array. Returns the number of copied elements."),
    },
    BuiltinFunction {
        name: "StringToShortArray",
        signature: "int StringToShortArray(string text_string, ushort& array[], int start = 0, int count = -1)",
        doc: Some("Copies a string to a ushort array. Returns the number of copied elements."),
    },
    BuiltinFunction {
        name: "ShortToString",
        signature: "string ShortToString(ushort char_code)",
        doc: Some("Converts a Unicode character code to a one-character string."),
    },
    BuiltinFunction {
        name: "ShortArrayToString",
        signature: "string ShortArrayToString(ushort& array[], int start = 0, int count = -1)",
        doc: Some("Converts a ushort array to a string."),
    },
    BuiltinFunction {
        name: "CharArrayToString",
        signature: "string CharArrayToString(uchar& array[], int start = 0, int count = -1, uint codepage = 0)",
        doc: Some("Converts a uchar array to a string."),
    },
    // === Conversion Functions ===
    BuiltinFunction {
        name: "ColorToARGB",
        signature: "uint ColorToARGB(color clr, uchar alpha = 255)",
        doc: Some("Converts a color value to ARGB format with the specified alpha."),
    },
    BuiltinFunction {
        name: "ColorToString",
        signature: "string ColorToString(color color_value, bool color_name = false)",
        doc: Some("Converts a color to a string representation."),
    },
    BuiltinFunction {
        name: "DoubleToString",
        signature: "string DoubleToString(double value, int digits = 8)",
        doc: Some("Converts a double to a string with the specified number of decimal places."),
    },
    BuiltinFunction {
        name: "EnumToString",
        signature: "string EnumToString(int value)",
        doc: Some("Converts an enum value to its string name."),
    },
    BuiltinFunction {
        name: "IntegerToString",
        signature: "string IntegerToString(long number, int str_len = 0, ushort fill_char = ' ')",
        doc: Some("Converts an integer to a string, optionally padded."),
    },
    BuiltinFunction {
        name: "TimeToString",
        signature: "string TimeToString(datetime value, int mode = 0)",
        doc: Some("Converts a datetime to a string. Mode flags: TIME_DATE, TIME_MINUTES, TIME_SECONDS."),
    },
    BuiltinFunction {
        name: "StringToDouble",
        signature: "double StringToDouble(string value)",
        doc: Some("Converts a string to a double value."),
    },
    BuiltinFunction {
        name: "StringToInteger",
        signature: "long StringToInteger(string value)",
        doc: Some("Converts a string to a long integer value."),
    },
    BuiltinFunction {
        name: "StringToTime",
        signature: "datetime StringToTime(string value)",
        doc: Some("Converts a string to a datetime value."),
    },
    BuiltinFunction {
        name: "StringToColor",
        signature: "color StringToColor(string color_string)",
        doc: Some("Converts a string color name or RGB representation to a color value."),
    },
    BuiltinFunction {
        name: "NormalizeDouble",
        signature: "double NormalizeDouble(double value, int digits)",
        doc: Some("Rounds a double to the specified number of decimal places. Essential for price comparisons."),
    },
    // === Math Functions ===
    BuiltinFunction {
        name: "MathAbs",
        signature: "double MathAbs(double value)",
        doc: Some("Returns the absolute value."),
    },
    BuiltinFunction {
        name: "MathArccos",
        signature: "double MathArccos(double value)",
        doc: None,
    },
    BuiltinFunction {
        name: "MathArcsin",
        signature: "double MathArcsin(double value)",
        doc: None,
    },
    BuiltinFunction {
        name: "MathArctan",
        signature: "double MathArctan(double value)",
        doc: None,
    },
    BuiltinFunction {
        name: "MathCeil",
        signature: "double MathCeil(double value)",
        doc: Some("Returns the smallest integer >= value."),
    },
    BuiltinFunction {
        name: "MathCos",
        signature: "double MathCos(double value)",
        doc: None,
    },
    BuiltinFunction {
        name: "MathExp",
        signature: "double MathExp(double value)",
        doc: None,
    },
    BuiltinFunction {
        name: "MathFloor",
        signature: "double MathFloor(double value)",
        doc: Some("Returns the largest integer <= value."),
    },
    BuiltinFunction {
        name: "MathLog",
        signature: "double MathLog(double value)",
        doc: None,
    },
    BuiltinFunction {
        name: "MathLog10",
        signature: "double MathLog10(double value)",
        doc: None,
    },
    BuiltinFunction {
        name: "MathMax",
        signature: "double MathMax(double value1, double value2)",
        doc: Some("Returns the greater of two values."),
    },
    BuiltinFunction {
        name: "MathMin",
        signature: "double MathMin(double value1, double value2)",
        doc: Some("Returns the lesser of two values."),
    },
    BuiltinFunction {
        name: "MathMod",
        signature: "double MathMod(double value, double value2)",
        doc: Some("Returns the remainder of division (fmod)."),
    },
    BuiltinFunction {
        name: "MathPow",
        signature: "double MathPow(double base, double exponent)",
        doc: Some("Returns base raised to the power of exponent."),
    },
    BuiltinFunction {
        name: "MathRand",
        signature: "int MathRand()",
        doc: Some("Returns a pseudorandom integer in the range 0..32767."),
    },
    BuiltinFunction {
        name: "MathRound",
        signature: "double MathRound(double value)",
        doc: Some("Rounds to the nearest integer."),
    },
    BuiltinFunction {
        name: "MathSin",
        signature: "double MathSin(double value)",
        doc: None,
    },
    BuiltinFunction {
        name: "MathSqrt",
        signature: "double MathSqrt(double value)",
        doc: Some("Returns the square root."),
    },
    BuiltinFunction {
        name: "MathSrand",
        signature: "void MathSrand(int seed)",
        doc: Some("Sets the seed for the pseudorandom number generator."),
    },
    BuiltinFunction {
        name: "MathTan",
        signature: "double MathTan(double value)",
        doc: None,
    },
    BuiltinFunction {
        name: "MathIsValidNumber",
        signature: "bool MathIsValidNumber(double value)",
        doc: Some("Returns true if the value is a valid (not NaN/Inf) number."),
    },
    BuiltinFunction {
        name: "MathSwap",
        signature: "void MathSwap(double& var1, double& var2)",
        doc: Some("Swaps the values of two double variables."),
    },
    // === Time Functions ===
    BuiltinFunction {
        name: "TimeCurrent",
        signature: "datetime TimeCurrent()",
        doc: Some("Returns the last known server time (time of the last quote)."),
    },
    BuiltinFunction {
        name: "TimeTradeServer",
        signature: "datetime TimeTradeServer()",
        doc: Some("Returns the current trade server time."),
    },
    BuiltinFunction {
        name: "TimeLocal",
        signature: "datetime TimeLocal()",
        doc: Some("Returns the local computer time."),
    },
    BuiltinFunction {
        name: "TimeGMT",
        signature: "datetime TimeGMT()",
        doc: Some("Returns the GMT time."),
    },
    BuiltinFunction {
        name: "TimeGMTOffset",
        signature: "int TimeGMTOffset()",
        doc: Some("Returns the difference between GMT and local time in seconds."),
    },
    BuiltinFunction {
        name: "TimeDaylightSavings",
        signature: "int TimeDaylightSavings()",
        doc: Some("Returns the daylight savings time correction in seconds."),
    },
    BuiltinFunction {
        name: "TimeToStruct",
        signature: "void TimeToStruct(datetime dt, MqlDateTime& dt_struct)",
        doc: Some("Converts a datetime to an MqlDateTime structure."),
    },
    BuiltinFunction {
        name: "StructToTime",
        signature: "datetime StructToTime(MqlDateTime& dt_struct)",
        doc: Some("Converts an MqlDateTime structure to a datetime value."),
    },
    BuiltinFunction {
        name: "PeriodSeconds",
        signature: "int PeriodSeconds(ENUM_TIMEFRAMES period = PERIOD_CURRENT)",
        doc: Some("Returns the number of seconds in the specified timeframe period."),
    },
    // === File Functions ===
    BuiltinFunction {
        name: "FileOpen",
        signature: "int FileOpen(string file_name, int open_flags, short delimiter = '\\t', uint codepage = 0)",
        doc: Some("Opens a file. Returns a file handle or INVALID_HANDLE on error."),
    },
    BuiltinFunction {
        name: "FileClose",
        signature: "void FileClose(int file_handle)",
        doc: Some("Closes a previously opened file."),
    },
    BuiltinFunction {
        name: "FileDelete",
        signature: "bool FileDelete(string file_name, int common_flag = 0)",
        doc: Some("Deletes a file. Use FILE_COMMON flag for common folder."),
    },
    BuiltinFunction {
        name: "FileCopy",
        signature: "bool FileCopy(string src_file_name, int common_flag, string dst_file_name, int mode_flags)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileMove",
        signature: "bool FileMove(string src_file_name, int common_flag, string dst_file_name, int mode_flags)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileFlush",
        signature: "void FileFlush(int file_handle)",
        doc: Some("Flushes the file buffer to disk."),
    },
    BuiltinFunction {
        name: "FileIsExist",
        signature: "bool FileIsExist(string file_name, int common_flag = 0)",
        doc: Some("Checks if a file exists."),
    },
    BuiltinFunction {
        name: "FileIsEnding",
        signature: "bool FileIsEnding(int file_handle)",
        doc: Some("Returns true if the file pointer is at the end of the file."),
    },
    BuiltinFunction {
        name: "FileIsLineEnding",
        signature: "bool FileIsLineEnding(int file_handle)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileReadBool",
        signature: "bool FileReadBool(int file_handle)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileReadDatetime",
        signature: "datetime FileReadDatetime(int file_handle)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileReadDouble",
        signature: "double FileReadDouble(int file_handle)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileReadFloat",
        signature: "float FileReadFloat(int file_handle)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileReadInteger",
        signature: "int FileReadInteger(int file_handle, int size = 4)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileReadLong",
        signature: "long FileReadLong(int file_handle)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileReadNumber",
        signature: "double FileReadNumber(int file_handle)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileReadString",
        signature: "string FileReadString(int file_handle, int length = -1)",
        doc: Some("Reads a string from a file. If length=-1, reads until delimiter or EOF."),
    },
    BuiltinFunction {
        name: "FileReadStruct",
        signature: "uint FileReadStruct(int file_handle, void& struct_object, int size = -1)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileReadArray",
        signature: "uint FileReadArray(int file_handle, void& array[], int start = 0, int count = WHOLE_ARRAY)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileWrite",
        signature: "uint FileWrite(int file_handle, ...)",
        doc: Some("Writes data to a CSV or TXT file. Parameters are separated by the delimiter."),
    },
    BuiltinFunction {
        name: "FileWriteArray",
        signature: "uint FileWriteArray(int file_handle, const void& array[], int start = 0, int count = WHOLE_ARRAY)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileWriteDouble",
        signature: "uint FileWriteDouble(int file_handle, double value)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileWriteFloat",
        signature: "uint FileWriteFloat(int file_handle, float value)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileWriteInteger",
        signature: "uint FileWriteInteger(int file_handle, int value, int size = 4)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileWriteLong",
        signature: "uint FileWriteLong(int file_handle, long value)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileWriteString",
        signature: "uint FileWriteString(int file_handle, string text_string, int length = -1)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileWriteStruct",
        signature: "uint FileWriteStruct(int file_handle, const void& struct_object, int size = -1)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileSeek",
        signature: "bool FileSeek(int file_handle, long offset, int origin)",
        doc: Some("Moves the file pointer. Origin: SEEK_SET, SEEK_CUR, SEEK_END."),
    },
    BuiltinFunction {
        name: "FileTell",
        signature: "ulong FileTell(int file_handle)",
        doc: Some("Returns the current position of the file pointer."),
    },
    BuiltinFunction {
        name: "FileSize",
        signature: "ulong FileSize(int file_handle)",
        doc: Some("Returns the file size in bytes."),
    },
    BuiltinFunction {
        name: "FileFindFirst",
        signature: "long FileFindFirst(string file_filter, string& returned_filename, int common_flag = 0)",
        doc: Some("Starts file search. Returns a search handle or INVALID_HANDLE."),
    },
    BuiltinFunction {
        name: "FileFindNext",
        signature: "bool FileFindNext(long search_handle, string& returned_filename)",
        doc: Some("Continues file search. Returns false when no more files."),
    },
    BuiltinFunction {
        name: "FileFindClose",
        signature: "void FileFindClose(long search_handle)",
        doc: Some("Closes a file search handle."),
    },
    BuiltinFunction {
        name: "FileGetInteger",
        signature: "bool FileGetInteger(int file_handle, int property_id)",
        doc: None,
    },
    BuiltinFunction {
        name: "FileLoad",
        signature: "int FileLoad(string file_name, void& buffer[], int common_flag = 0)",
        doc: Some("Reads the entire file into a buffer array."),
    },
    BuiltinFunction {
        name: "FileSave",
        signature: "bool FileSave(string file_name, const void& buffer[], int common_flag = 0)",
        doc: Some("Saves a buffer array to a file."),
    },
    // === Global Variables (of Terminal) ===
    BuiltinFunction {
        name: "GlobalVariableCheck",
        signature: "bool GlobalVariableCheck(string name)",
        doc: Some("Checks if a global variable exists in the terminal."),
    },
    BuiltinFunction {
        name: "GlobalVariableDel",
        signature: "bool GlobalVariableDel(string name)",
        doc: Some("Deletes a global variable from the terminal."),
    },
    BuiltinFunction {
        name: "GlobalVariableGet",
        signature: "double GlobalVariableGet(string name)",
        doc: Some("Returns the value of a global terminal variable."),
    },
    BuiltinFunction {
        name: "GlobalVariableName",
        signature: "string GlobalVariableName(int index)",
        doc: None,
    },
    BuiltinFunction {
        name: "GlobalVariableSet",
        signature: "bool GlobalVariableSet(string name, double value)",
        doc: Some("Sets the value of a global terminal variable. Creates it if needed."),
    },
    BuiltinFunction {
        name: "GlobalVariableSetOnCondition",
        signature: "bool GlobalVariableSetOnCondition(string name, double value, double check_value)",
        doc: Some("Sets value only if the current value equals check_value (atomic compare-and-swap)."),
    },
    BuiltinFunction {
        name: "GlobalVariableTemp",
        signature: "bool GlobalVariableTemp(string name)",
        doc: Some("Creates a temporary global variable that is deleted when the terminal closes."),
    },
    BuiltinFunction {
        name: "GlobalVariableTime",
        signature: "datetime GlobalVariableTime(string name)",
        doc: None,
    },
    BuiltinFunction {
        name: "GlobalVariablesDeleteAll",
        signature: "int GlobalVariablesDeleteAll(string prefix_name = \"\", datetime limit_data = 0)",
        doc: None,
    },
    BuiltinFunction {
        name: "GlobalVariablesFlush",
        signature: "bool GlobalVariablesFlush()",
        doc: None,
    },
    BuiltinFunction {
        name: "GlobalVariablesTotal",
        signature: "int GlobalVariablesTotal()",
        doc: None,
    },
    // === Common Functions ===
    BuiltinFunction {
        name: "Print",
        signature: "void Print(...)",
        doc: Some("Prints a message to the Experts tab in the terminal."),
    },
    BuiltinFunction {
        name: "PrintFormat",
        signature: "void PrintFormat(string format, ...)",
        doc: Some("Prints a formatted message (printf-style) to the Experts tab."),
    },
    BuiltinFunction {
        name: "Alert",
        signature: "void Alert(...)",
        doc: Some("Displays an alert dialog with a message and plays a sound."),
    },
    BuiltinFunction {
        name: "Comment",
        signature: "void Comment(...)",
        doc: Some("Displays text in the upper-left corner of the chart."),
    },
    BuiltinFunction {
        name: "SendFTP",
        signature: "bool SendFTP(string filename, string ftp_path = \"\")",
        doc: None,
    },
    BuiltinFunction {
        name: "SendMail",
        signature: "bool SendMail(string subject, string text)",
        doc: Some("Sends an email using the settings configured in the terminal."),
    },
    BuiltinFunction {
        name: "SendNotification",
        signature: "bool SendNotification(string text)",
        doc: Some("Sends a push notification to the mobile terminal."),
    },
    BuiltinFunction {
        name: "Sleep",
        signature: "void Sleep(int milliseconds)",
        doc: Some("Suspends execution for the specified number of milliseconds. Only works in EAs/scripts."),
    },
    BuiltinFunction {
        name: "MessageBox",
        signature: "int MessageBox(string text, string caption = \"\", int flags = 0)",
        doc: Some("Displays a message box dialog. Returns the button clicked (IDOK, IDCANCEL, etc.)."),
    },
    BuiltinFunction {
        name: "PlaySound",
        signature: "bool PlaySound(string filename)",
        doc: Some("Plays a WAV sound file."),
    },
    // === Error/State Functions ===
    BuiltinFunction {
        name: "GetLastError",
        signature: "int GetLastError()",
        doc: Some("Returns the last error code and resets it to 0."),
    },
    BuiltinFunction {
        name: "ResetLastError",
        signature: "void ResetLastError()",
        doc: Some("Resets the last error code to 0."),
    },
    BuiltinFunction {
        name: "SetUserError",
        signature: "void SetUserError(ushort user_error)",
        doc: Some("Sets a user-defined error code (ERR_USER_ERROR_FIRST + user_error)."),
    },
    BuiltinFunction {
        name: "GetTickCount",
        signature: "uint GetTickCount()",
        doc: Some("Returns the number of milliseconds since system start (wraps at ~49 days)."),
    },
    BuiltinFunction {
        name: "GetTickCount64",
        signature: "ulong GetTickCount64()",
        doc: Some("Returns the 64-bit millisecond counter since system start."),
    },
    BuiltinFunction {
        name: "GetMicrosecondCount",
        signature: "ulong GetMicrosecondCount()",
        doc: Some("Returns the number of microseconds since MQL5 program start."),
    },
    BuiltinFunction {
        name: "TerminalClose",
        signature: "bool TerminalClose(int ret_code)",
        doc: Some("Commands the terminal to shut down."),
    },
    BuiltinFunction {
        name: "ExpertRemove",
        signature: "void ExpertRemove()",
        doc: Some("Stops the Expert Advisor and removes it from the chart."),
    },
    BuiltinFunction {
        name: "IsStopped",
        signature: "bool IsStopped()",
        doc: Some("Returns true if the MQL5 program has been commanded to stop."),
    },
    // === Info Functions ===
    BuiltinFunction {
        name: "Symbol",
        signature: "string Symbol()",
        doc: Some("Returns the symbol name of the current chart."),
    },
    BuiltinFunction {
        name: "Period",
        signature: "int Period()",
        doc: Some("Returns the timeframe of the current chart as an integer."),
    },
    BuiltinFunction {
        name: "Digits",
        signature: "int Digits()",
        doc: Some("Returns the number of decimal digits for the current symbol's price."),
    },
    BuiltinFunction {
        name: "Point",
        signature: "double Point()",
        doc: Some("Returns the point size of the current symbol (e.g. 0.00001 for 5-digit forex)."),
    },
    BuiltinFunction {
        name: "UninitializeReason",
        signature: "int UninitializeReason()",
        doc: Some("Returns the code of the reason for EA deinitialization."),
    },
    BuiltinFunction {
        name: "MQLInfoInteger",
        signature: "long MQLInfoInteger(int property_id)",
        doc: None,
    },
    BuiltinFunction {
        name: "MQLInfoString",
        signature: "string MQLInfoString(int property_id)",
        doc: None,
    },
    BuiltinFunction {
        name: "TerminalInfoInteger",
        signature: "long TerminalInfoInteger(ENUM_TERMINAL_INFO_INTEGER property_id)",
        doc: Some("Returns an integer property of the terminal."),
    },
    BuiltinFunction {
        name: "TerminalInfoDouble",
        signature: "double TerminalInfoDouble(ENUM_TERMINAL_INFO_DOUBLE property_id)",
        doc: None,
    },
    BuiltinFunction {
        name: "TerminalInfoString",
        signature: "string TerminalInfoString(ENUM_TERMINAL_INFO_STRING property_id)",
        doc: None,
    },
    BuiltinFunction {
        name: "SymbolInfoInteger",
        signature: "long SymbolInfoInteger(string name, ENUM_SYMBOL_INFO_INTEGER property_id)",
        doc: Some("Returns an integer property of the specified symbol."),
    },
    BuiltinFunction {
        name: "SymbolInfoDouble",
        signature: "double SymbolInfoDouble(string name, ENUM_SYMBOL_INFO_DOUBLE property_id)",
        doc: Some("Returns a double property of the specified symbol."),
    },
    BuiltinFunction {
        name: "SymbolInfoString",
        signature: "string SymbolInfoString(string name, ENUM_SYMBOL_INFO_STRING property_id)",
        doc: Some("Returns a string property of the specified symbol."),
    },
    BuiltinFunction {
        name: "SymbolInfoTick",
        signature: "bool SymbolInfoTick(string symbol, MqlTick& tick)",
        doc: Some("Returns the latest tick data for the symbol."),
    },
    BuiltinFunction {
        name: "SymbolsTotal",
        signature: "int SymbolsTotal(bool selected)",
        doc: Some("Returns the total number of symbols. If selected=true, only Market Watch symbols."),
    },
    BuiltinFunction {
        name: "SymbolName",
        signature: "string SymbolName(int pos, bool selected)",
        doc: Some("Returns the symbol name by its index."),
    },
    BuiltinFunction {
        name: "SymbolSelect",
        signature: "bool SymbolSelect(string name, bool select)",
        doc: Some("Selects/deselects a symbol in the Market Watch window."),
    },
    // === Market Book ===
    BuiltinFunction {
        name: "MarketBookAdd",
        signature: "bool MarketBookAdd(string symbol)",
        doc: Some("Subscribes to the market depth (DOM) for the specified symbol."),
    },
    BuiltinFunction {
        name: "MarketBookRelease",
        signature: "bool MarketBookRelease(string symbol)",
        doc: Some("Unsubscribes from the market depth for the specified symbol."),
    },
    BuiltinFunction {
        name: "MarketBookGet",
        signature: "bool MarketBookGet(string symbol, MqlBookInfo& book[])",
        doc: Some("Returns the current market depth data."),
    },
    // === Events ===
    BuiltinFunction {
        name: "EventSetTimer",
        signature: "bool EventSetTimer(int seconds)",
        doc: Some("Sets a timer with the specified interval in seconds. Triggers OnTimer()."),
    },
    BuiltinFunction {
        name: "EventSetMillisecondTimer",
        signature: "bool EventSetMillisecondTimer(int milliseconds)",
        doc: Some("Sets a high-resolution timer with the specified interval in milliseconds."),
    },
    BuiltinFunction {
        name: "EventKillTimer",
        signature: "void EventKillTimer()",
        doc: Some("Stops the timer."),
    },
    BuiltinFunction {
        name: "EventChartCustom",
        signature: "bool EventChartCustom(long chart_id, ushort custom_event_id, long lparam = 0, double dparam = 0.0, string sparam = \"\")",
        doc: Some("Sends a custom chart event. Received in OnChartEvent with id >= CHARTEVENT_CUSTOM."),
    },
    // === Indicator Buffers ===
    BuiltinFunction {
        name: "IndicatorSetInteger",
        signature: "bool IndicatorSetInteger(int property_id, int value)",
        doc: None,
    },
    BuiltinFunction {
        name: "IndicatorSetDouble",
        signature: "bool IndicatorSetDouble(int property_id, double value)",
        doc: None,
    },
    BuiltinFunction {
        name: "IndicatorSetString",
        signature: "bool IndicatorSetString(int property_id, string value)",
        doc: None,
    },
    BuiltinFunction {
        name: "SetIndexBuffer",
        signature: "bool SetIndexBuffer(int index, double& buffer[], int data_type = 0)",
        doc: Some("Binds a double array to an indicator buffer."),
    },
    BuiltinFunction {
        name: "IndicatorBuffers",
        signature: "void IndicatorBuffers(int count)",
        doc: None,
    },
    BuiltinFunction {
        name: "PlotIndexSetInteger",
        signature: "bool PlotIndexSetInteger(int plot_index, int property_id, int value)",
        doc: None,
    },
    BuiltinFunction {
        name: "PlotIndexSetDouble",
        signature: "bool PlotIndexSetDouble(int plot_index, int property_id, double value)",
        doc: None,
    },
    BuiltinFunction {
        name: "PlotIndexSetString",
        signature: "bool PlotIndexSetString(int plot_index, int property_id, string value)",
        doc: None,
    },
    BuiltinFunction {
        name: "PlotIndexGetInteger",
        signature: "int PlotIndexGetInteger(int plot_index, int property_id)",
        doc: None,
    },
    // === Testing ===
    BuiltinFunction {
        name: "IsTesting",
        signature: "bool IsTesting()",
        doc: Some("Returns true if the program is running in the Strategy Tester."),
    },
    BuiltinFunction {
        name: "IsOptimization",
        signature: "bool IsOptimization()",
        doc: Some("Returns true if the program is running in optimization mode."),
    },
    BuiltinFunction {
        name: "IsVisualMode",
        signature: "bool IsVisualMode()",
        doc: Some("Returns true if the Strategy Tester is running in visual mode."),
    },
    BuiltinFunction {
        name: "IsDemo",
        signature: "bool IsDemo()",
        doc: Some("Returns true if connected to a demo account."),
    },
    // === Resource Functions ===
    BuiltinFunction {
        name: "ResourceCreate",
        signature: "bool ResourceCreate(string resource_name, const uint& data[], uint img_width, uint img_height, uint data_xoffset, uint data_yoffset, uint data_width, ENUM_COLOR_FORMAT color_format)",
        doc: Some("Creates a dynamic graphical resource from a pixel array."),
    },
    BuiltinFunction {
        name: "ResourceFree",
        signature: "bool ResourceFree(string resource_name)",
        doc: Some("Frees a dynamic resource."),
    },
    BuiltinFunction {
        name: "ResourceReadImage",
        signature: "bool ResourceReadImage(string resource_name, uint& data[], uint& width, uint& height)",
        doc: Some("Reads a graphical resource into a pixel array."),
    },
    BuiltinFunction {
        name: "ResourceSave",
        signature: "bool ResourceSave(string resource_name, string file_name)",
        doc: None,
    },
    // === Cryptography ===
    BuiltinFunction {
        name: "CryptEncode",
        signature: "int CryptEncode(ENUM_CRYPT_METHOD method, const uchar& data[], const uchar& key[], uchar& result[])",
        doc: Some("Encodes/encrypts data using the specified method."),
    },
    BuiltinFunction {
        name: "CryptDecode",
        signature: "int CryptDecode(ENUM_CRYPT_METHOD method, const uchar& data[], const uchar& key[], uchar& result[])",
        doc: Some("Decodes/decrypts data using the specified method."),
    },
    // === Web/Network ===
    BuiltinFunction {
        name: "WebRequest",
        signature: "int WebRequest(string method, string url, string headers, int timeout, const char& data[], char& result[], string& result_headers)",
        doc: Some("Sends an HTTP request to a URL. The URL must be added in Tools > Options > Expert Advisors."),
    },
    // === Canvas/Text ===
    BuiltinFunction {
        name: "TextSetFont",
        signature: "bool TextSetFont(string name, int size = -120, uint flags = 0, int orientation = 0)",
        doc: Some("Sets the font for TextOut. Size in tenths of a point (negative = pixels)."),
    },
    BuiltinFunction {
        name: "TextOut",
        signature: "bool TextOut(string text, int x, int y, uint anchor, uint& data[], uint width, uint height, uint color, ENUM_COLOR_FORMAT color_format)",
        doc: Some("Draws text on a canvas (pixel array)."),
    },
    BuiltinFunction {
        name: "TextGetSize",
        signature: "bool TextGetSize(string text, uint& width, uint& height)",
        doc: Some("Returns the width and height of the text with the current font."),
    },
    // === Folder Functions ===
    BuiltinFunction {
        name: "FolderCreate",
        signature: "bool FolderCreate(string folder_name, int common_flag = 0)",
        doc: Some("Creates a folder in the terminal's file sandbox."),
    },
    BuiltinFunction {
        name: "FolderDelete",
        signature: "bool FolderDelete(string folder_name, int common_flag = 0)",
        doc: Some("Deletes a folder from the terminal's file sandbox."),
    },
    BuiltinFunction {
        name: "FolderClean",
        signature: "bool FolderClean(string folder_name, int common_flag = 0)",
        doc: Some("Deletes all files in a folder."),
    },
    // === Memory ===
    BuiltinFunction {
        name: "ZeroMemory",
        signature: "void ZeroMemory(void& variable)",
        doc: Some("Fills a variable or array with zeros."),
    },
    // === Checkup / Struct Conversion ===
    BuiltinFunction {
        name: "CharArrayToStruct",
        signature: "int CharArrayToStruct(void& struct_object, uchar& array[], int start_pos = 0)",
        doc: None,
    },
    BuiltinFunction {
        name: "StructToCharArray",
        signature: "int StructToCharArray(const void& struct_object, uchar& array[], int start_pos = 0)",
        doc: None,
    },
    // === Pointer ===
    BuiltinFunction {
        name: "GetPointer",
        signature: "void* GetPointer(void& object)",
        doc: Some("Returns a pointer to the specified object."),
    },
    // === Symbol (extended) ===
    BuiltinFunction {
        name: "SymbolExist",
        signature: "bool SymbolExist(string name, bool& is_custom)",
        doc: Some("Checks if a symbol exists. Sets is_custom to true if it is a custom symbol."),
    },
    BuiltinFunction {
        name: "SymbolInfoSessionTrade",
        signature: "int SymbolInfoSessionTrade(string name, ENUM_DAY_OF_WEEK day_of_week, uint session_index, datetime& from, datetime& to)",
        doc: None,
    },
    BuiltinFunction {
        name: "SymbolInfoSessionQuote",
        signature: "int SymbolInfoSessionQuote(string name, ENUM_DAY_OF_WEEK day_of_week, uint session_index, datetime& from, datetime& to)",
        doc: None,
    },
    // === Custom Symbols ===
    BuiltinFunction {
        name: "CustomSymbolCreate",
        signature: "bool CustomSymbolCreate(string symbol_name, string symbol_path = \"\", string symbol_origin = \"\")",
        doc: None,
    },
    BuiltinFunction {
        name: "CustomSymbolSetString",
        signature: "bool CustomSymbolSetString(string symbol_name, ENUM_SYMBOL_INFO_STRING property_id, string property_value)",
        doc: None,
    },
    BuiltinFunction {
        name: "CustomTicksAdd",
        signature: "int CustomTicksAdd(string symbol, MqlTick& ticks[], uint count)",
        doc: None,
    },
    // === Missing common functions ===
    BuiltinFunction { name: "DebugBreak", signature: "void DebugBreak()", doc: Some("Breaks program execution in the debugger.") },
    BuiltinFunction { name: "CheckPointer", signature: "ENUM_POINTER_TYPE CheckPointer(void* anyobj)", doc: Some("Returns the type of the pointer.") },
    BuiltinFunction { name: "ArrayCompare", signature: "int ArrayCompare(const void& array1[], const void& array2[], int start1 = 0, int start2 = 0, int count = WHOLE_ARRAY)", doc: Some("Compares two arrays element by element.") },
    BuiltinFunction { name: "ArrayGetAsSeries", signature: "bool ArrayGetAsSeries(const void& array[])", doc: Some("Returns whether the array is set as timeseries.") },
    // === Database Functions ===
    BuiltinFunction { name: "DatabaseOpen", signature: "int DatabaseOpen(string filename, uint flags)", doc: Some("Opens or creates a database.") },
    BuiltinFunction { name: "DatabaseClose", signature: "void DatabaseClose(int database)", doc: Some("Closes a database.") },
    BuiltinFunction { name: "DatabasePrepare", signature: "int DatabasePrepare(int database, string sql)", doc: Some("Prepares an SQL statement for execution.") },
    BuiltinFunction { name: "DatabaseRead", signature: "bool DatabaseRead(int request)", doc: Some("Reads the next row from the SQL query result.") },
    BuiltinFunction { name: "DatabaseFinalize", signature: "void DatabaseFinalize(int request)", doc: Some("Finalizes an SQL query.") },
    BuiltinFunction { name: "DatabaseExecute", signature: "bool DatabaseExecute(int database, string sql)", doc: Some("Executes an SQL statement that does not return data.") },
    BuiltinFunction { name: "DatabaseColumnName", signature: "string DatabaseColumnName(int request, int column)", doc: None },
    BuiltinFunction { name: "DatabaseColumnInteger", signature: "long DatabaseColumnInteger(int request, int column)", doc: None },
    BuiltinFunction { name: "DatabaseColumnDouble", signature: "double DatabaseColumnDouble(int request, int column)", doc: None },
    BuiltinFunction { name: "DatabaseColumnText", signature: "string DatabaseColumnText(int request, int column)", doc: None },
    BuiltinFunction { name: "DatabaseColumnsCount", signature: "int DatabaseColumnsCount(int request)", doc: None },
    BuiltinFunction { name: "DatabaseTableExists", signature: "bool DatabaseTableExists(int database, string table)", doc: None },
    BuiltinFunction { name: "DatabaseTransactionBegin", signature: "bool DatabaseTransactionBegin(int database)", doc: None },
    BuiltinFunction { name: "DatabaseTransactionCommit", signature: "bool DatabaseTransactionCommit(int database)", doc: None },
    BuiltinFunction { name: "DatabaseTransactionRollback", signature: "bool DatabaseTransactionRollback(int database)", doc: None },
    // === Socket Functions ===
    BuiltinFunction { name: "SocketCreate", signature: "int SocketCreate()", doc: Some("Creates a socket.") },
    BuiltinFunction { name: "SocketConnect", signature: "bool SocketConnect(int socket, string server, uint port, uint timeout)", doc: None },
    BuiltinFunction { name: "SocketClose", signature: "bool SocketClose(int socket)", doc: None },
    BuiltinFunction { name: "SocketSend", signature: "int SocketSend(int socket, const uchar& buffer[], int buffer_len)", doc: None },
    BuiltinFunction { name: "SocketRead", signature: "int SocketRead(int socket, uchar& buffer[], int buffer_len, uint timeout)", doc: None },
    BuiltinFunction { name: "SocketIsConnected", signature: "bool SocketIsConnected(int socket)", doc: None },
    BuiltinFunction { name: "SocketTlsHandshake", signature: "bool SocketTlsHandshake(int socket, string host)", doc: None },
    BuiltinFunction { name: "SocketTlsRead", signature: "int SocketTlsRead(int socket, uchar& buffer[], int buffer_len)", doc: None },
    BuiltinFunction { name: "SocketTlsSend", signature: "int SocketTlsSend(int socket, const uchar& buffer[], int buffer_len)", doc: None },
    // === CustomRates / CustomTicks ===
    BuiltinFunction { name: "CustomRatesDelete", signature: "int CustomRatesDelete(string symbol, datetime from, datetime to)", doc: None },
    BuiltinFunction { name: "CustomRatesReplace", signature: "int CustomRatesReplace(string symbol, datetime from, datetime to, const MqlRates& rates[], uint count)", doc: None },
    BuiltinFunction { name: "CustomRatesUpdate", signature: "int CustomRatesUpdate(string symbol, const MqlRates& rates[], uint count)", doc: None },
    BuiltinFunction { name: "CustomSymbolDelete", signature: "bool CustomSymbolDelete(string symbol_name)", doc: None },
    BuiltinFunction { name: "CustomSymbolSetInteger", signature: "bool CustomSymbolSetInteger(string symbol_name, ENUM_SYMBOL_INFO_INTEGER property_id, long property_value)", doc: None },
    BuiltinFunction { name: "CustomSymbolSetDouble", signature: "bool CustomSymbolSetDouble(string symbol_name, ENUM_SYMBOL_INFO_DOUBLE property_id, double property_value)", doc: None },
    BuiltinFunction { name: "CustomTicksDelete", signature: "int CustomTicksDelete(string symbol, long from_msc, long to_msc)", doc: None },
    BuiltinFunction { name: "CustomTicksReplace", signature: "int CustomTicksReplace(string symbol, long from_msc, long to_msc, const MqlTick& ticks[], uint count)", doc: None },
    BuiltinFunction { name: "CustomBookAdd", signature: "bool CustomBookAdd(string symbol, const MqlBookInfo& books[], uint count)", doc: None },
    // === Matrix/Vector ===
    BuiltinFunction { name: "MathAtan2", signature: "double MathAtan2(double y, double x)", doc: None },
    BuiltinFunction { name: "MathIsValidNumber", signature: "bool MathIsValidNumber(double number)", doc: None },
    BuiltinFunction { name: "MathSwap", signature: "ushort MathSwap(ushort value)", doc: None },
];

// =============================================================================
// BUILTIN ENUMS
// =============================================================================
pub static BUILTIN_ENUMS: &[BuiltinEnum] = &[
    BuiltinEnum {
        name: "ENUM_TIMEFRAMES",
        values: &[
            "PERIOD_CURRENT", "PERIOD_M1", "PERIOD_M2", "PERIOD_M3", "PERIOD_M4", "PERIOD_M5",
            "PERIOD_M6", "PERIOD_M10", "PERIOD_M12", "PERIOD_M15", "PERIOD_M20", "PERIOD_M30",
            "PERIOD_H1", "PERIOD_H2", "PERIOD_H3", "PERIOD_H4",
            "PERIOD_H6", "PERIOD_H8", "PERIOD_H12",
            "PERIOD_D1", "PERIOD_W1", "PERIOD_MN1",
        ],
    },
    BuiltinEnum {
        name: "ENUM_CHART_PROPERTY_INTEGER",
        values: &[
            "CHART_SHOW", "CHART_IS_OBJECT", "CHART_BRING_TO_TOP", "CHART_CONTEXT_MENU",
            "CHART_CROSSHAIR_TOOL", "CHART_MOUSE_SCROLL", "CHART_EVENT_MOUSE_WHEEL",
            "CHART_EVENT_MOUSE_MOVE", "CHART_EVENT_OBJECT_CREATE", "CHART_EVENT_OBJECT_DELETE",
            "CHART_MODE", "CHART_FOREGROUND", "CHART_SHIFT", "CHART_AUTOSCROLL",
            "CHART_KEYBOARD_CONTROL", "CHART_QUICK_NAVIGATION", "CHART_SCALE",
            "CHART_SCALEFIX", "CHART_SCALEFIX_11", "CHART_SCALE_PT_PER_BAR",
            "CHART_SHOW_TICKER", "CHART_SHOW_OHLC", "CHART_SHOW_BID_LINE",
            "CHART_SHOW_ASK_LINE", "CHART_SHOW_LAST_LINE", "CHART_SHOW_PERIOD_SEP",
            "CHART_SHOW_GRID", "CHART_SHOW_VOLUMES", "CHART_SHOW_OBJECT_DESCR",
            "CHART_VISIBLE_BARS", "CHART_WINDOWS_TOTAL", "CHART_WINDOW_IS_VISIBLE",
            "CHART_WINDOW_HANDLE", "CHART_WINDOW_YDISTANCE", "CHART_FIRST_VISIBLE_BAR",
            "CHART_WIDTH_IN_BARS", "CHART_WIDTH_IN_PIXELS", "CHART_HEIGHT_IN_PIXELS",
            "CHART_COLOR_BACKGROUND", "CHART_COLOR_FOREGROUND", "CHART_COLOR_GRID",
            "CHART_COLOR_VOLUME", "CHART_COLOR_CHART_UP", "CHART_COLOR_CHART_DOWN",
            "CHART_COLOR_CHART_LINE", "CHART_COLOR_CANDLE_BULL", "CHART_COLOR_CANDLE_BEAR",
            "CHART_COLOR_BID", "CHART_COLOR_ASK", "CHART_COLOR_LAST", "CHART_COLOR_STOP_LEVEL",
            "CHART_SHOW_TRADE_LEVELS", "CHART_DRAG_TRADE_LEVELS", "CHART_SHOW_DATE_SCALE",
            "CHART_SHOW_PRICE_SCALE", "CHART_SHOW_ONE_CLICK", "CHART_IS_MAXIMIZED",
            "CHART_IS_MINIMIZED", "CHART_IS_DOCKED", "CHART_FLOAT_LEFT", "CHART_FLOAT_TOP",
            "CHART_FLOAT_RIGHT", "CHART_FLOAT_BOTTOM",
        ],
    },
    BuiltinEnum {
        name: "ENUM_CHART_PROPERTY_DOUBLE",
        values: &[
            "CHART_SHIFT_SIZE", "CHART_FIXED_POSITION", "CHART_FIXED_MAX", "CHART_FIXED_MIN",
            "CHART_POINTS_PER_BAR", "CHART_PRICE_MIN", "CHART_PRICE_MAX",
        ],
    },
    BuiltinEnum {
        name: "ENUM_CHART_PROPERTY_STRING",
        values: &[
            "CHART_COMMENT", "CHART_EXPERT_NAME", "CHART_SCRIPT_NAME",
        ],
    },
    BuiltinEnum {
        name: "ENUM_OBJECT_TYPE",
        values: &[
            "OBJ_VLINE", "OBJ_HLINE", "OBJ_TREND", "OBJ_TRENDBYANGLE", "OBJ_CYCLES",
            "OBJ_ARROWED_LINE", "OBJ_CHANNEL", "OBJ_STDDEVCHANNEL", "OBJ_REGRESSION",
            "OBJ_PITCHFORK", "OBJ_GANNLINE", "OBJ_GANNFAN", "OBJ_GANNGRID",
            "OBJ_FIBO", "OBJ_FIBOTIMES", "OBJ_FIBOFAN", "OBJ_FIBOARC", "OBJ_FIBOCHANNEL",
            "OBJ_EXPANSION", "OBJ_ELLIOTWAVE5", "OBJ_ELLIOTWAVE3",
            "OBJ_RECTANGLE", "OBJ_TRIANGLE", "OBJ_ELLIPSE",
            "OBJ_ARROW_THUMB_UP", "OBJ_ARROW_THUMB_DOWN", "OBJ_ARROW_UP", "OBJ_ARROW_DOWN",
            "OBJ_ARROW_STOP", "OBJ_ARROW_CHECK", "OBJ_ARROW_LEFT_PRICE", "OBJ_ARROW_RIGHT_PRICE",
            "OBJ_ARROW_BUY", "OBJ_ARROW_SELL",
            "OBJ_ARROW", "OBJ_TEXT", "OBJ_LABEL", "OBJ_BUTTON",
            "OBJ_CHART", "OBJ_BITMAP", "OBJ_BITMAP_LABEL", "OBJ_EDIT", "OBJ_EVENT", "OBJ_RECTANGLE_LABEL",
        ],
    },
    BuiltinEnum {
        name: "ENUM_OBJECT_PROPERTY_INTEGER",
        values: &[
            "OBJPROP_COLOR", "OBJPROP_STYLE", "OBJPROP_WIDTH", "OBJPROP_BACK", "OBJPROP_ZORDER",
            "OBJPROP_FILL", "OBJPROP_HIDDEN", "OBJPROP_SELECTED", "OBJPROP_READONLY",
            "OBJPROP_TYPE", "OBJPROP_TIME", "OBJPROP_SELECTABLE", "OBJPROP_CREATETIME",
            "OBJPROP_LEVELS", "OBJPROP_LEVELCOLOR", "OBJPROP_LEVELSTYLE", "OBJPROP_LEVELWIDTH",
            "OBJPROP_ALIGN", "OBJPROP_FONTSIZE", "OBJPROP_RAY_LEFT", "OBJPROP_RAY_RIGHT",
            "OBJPROP_RAY", "OBJPROP_ELLIPSE", "OBJPROP_ARROWCODE", "OBJPROP_TIMEFRAMES",
            "OBJPROP_ANCHOR", "OBJPROP_XDISTANCE", "OBJPROP_YDISTANCE", "OBJPROP_DIRECTION",
            "OBJPROP_DEGREE", "OBJPROP_DRAWLINES", "OBJPROP_STATE", "OBJPROP_CHART_ID",
            "OBJPROP_XSIZE", "OBJPROP_YSIZE", "OBJPROP_XOFFSET", "OBJPROP_YOFFSET",
            "OBJPROP_PERIOD", "OBJPROP_DATE_SCALE", "OBJPROP_PRICE_SCALE", "OBJPROP_CHART_SCALE",
            "OBJPROP_BGCOLOR", "OBJPROP_CORNER", "OBJPROP_BORDER_TYPE", "OBJPROP_BORDER_COLOR",
        ],
    },
    BuiltinEnum {
        name: "ENUM_OBJECT_PROPERTY_DOUBLE",
        values: &[
            "OBJPROP_PRICE", "OBJPROP_LEVELVALUE", "OBJPROP_SCALE", "OBJPROP_ANGLE",
            "OBJPROP_DEVIATION",
        ],
    },
    BuiltinEnum {
        name: "ENUM_OBJECT_PROPERTY_STRING",
        values: &[
            "OBJPROP_NAME", "OBJPROP_TEXT", "OBJPROP_TOOLTIP", "OBJPROP_LEVELTEXT",
            "OBJPROP_FONT", "OBJPROP_BMPFILE", "OBJPROP_SYMBOL",
        ],
    },
    BuiltinEnum {
        name: "ENUM_ORDER_TYPE",
        values: &[
            "ORDER_TYPE_BUY", "ORDER_TYPE_SELL",
            "ORDER_TYPE_BUY_LIMIT", "ORDER_TYPE_SELL_LIMIT",
            "ORDER_TYPE_BUY_STOP", "ORDER_TYPE_SELL_STOP",
            "ORDER_TYPE_BUY_STOP_LIMIT", "ORDER_TYPE_SELL_STOP_LIMIT",
            "ORDER_TYPE_CLOSE_BY",
        ],
    },
    BuiltinEnum {
        name: "ENUM_ORDER_TYPE_FILLING",
        values: &[
            "ORDER_FILLING_FOK", "ORDER_FILLING_IOC", "ORDER_FILLING_RETURN", "ORDER_FILLING_BOC",
        ],
    },
    BuiltinEnum {
        name: "ENUM_ORDER_TYPE_TIME",
        values: &[
            "ORDER_TIME_GTC", "ORDER_TIME_DAY", "ORDER_TIME_SPECIFIED", "ORDER_TIME_SPECIFIED_DAY",
        ],
    },
    BuiltinEnum {
        name: "ENUM_ORDER_PROPERTY_INTEGER",
        values: &[
            "ORDER_TICKET", "ORDER_TIME_SETUP", "ORDER_TYPE", "ORDER_STATE", "ORDER_TIME_EXPIRATION",
            "ORDER_TIME_DONE", "ORDER_TIME_SETUP_MSC", "ORDER_TIME_DONE_MSC",
            "ORDER_TYPE_FILLING", "ORDER_TYPE_TIME", "ORDER_MAGIC", "ORDER_REASON",
            "ORDER_POSITION_ID", "ORDER_POSITION_BY_ID",
        ],
    },
    BuiltinEnum {
        name: "ENUM_ORDER_PROPERTY_DOUBLE",
        values: &[
            "ORDER_VOLUME_INITIAL", "ORDER_VOLUME_CURRENT", "ORDER_PRICE_OPEN",
            "ORDER_SL", "ORDER_TP", "ORDER_PRICE_CURRENT", "ORDER_PRICE_STOPLIMIT",
        ],
    },
    BuiltinEnum {
        name: "ENUM_ORDER_PROPERTY_STRING",
        values: &[
            "ORDER_SYMBOL", "ORDER_COMMENT", "ORDER_EXTERNAL_ID",
        ],
    },
    BuiltinEnum {
        name: "ENUM_POSITION_TYPE",
        values: &[
            "POSITION_TYPE_BUY", "POSITION_TYPE_SELL",
        ],
    },
    BuiltinEnum {
        name: "ENUM_POSITION_PROPERTY_INTEGER",
        values: &[
            "POSITION_TICKET", "POSITION_TIME", "POSITION_TIME_MSC", "POSITION_TIME_UPDATE",
            "POSITION_TIME_UPDATE_MSC", "POSITION_TYPE", "POSITION_MAGIC", "POSITION_IDENTIFIER",
            "POSITION_REASON",
        ],
    },
    BuiltinEnum {
        name: "ENUM_POSITION_PROPERTY_DOUBLE",
        values: &[
            "POSITION_VOLUME", "POSITION_PRICE_OPEN", "POSITION_SL", "POSITION_TP",
            "POSITION_PRICE_CURRENT", "POSITION_SWAP", "POSITION_PROFIT",
        ],
    },
    BuiltinEnum {
        name: "ENUM_POSITION_PROPERTY_STRING",
        values: &[
            "POSITION_SYMBOL", "POSITION_COMMENT", "POSITION_EXTERNAL_ID",
        ],
    },
    BuiltinEnum {
        name: "ENUM_DEAL_TYPE",
        values: &[
            "DEAL_TYPE_BUY", "DEAL_TYPE_SELL", "DEAL_TYPE_BALANCE", "DEAL_TYPE_CREDIT",
            "DEAL_TYPE_CHARGE", "DEAL_TYPE_CORRECTION", "DEAL_TYPE_BONUS",
            "DEAL_TYPE_COMMISSION", "DEAL_TYPE_COMMISSION_DAILY", "DEAL_TYPE_COMMISSION_MONTHLY",
            "DEAL_TYPE_COMMISSION_AGENT_DAILY", "DEAL_TYPE_COMMISSION_AGENT_MONTHLY",
            "DEAL_TYPE_INTEREST", "DEAL_TYPE_BUY_CANCELED", "DEAL_TYPE_SELL_CANCELED",
            "DEAL_DIVIDEND", "DEAL_DIVIDEND_FRANKED", "DEAL_TAX",
        ],
    },
    BuiltinEnum {
        name: "ENUM_DEAL_PROPERTY_INTEGER",
        values: &[
            "DEAL_TICKET", "DEAL_ORDER", "DEAL_TIME", "DEAL_TIME_MSC", "DEAL_TYPE",
            "DEAL_ENTRY", "DEAL_MAGIC", "DEAL_REASON", "DEAL_POSITION_ID",
        ],
    },
    BuiltinEnum {
        name: "ENUM_DEAL_PROPERTY_DOUBLE",
        values: &[
            "DEAL_VOLUME", "DEAL_PRICE", "DEAL_COMMISSION", "DEAL_SWAP", "DEAL_PROFIT",
            "DEAL_FEE", "DEAL_SL", "DEAL_TP",
        ],
    },
    BuiltinEnum {
        name: "ENUM_DEAL_PROPERTY_STRING",
        values: &[
            "DEAL_SYMBOL", "DEAL_COMMENT", "DEAL_EXTERNAL_ID",
        ],
    },
    BuiltinEnum {
        name: "ENUM_DEAL_ENTRY",
        values: &[
            "DEAL_ENTRY_IN", "DEAL_ENTRY_OUT", "DEAL_ENTRY_INOUT", "DEAL_ENTRY_OUT_BY",
        ],
    },
    BuiltinEnum {
        name: "ENUM_TRADE_REQUEST_ACTIONS",
        values: &[
            "TRADE_ACTION_DEAL", "TRADE_ACTION_PENDING", "TRADE_ACTION_SLTP",
            "TRADE_ACTION_MODIFY", "TRADE_ACTION_REMOVE", "TRADE_ACTION_CLOSE_BY",
        ],
    },
    BuiltinEnum {
        name: "ENUM_SYMBOL_INFO_INTEGER",
        values: &[
            "SYMBOL_CUSTOM", "SYMBOL_CHART_MODE", "SYMBOL_SELECT", "SYMBOL_VISIBLE",
            "SYMBOL_SESSION_DEALS", "SYMBOL_SESSION_BUY_ORDERS", "SYMBOL_SESSION_SELL_ORDERS",
            "SYMBOL_VOLUME", "SYMBOL_VOLUMEHIGH", "SYMBOL_VOLUMELOW",
            "SYMBOL_TIME", "SYMBOL_TIME_MSC", "SYMBOL_DIGITS", "SYMBOL_SPREAD_FLOAT", "SYMBOL_SPREAD",
            "SYMBOL_TICKS_BOOKDEPTH", "SYMBOL_TRADE_CALC_MODE", "SYMBOL_TRADE_MODE",
            "SYMBOL_START_TIME", "SYMBOL_EXPIRATION_TIME", "SYMBOL_TRADE_STOPS_LEVEL",
            "SYMBOL_TRADE_FREEZE_LEVEL", "SYMBOL_TRADE_EXEMODE", "SYMBOL_SWAP_MODE",
            "SYMBOL_SWAP_ROLLOVER3DAYS", "SYMBOL_MARGIN_HEDGED_USE_LEG", "SYMBOL_EXPIRATION_MODE",
            "SYMBOL_FILLING_MODE", "SYMBOL_ORDER_MODE", "SYMBOL_ORDER_GTC_MODE",
            "SYMBOL_OPTION_MODE", "SYMBOL_OPTION_RIGHT",
        ],
    },
    BuiltinEnum {
        name: "ENUM_SYMBOL_INFO_DOUBLE",
        values: &[
            "SYMBOL_BID", "SYMBOL_BIDHIGH", "SYMBOL_BIDLOW", "SYMBOL_ASK", "SYMBOL_ASKHIGH", "SYMBOL_ASKLOW",
            "SYMBOL_LAST", "SYMBOL_LASTHIGH", "SYMBOL_LASTLOW", "SYMBOL_VOLUME_REAL",
            "SYMBOL_VOLUMEHIGH_REAL", "SYMBOL_VOLUMELOW_REAL",
            "SYMBOL_POINT", "SYMBOL_TRADE_TICK_VALUE", "SYMBOL_TRADE_TICK_VALUE_PROFIT",
            "SYMBOL_TRADE_TICK_VALUE_LOSS", "SYMBOL_TRADE_TICK_SIZE",
            "SYMBOL_TRADE_CONTRACT_SIZE", "SYMBOL_TRADE_ACCRUED_INTEREST",
            "SYMBOL_TRADE_FACE_VALUE", "SYMBOL_TRADE_LIQUIDITY_RATE",
            "SYMBOL_VOLUME_MIN", "SYMBOL_VOLUME_MAX", "SYMBOL_VOLUME_STEP",
            "SYMBOL_VOLUME_LIMIT", "SYMBOL_SWAP_LONG", "SYMBOL_SWAP_SHORT",
            "SYMBOL_MARGIN_INITIAL", "SYMBOL_MARGIN_MAINTENANCE", "SYMBOL_SESSION_VOLUME",
            "SYMBOL_SESSION_TURNOVER", "SYMBOL_SESSION_INTEREST", "SYMBOL_SESSION_BUY_ORDERS_VOLUME",
            "SYMBOL_SESSION_SELL_ORDERS_VOLUME", "SYMBOL_SESSION_OPEN", "SYMBOL_SESSION_CLOSE",
            "SYMBOL_SESSION_AW", "SYMBOL_SESSION_PRICE_SETTLEMENT", "SYMBOL_SESSION_PRICE_LIMIT_MIN",
            "SYMBOL_SESSION_PRICE_LIMIT_MAX", "SYMBOL_MARGIN_HEDGED",
            "SYMBOL_PRICE_CHANGE", "SYMBOL_PRICE_VOLATILITY", "SYMBOL_PRICE_THEORETICAL",
            "SYMBOL_PRICE_GREEKS_DELTA", "SYMBOL_PRICE_GREEKS_THETA", "SYMBOL_PRICE_GREEKS_GAMMA",
            "SYMBOL_PRICE_GREEKS_VEGA", "SYMBOL_PRICE_GREEKS_RHO", "SYMBOL_PRICE_GREEKS_OMEGA",
            "SYMBOL_PRICE_SENSITIVITY",
        ],
    },
    BuiltinEnum {
        name: "ENUM_SYMBOL_INFO_STRING",
        values: &[
            "SYMBOL_CURRENCY_BASE", "SYMBOL_CURRENCY_PROFIT", "SYMBOL_CURRENCY_MARGIN",
            "SYMBOL_BANK", "SYMBOL_DESCRIPTION", "SYMBOL_EXCHANGE", "SYMBOL_FORMULA",
            "SYMBOL_ISIN", "SYMBOL_NAME", "SYMBOL_PAGE", "SYMBOL_PATH",
        ],
    },
    BuiltinEnum {
        name: "ENUM_ACCOUNT_INFO_INTEGER",
        values: &[
            "ACCOUNT_LOGIN", "ACCOUNT_TRADE_MODE", "ACCOUNT_LEVERAGE", "ACCOUNT_LIMIT_ORDERS",
            "ACCOUNT_MARGIN_SO_MODE", "ACCOUNT_TRADE_ALLOWED", "ACCOUNT_TRADE_EXPERT",
            "ACCOUNT_MARGIN_MODE", "ACCOUNT_CURRENCY_DIGITS", "ACCOUNT_FIFO_CLOSE",
        ],
    },
    BuiltinEnum {
        name: "ENUM_ACCOUNT_INFO_DOUBLE",
        values: &[
            "ACCOUNT_BALANCE", "ACCOUNT_CREDIT", "ACCOUNT_PROFIT", "ACCOUNT_EQUITY",
            "ACCOUNT_MARGIN", "ACCOUNT_MARGIN_FREE", "ACCOUNT_MARGIN_LEVEL",
            "ACCOUNT_MARGIN_SO_CALL", "ACCOUNT_MARGIN_SO_SO", "ACCOUNT_MARGIN_INITIAL",
            "ACCOUNT_MARGIN_MAINTENANCE", "ACCOUNT_ASSETS", "ACCOUNT_LIABILITIES",
            "ACCOUNT_COMMISSION_BLOCKED",
        ],
    },
    BuiltinEnum {
        name: "ENUM_ACCOUNT_INFO_STRING",
        values: &[
            "ACCOUNT_NAME", "ACCOUNT_SERVER", "ACCOUNT_CURRENCY", "ACCOUNT_COMPANY",
        ],
    },
    BuiltinEnum {
        name: "ENUM_TERMINAL_INFO_INTEGER",
        values: &[
            "TERMINAL_BUILD", "TERMINAL_COMMUNITY_ACCOUNT", "TERMINAL_COMMUNITY_CONNECTION",
            "TERMINAL_CONNECTED", "TERMINAL_DLLS_ALLOWED", "TERMINAL_TRADE_ALLOWED",
            "TERMINAL_EMAIL_ENABLED", "TERMINAL_FTP_ENABLED", "TERMINAL_NOTIFICATIONS_ENABLED",
            "TERMINAL_MAXBARS", "TERMINAL_MQID", "TERMINAL_CODEPAGE", "TERMINAL_CPU_CORES",
            "TERMINAL_DISK_SPACE", "TERMINAL_MEMORY_PHYSICAL", "TERMINAL_MEMORY_TOTAL",
            "TERMINAL_MEMORY_AVAILABLE", "TERMINAL_MEMORY_USED", "TERMINAL_SCREEN_DPI",
            "TERMINAL_PING_LAST", "TERMINAL_KEYSTATE_LEFT", "TERMINAL_KEYSTATE_UP",
            "TERMINAL_KEYSTATE_RIGHT", "TERMINAL_KEYSTATE_DOWN", "TERMINAL_KEYSTATE_SHIFT",
            "TERMINAL_KEYSTATE_CONTROL", "TERMINAL_KEYSTATE_MENU", "TERMINAL_KEYSTATE_CAPSLOCK",
            "TERMINAL_KEYSTATE_NUMLOCK", "TERMINAL_KEYSTATE_SCRLOCK", "TERMINAL_KEYSTATE_ENTER",
            "TERMINAL_KEYSTATE_INSERT", "TERMINAL_KEYSTATE_DELETE", "TERMINAL_KEYSTATE_HOME",
            "TERMINAL_KEYSTATE_END", "TERMINAL_KEYSTATE_TAB", "TERMINAL_KEYSTATE_PAGEUP",
            "TERMINAL_KEYSTATE_PAGEDOWN", "TERMINAL_KEYSTATE_ESCAPE",
        ],
    },
    BuiltinEnum {
        name: "ENUM_TERMINAL_INFO_DOUBLE",
        values: &[
            "TERMINAL_COMMUNITY_BALANCE", "TERMINAL_RETRANSMISSION",
        ],
    },
    BuiltinEnum {
        name: "ENUM_TERMINAL_INFO_STRING",
        values: &[
            "TERMINAL_LANGUAGE", "TERMINAL_COMPANY", "TERMINAL_NAME", "TERMINAL_PATH",
            "TERMINAL_DATA_PATH", "TERMINAL_COMMONDATA_PATH",
        ],
    },
    BuiltinEnum {
        name: "ENUM_SERIES_INFO_INTEGER",
        values: &[
            "SERIES_BARS_COUNT", "SERIES_FIRSTDATE", "SERIES_LASTBAR_DATE",
            "SERIES_SERVER_FIRSTDATE",
        ],
    },
    BuiltinEnum {
        name: "ENUM_MA_METHOD",
        values: &[
            "MODE_SMA", "MODE_EMA", "MODE_SMMA", "MODE_LWMA",
        ],
    },
    BuiltinEnum {
        name: "ENUM_APPLIED_PRICE",
        values: &[
            "PRICE_CLOSE", "PRICE_OPEN", "PRICE_HIGH", "PRICE_LOW",
            "PRICE_MEDIAN", "PRICE_TYPICAL", "PRICE_WEIGHTED",
        ],
    },
    BuiltinEnum {
        name: "ENUM_STO_PRICE",
        values: &[
            "STO_LOWHIGH", "STO_CLOSECLOSE",
        ],
    },
    BuiltinEnum {
        name: "ENUM_APPLIED_VOLUME",
        values: &[
            "VOLUME_TICK", "VOLUME_REAL",
        ],
    },
    BuiltinEnum {
        name: "ENUM_DRAW_TYPE",
        values: &[
            "DRAW_NONE", "DRAW_LINE", "DRAW_SECTION", "DRAW_HISTOGRAM", "DRAW_HISTOGRAM2",
            "DRAW_ARROW", "DRAW_ZIGZAG", "DRAW_FILLING", "DRAW_BARS", "DRAW_CANDLES",
            "DRAW_COLOR_LINE", "DRAW_COLOR_SECTION", "DRAW_COLOR_HISTOGRAM",
            "DRAW_COLOR_HISTOGRAM2", "DRAW_COLOR_ARROW", "DRAW_COLOR_ZIGZAG",
            "DRAW_COLOR_BARS", "DRAW_COLOR_CANDLES",
        ],
    },
    BuiltinEnum {
        name: "ENUM_INDICATOR_PROPERTY_INTEGER",
        values: &[
            "INDICATOR_DIGITS", "INDICATOR_HEIGHT", "INDICATOR_LEVELS",
            "INDICATOR_LEVELCOLOR", "INDICATOR_LEVELSTYLE", "INDICATOR_LEVELWIDTH",
        ],
    },
    BuiltinEnum {
        name: "ENUM_INDICATOR_PROPERTY_DOUBLE",
        values: &[
            "INDICATOR_MINIMUM", "INDICATOR_MAXIMUM", "INDICATOR_LEVELVALUE",
        ],
    },
    BuiltinEnum {
        name: "ENUM_INDICATOR_PROPERTY_STRING",
        values: &[
            "INDICATOR_SHORTNAME", "INDICATOR_LEVELTEXT",
        ],
    },
    BuiltinEnum {
        name: "ENUM_LINE_STYLE",
        values: &[
            "STYLE_SOLID", "STYLE_DASH", "STYLE_DOT", "STYLE_DASHDOT", "STYLE_DASHDOTDOT",
        ],
    },
    BuiltinEnum {
        name: "ENUM_ANCHOR_POINT",
        values: &[
            "ANCHOR_LEFT_UPPER", "ANCHOR_LEFT", "ANCHOR_LEFT_LOWER",
            "ANCHOR_LOWER", "ANCHOR_RIGHT_LOWER", "ANCHOR_RIGHT",
            "ANCHOR_RIGHT_UPPER", "ANCHOR_UPPER", "ANCHOR_CENTER",
            "ANCHOR_TOP", "ANCHOR_BOTTOM",
        ],
    },
    BuiltinEnum {
        name: "ENUM_BASE_CORNER",
        values: &[
            "CORNER_LEFT_UPPER", "CORNER_LEFT_LOWER", "CORNER_RIGHT_LOWER", "CORNER_RIGHT_UPPER",
        ],
    },
    BuiltinEnum {
        name: "ENUM_BORDER_TYPE",
        values: &[
            "BORDER_FLAT", "BORDER_RAISED", "BORDER_SUNKEN",
        ],
    },
    BuiltinEnum {
        name: "ENUM_ALIGN_MODE",
        values: &[
            "ALIGN_LEFT", "ALIGN_CENTER", "ALIGN_RIGHT",
        ],
    },
    BuiltinEnum {
        name: "ENUM_CHART_MODE",
        values: &[
            "CHART_BARS", "CHART_CANDLES", "CHART_LINE",
        ],
    },
    BuiltinEnum {
        name: "ENUM_CHART_VOLUME_MODE",
        values: &[
            "CHART_VOLUME_HIDE", "CHART_VOLUME_TICK", "CHART_VOLUME_REAL",
        ],
    },
    BuiltinEnum {
        name: "ENUM_INIT_RETCODE",
        values: &[
            "INIT_SUCCEEDED", "INIT_FAILED", "INIT_PARAMETERS_INCORRECT", "INIT_AGENT_NOT_SUITABLE",
        ],
    },
    BuiltinEnum {
        name: "ENUM_UNINIT_REASON",
        values: &[
            "REASON_PROGRAM", "REASON_REMOVE", "REASON_RECOMPILE", "REASON_CHARTCHANGE",
            "REASON_CHARTCLOSE", "REASON_PARAMETERS", "REASON_ACCOUNT", "REASON_TEMPLATE",
            "REASON_INITFAILED", "REASON_CLOSE",
        ],
    },
    BuiltinEnum {
        name: "ENUM_FILE_FLAGS",
        values: &[
            "FILE_READ", "FILE_WRITE", "FILE_BIN", "FILE_CSV",
            "FILE_TXT", "FILE_ANSI", "FILE_UNICODE", "FILE_SHARE_READ",
            "FILE_SHARE_WRITE", "FILE_REWRITE", "FILE_COMMON",
        ],
    },
    BuiltinEnum {
        name: "ENUM_CHARTEVENT",
        values: &[
            "CHARTEVENT_KEYDOWN", "CHARTEVENT_MOUSE_MOVE",
            "CHARTEVENT_OBJECT_CREATE", "CHARTEVENT_OBJECT_CHANGE",
            "CHARTEVENT_OBJECT_DELETE", "CHARTEVENT_CLICK",
            "CHARTEVENT_OBJECT_CLICK", "CHARTEVENT_OBJECT_DRAG",
            "CHARTEVENT_OBJECT_ENDEDIT", "CHARTEVENT_CHART_CHANGE",
            "CHARTEVENT_CUSTOM", "CHARTEVENT_CUSTOM_LAST",
            "CHARTEVENT_MOUSE_WHEEL",
        ],
    },
    BuiltinEnum {
        name: "ENUM_CRYPT_METHOD",
        values: &[
            "CRYPT_BASE64", "CRYPT_AES128", "CRYPT_AES256", "CRYPT_DES",
            "CRYPT_HASH_SHA1", "CRYPT_HASH_SHA256", "CRYPT_HASH_MD5",
            "CRYPT_ARCH_ZIP",
        ],
    },
    BuiltinEnum {
        name: "ENUM_ORDER_STATE",
        values: &[
            "ORDER_STATE_STARTED", "ORDER_STATE_PLACED", "ORDER_STATE_CANCELED",
            "ORDER_STATE_PARTIAL", "ORDER_STATE_FILLED", "ORDER_STATE_REJECTED",
            "ORDER_STATE_EXPIRED", "ORDER_STATE_REQUEST_ADD", "ORDER_STATE_REQUEST_MODIFY",
            "ORDER_STATE_REQUEST_CANCEL",
        ],
    },
    BuiltinEnum {
        name: "ENUM_TRADE_RETCODE",
        values: &[
            "TRADE_RETCODE_REQUOTE", "TRADE_RETCODE_REJECT", "TRADE_RETCODE_CANCEL",
            "TRADE_RETCODE_PLACED", "TRADE_RETCODE_DONE", "TRADE_RETCODE_DONE_PARTIAL",
            "TRADE_RETCODE_ERROR", "TRADE_RETCODE_TIMEOUT", "TRADE_RETCODE_INVALID",
            "TRADE_RETCODE_INVALID_VOLUME", "TRADE_RETCODE_INVALID_PRICE",
            "TRADE_RETCODE_INVALID_STOPS", "TRADE_RETCODE_TRADE_DISABLED",
            "TRADE_RETCODE_MARKET_CLOSED", "TRADE_RETCODE_NO_MONEY",
            "TRADE_RETCODE_PRICE_CHANGED", "TRADE_RETCODE_PRICE_OFF",
            "TRADE_RETCODE_INVALID_EXPIRATION", "TRADE_RETCODE_ORDER_CHANGED",
            "TRADE_RETCODE_TOO_MANY_REQUESTS", "TRADE_RETCODE_NO_CHANGES",
            "TRADE_RETCODE_SERVER_DISABLES_AT", "TRADE_RETCODE_CLIENT_DISABLES_AT",
            "TRADE_RETCODE_LOCKED", "TRADE_RETCODE_FROZEN",
            "TRADE_RETCODE_INVALID_FILL", "TRADE_RETCODE_CONNECTION",
            "TRADE_RETCODE_ONLY_REAL", "TRADE_RETCODE_LIMIT_ORDERS",
            "TRADE_RETCODE_LIMIT_VOLUME", "TRADE_RETCODE_INVALID_ORDER",
            "TRADE_RETCODE_POSITION_CLOSED", "TRADE_RETCODE_CLOSE_ORDER_EXIST",
            "TRADE_RETCODE_LIMIT_POSITIONS", "TRADE_RETCODE_REJECT_CANCEL",
            "TRADE_RETCODE_LONG_ONLY", "TRADE_RETCODE_SHORT_ONLY",
            "TRADE_RETCODE_CLOSE_ONLY", "TRADE_RETCODE_FIFO_CLOSE",
        ],
    },
    BuiltinEnum {
        name: "ENUM_SYMBOL_TRADE_EXECUTION",
        values: &[
            "SYMBOL_TRADE_EXECUTION_REQUEST", "SYMBOL_TRADE_EXECUTION_INSTANT",
            "SYMBOL_TRADE_EXECUTION_MARKET", "SYMBOL_TRADE_EXECUTION_EXCHANGE",
        ],
    },
    BuiltinEnum {
        name: "ENUM_SYMBOL_TRADE_MODE",
        values: &[
            "SYMBOL_TRADE_MODE_DISABLED", "SYMBOL_TRADE_MODE_LONGONLY",
            "SYMBOL_TRADE_MODE_SHORTONLY", "SYMBOL_TRADE_MODE_CLOSEONLY",
            "SYMBOL_TRADE_MODE_FULL",
        ],
    },
    BuiltinEnum {
        name: "ENUM_POSITION_REASON",
        values: &[
            "POSITION_REASON_CLIENT", "POSITION_REASON_MOBILE",
            "POSITION_REASON_WEB", "POSITION_REASON_EXPERT",
        ],
    },
    BuiltinEnum {
        name: "ENUM_ORDER_REASON",
        values: &[
            "ORDER_REASON_CLIENT", "ORDER_REASON_MOBILE", "ORDER_REASON_WEB",
            "ORDER_REASON_EXPERT", "ORDER_REASON_SL", "ORDER_REASON_TP", "ORDER_REASON_SO",
        ],
    },
    BuiltinEnum {
        name: "ENUM_DEAL_REASON",
        values: &[
            "DEAL_REASON_CLIENT", "DEAL_REASON_MOBILE", "DEAL_REASON_WEB",
            "DEAL_REASON_EXPERT", "DEAL_REASON_SL", "DEAL_REASON_TP",
            "DEAL_REASON_SO", "DEAL_REASON_ROLLOVER", "DEAL_REASON_VMARGIN",
            "DEAL_REASON_SPLIT",
        ],
    },
    BuiltinEnum {
        name: "ENUM_COLOR_FORMAT",
        values: &[
            "COLOR_FORMAT_XRGB_NOALPHA", "COLOR_FORMAT_ARGB_RAW", "COLOR_FORMAT_ARGB_NORMALIZE",
        ],
    },
    BuiltinEnum {
        name: "ENUM_FILE_POSITION",
        values: &[
            "SEEK_SET", "SEEK_CUR", "SEEK_END",
        ],
    },
    BuiltinEnum {
        name: "ENUM_SYMBOL_CALC_MODE",
        values: &[
            "SYMBOL_CALC_MODE_FOREX", "SYMBOL_CALC_MODE_FUTURES",
            "SYMBOL_CALC_MODE_CFD", "SYMBOL_CALC_MODE_CFDINDEX",
            "SYMBOL_CALC_MODE_CFDLEVERAGE", "SYMBOL_CALC_MODE_FOREX_NO_LEVERAGE",
            "SYMBOL_CALC_MODE_EXCH_STOCKS", "SYMBOL_CALC_MODE_EXCH_FUTURES",
            "SYMBOL_CALC_MODE_EXCH_FUTURES_FORTS", "SYMBOL_CALC_MODE_EXCH_BONDS",
            "SYMBOL_CALC_MODE_EXCH_STOCKS_MOEX", "SYMBOL_CALC_MODE_EXCH_BONDS_MOEX",
            "SYMBOL_CALC_MODE_SERV_COLLATERAL",
        ],
    },
    BuiltinEnum {
        name: "ENUM_SYMBOL_SWAP_MODE",
        values: &[
            "SYMBOL_SWAP_MODE_DISABLED", "SYMBOL_SWAP_MODE_POINTS",
            "SYMBOL_SWAP_MODE_CURRENCY_SYMBOL", "SYMBOL_SWAP_MODE_CURRENCY_MARGIN",
            "SYMBOL_SWAP_MODE_CURRENCY_DEPOSIT", "SYMBOL_SWAP_MODE_INTEREST_CURRENT",
            "SYMBOL_SWAP_MODE_INTEREST_OPEN", "SYMBOL_SWAP_MODE_REOPEN_CURRENT",
            "SYMBOL_SWAP_MODE_REOPEN_BID",
        ],
    },
    BuiltinEnum {
        name: "ENUM_DAY_OF_WEEK",
        values: &[
            "SUNDAY", "MONDAY", "TUESDAY", "WEDNESDAY", "THURSDAY", "FRIDAY", "SATURDAY",
        ],
    },
    BuiltinEnum {
        name: "ENUM_ACCOUNT_TRADE_MODE",
        values: &[
            "ACCOUNT_TRADE_MODE_DEMO", "ACCOUNT_TRADE_MODE_CONTEST", "ACCOUNT_TRADE_MODE_REAL",
        ],
    },
    BuiltinEnum {
        name: "ENUM_ACCOUNT_STOPOUT_MODE",
        values: &[
            "ACCOUNT_STOPOUT_MODE_PERCENT", "ACCOUNT_STOPOUT_MODE_MONEY",
        ],
    },
    BuiltinEnum {
        name: "ENUM_ACCOUNT_MARGIN_MODE",
        values: &[
            "ACCOUNT_MARGIN_MODE_RETAIL_NETTING", "ACCOUNT_MARGIN_MODE_EXCHANGE",
            "ACCOUNT_MARGIN_MODE_RETAIL_HEDGING",
        ],
    },
    BuiltinEnum {
        name: "ENUM_MQL_INFO_INTEGER",
        values: &[
            "MQL_PROGRAM_TYPE", "MQL_DLLS_ALLOWED", "MQL_TRADE_ALLOWED",
            "MQL_SIGNALS_ALLOWED", "MQL_DEBUG", "MQL_PROFILER",
            "MQL_TESTER", "MQL_FORWARD", "MQL_OPTIMIZATION",
            "MQL_VISUAL_MODE", "MQL_FRAME_MODE", "MQL_LICENSE_TYPE",
            "MQL_HANDLES_USED", "MQL_MEMORY_LIMIT", "MQL_MEMORY_USED",
        ],
    },
    BuiltinEnum {
        name: "ENUM_LOG_LEVELS",
        values: &[
            "LOG_LEVEL_NO", "LOG_LEVEL_ERRORS", "LOG_LEVEL_ALL",
        ],
    },
];

// =============================================================================
// BUILTIN STRUCTS
// =============================================================================
pub static BUILTIN_STRUCTS: &[BuiltinStruct] = &[
    BuiltinStruct {
        name: "MqlTick",
        fields: &[
            ("time", "datetime"),
            ("bid", "double"),
            ("ask", "double"),
            ("last", "double"),
            ("volume", "ulong"),
            ("time_msc", "long"),
            ("flags", "uint"),
            ("volume_real", "double"),
        ],
    },
    BuiltinStruct {
        name: "MqlRates",
        fields: &[
            ("time", "datetime"),
            ("open", "double"),
            ("high", "double"),
            ("low", "double"),
            ("close", "double"),
            ("tick_volume", "long"),
            ("spread", "int"),
            ("real_volume", "long"),
        ],
    },
    BuiltinStruct {
        name: "MqlTradeRequest",
        fields: &[
            ("action", "ENUM_TRADE_REQUEST_ACTIONS"),
            ("magic", "ulong"),
            ("order", "ulong"),
            ("symbol", "string"),
            ("volume", "double"),
            ("price", "double"),
            ("stoplimit", "double"),
            ("sl", "double"),
            ("tp", "double"),
            ("deviation", "ulong"),
            ("type", "ENUM_ORDER_TYPE"),
            ("type_filling", "ENUM_ORDER_TYPE_FILLING"),
            ("type_time", "ENUM_ORDER_TYPE_TIME"),
            ("expiration", "datetime"),
            ("comment", "string"),
            ("position", "ulong"),
            ("position_by", "ulong"),
        ],
    },
    BuiltinStruct {
        name: "MqlTradeResult",
        fields: &[
            ("retcode", "uint"),
            ("deal", "ulong"),
            ("order", "ulong"),
            ("volume", "double"),
            ("price", "double"),
            ("bid", "double"),
            ("ask", "double"),
            ("comment", "string"),
            ("request_id", "uint"),
            ("retcode_external", "int"),
        ],
    },
    BuiltinStruct {
        name: "MqlTradeCheckResult",
        fields: &[
            ("retcode", "uint"),
            ("balance", "double"),
            ("equity", "double"),
            ("profit", "double"),
            ("margin", "double"),
            ("margin_free", "double"),
            ("margin_level", "double"),
            ("comment", "string"),
        ],
    },
    BuiltinStruct {
        name: "MqlBookInfo",
        fields: &[
            ("type", "int"),
            ("price", "double"),
            ("volume", "long"),
            ("volume_real", "double"),
        ],
    },
    BuiltinStruct {
        name: "MqlParam",
        fields: &[
            ("type", "int"),
            ("integer_value", "long"),
            ("double_value", "double"),
            ("string_value", "string"),
        ],
    },
    BuiltinStruct {
        name: "MqlDateTime",
        fields: &[
            ("year", "int"),
            ("mon", "int"),
            ("day", "int"),
            ("hour", "int"),
            ("min", "int"),
            ("sec", "int"),
            ("day_of_week", "int"),
            ("day_of_year", "int"),
        ],
    },
    BuiltinStruct {
        name: "MqlTradeTransaction",
        fields: &[
            ("deal", "ulong"),
            ("order", "ulong"),
            ("symbol", "string"),
            ("type", "int"),
            ("order_type", "int"),
            ("order_state", "int"),
            ("deal_type", "int"),
            ("time_type", "int"),
            ("time_expiration", "datetime"),
            ("price", "double"),
            ("price_trigger", "double"),
            ("price_sl", "double"),
            ("price_tp", "double"),
            ("volume", "double"),
            ("position", "ulong"),
            ("position_by", "ulong"),
        ],
    },
];

// =============================================================================
// BUILTIN CONSTANTS
// =============================================================================
pub static BUILTIN_CONSTANTS: &[BuiltinConstant] = &[
    // --- Text alignment flags ---
    BuiltinConstant { name: "TA_LEFT", value: "0", doc: Some("Text alignment: left") },
    BuiltinConstant { name: "TA_CENTER", value: "6", doc: Some("Text alignment: center") },
    BuiltinConstant { name: "TA_RIGHT", value: "2", doc: Some("Text alignment: right") },
    BuiltinConstant { name: "TA_TOP", value: "0", doc: Some("Text alignment: top") },
    BuiltinConstant { name: "TA_VCENTER", value: "24", doc: Some("Text alignment: vertical center") },
    BuiltinConstant { name: "TA_BOTTOM", value: "8", doc: Some("Text alignment: bottom") },
    // --- Font weight ---
    BuiltinConstant { name: "FW_NORMAL", value: "400", doc: Some("Font weight: normal") },
    BuiltinConstant { name: "FW_BOLD", value: "700", doc: Some("Font weight: bold") },
    // --- Object visibility ---
    BuiltinConstant { name: "OBJ_NO_PERIODS", value: "0", doc: Some("Object not visible on any timeframe") },
    BuiltinConstant { name: "OBJ_ALL_PERIODS", value: "0x1FFFFF", doc: Some("Object visible on all timeframes") },
    // --- Chart navigation ---
    BuiltinConstant { name: "CHART_BEGIN", value: "0", doc: Some("Chart navigation: beginning of chart") },
    BuiltinConstant { name: "CHART_END", value: "2", doc: Some("Chart navigation: end of chart") },
    BuiltinConstant { name: "CHART_CURRENT_POS", value: "1", doc: Some("Chart navigation: current position") },
    // --- Time flags ---
    BuiltinConstant { name: "TIME_DATE", value: "1", doc: Some("TimeToString flag: include date") },
    BuiltinConstant { name: "TIME_MINUTES", value: "2", doc: Some("TimeToString flag: include hours and minutes") },
    BuiltinConstant { name: "TIME_SECONDS", value: "4", doc: Some("TimeToString flag: include seconds") },
    // --- File property ---
    BuiltinConstant { name: "FILE_MODIFY_DATE", value: "7", doc: Some("File property: modification date") },
    // --- Type casting helpers ---
    BuiltinConstant { name: "INT_VALUE", value: "0", doc: None },
    BuiltinConstant { name: "CHAR_VALUE", value: "1", doc: None },
    // --- Special constants ---
    BuiltinConstant { name: "WRONG_VALUE", value: "-1", doc: Some("Invalid or wrong value") },
    BuiltinConstant { name: "EMPTY_VALUE", value: "DBL_MAX", doc: Some("Empty value for indicator buffers") },
    BuiltinConstant { name: "INVALID_HANDLE", value: "-1", doc: Some("Invalid handle value") },
    BuiltinConstant { name: "WHOLE_ARRAY", value: "-1", doc: Some("Process the entire array") },
    BuiltinConstant { name: "CLR_NONE", value: "-1", doc: Some("No color (transparent)") },
    BuiltinConstant { name: "CHARTS_MAX", value: "100", doc: Some("Maximum number of open charts") },
    BuiltinConstant { name: "EMPTY", value: "-1", doc: Some("Empty value") },
    // --- Symbol filling flags ---
    BuiltinConstant { name: "SYMBOL_FILLING_FOK", value: "1", doc: Some("Fill or Kill filling flag") },
    BuiltinConstant { name: "SYMBOL_FILLING_IOC", value: "2", doc: Some("Immediate or Cancel filling flag") },
    // --- Symbol expiration flags ---
    BuiltinConstant { name: "SYMBOL_EXPIRATION_GTC", value: "1", doc: Some("Good Till Cancelled expiration") },
    BuiltinConstant { name: "SYMBOL_EXPIRATION_DAY", value: "2", doc: Some("Expiration at end of day") },
    BuiltinConstant { name: "SYMBOL_EXPIRATION_SPECIFIED", value: "4", doc: Some("Expiration at specified time") },
    BuiltinConstant { name: "SYMBOL_EXPIRATION_SPECIFIED_DAY", value: "8", doc: Some("Expiration at specified day") },
    // --- Trade transaction types ---
    BuiltinConstant { name: "TRADE_TRANSACTION_ORDER_ADD", value: "0", doc: Some("Trade transaction: order added") },
    BuiltinConstant { name: "TRADE_TRANSACTION_ORDER_UPDATE", value: "1", doc: Some("Trade transaction: order updated") },
    BuiltinConstant { name: "TRADE_TRANSACTION_ORDER_DELETE", value: "2", doc: Some("Trade transaction: order deleted") },
    BuiltinConstant { name: "TRADE_TRANSACTION_DEAL_ADD", value: "6", doc: Some("Trade transaction: deal added") },
    BuiltinConstant { name: "TRADE_TRANSACTION_HISTORY_ADD", value: "8", doc: Some("Trade transaction: history order added") },
    BuiltinConstant { name: "TRADE_TRANSACTION_REQUEST", value: "10", doc: Some("Trade transaction: trade request") },
    // --- Color constants (MQL5 BGR format: 0x00BBGGRR) ---
    BuiltinConstant { name: "clrNONE", value: "-1", doc: Some("No color") },
    BuiltinConstant { name: "clrBlack", value: "0x000000", doc: None },
    BuiltinConstant { name: "clrDarkGreen", value: "0x006400", doc: None },
    BuiltinConstant { name: "clrDarkSlateGray", value: "0x2F4F4F", doc: None },
    BuiltinConstant { name: "clrOlive", value: "0x808000", doc: None },
    BuiltinConstant { name: "clrGreen", value: "0x008000", doc: None },
    BuiltinConstant { name: "clrTeal", value: "0x008080", doc: None },
    BuiltinConstant { name: "clrNavy", value: "0x000080", doc: None },
    BuiltinConstant { name: "clrPurple", value: "0x800080", doc: None },
    BuiltinConstant { name: "clrMaroon", value: "0x800000", doc: None },
    BuiltinConstant { name: "clrIndigo", value: "0x4B0082", doc: None },
    BuiltinConstant { name: "clrMidnightBlue", value: "0x191970", doc: None },
    BuiltinConstant { name: "clrDarkBlue", value: "0x00008B", doc: None },
    BuiltinConstant { name: "clrDarkOliveGreen", value: "0x556B2F", doc: None },
    BuiltinConstant { name: "clrSaddleBrown", value: "0x8B4513", doc: None },
    BuiltinConstant { name: "clrForestGreen", value: "0x228B22", doc: None },
    BuiltinConstant { name: "clrOliveDrab", value: "0x6B8E23", doc: None },
    BuiltinConstant { name: "clrSeaGreen", value: "0x2E8B57", doc: None },
    BuiltinConstant { name: "clrDarkGoldenrod", value: "0xB8860B", doc: None },
    BuiltinConstant { name: "clrDarkSlateBlue", value: "0x483D8B", doc: None },
    BuiltinConstant { name: "clrSienna", value: "0xA0522D", doc: None },
    BuiltinConstant { name: "clrMediumBlue", value: "0x0000CD", doc: None },
    BuiltinConstant { name: "clrBrown", value: "0xA52A2A", doc: None },
    BuiltinConstant { name: "clrDarkTurquoise", value: "0x00CED1", doc: None },
    BuiltinConstant { name: "clrDimGray", value: "0x696969", doc: None },
    BuiltinConstant { name: "clrLightSeaGreen", value: "0x20B2AA", doc: None },
    BuiltinConstant { name: "clrDarkViolet", value: "0x9400D3", doc: None },
    BuiltinConstant { name: "clrFireBrick", value: "0xB22222", doc: None },
    BuiltinConstant { name: "clrMediumVioletRed", value: "0xC71585", doc: None },
    BuiltinConstant { name: "clrMediumSeaGreen", value: "0x3CB371", doc: None },
    BuiltinConstant { name: "clrChocolate", value: "0xD2691E", doc: None },
    BuiltinConstant { name: "clrCrimson", value: "0xDC143C", doc: None },
    BuiltinConstant { name: "clrSteelBlue", value: "0x4682B4", doc: None },
    BuiltinConstant { name: "clrGoldenrod", value: "0xDAA520", doc: None },
    BuiltinConstant { name: "clrMediumSpringGreen", value: "0x00FA9A", doc: None },
    BuiltinConstant { name: "clrLawnGreen", value: "0x7CFC00", doc: None },
    BuiltinConstant { name: "clrCadetBlue", value: "0x5F9EA0", doc: None },
    BuiltinConstant { name: "clrDarkOrchid", value: "0x9932CC", doc: None },
    BuiltinConstant { name: "clrYellowGreen", value: "0x9ACD32", doc: None },
    BuiltinConstant { name: "clrLimeGreen", value: "0x32CD32", doc: None },
    BuiltinConstant { name: "clrOrangeRed", value: "0xFF4500", doc: None },
    BuiltinConstant { name: "clrDarkOrange", value: "0xFF8C00", doc: None },
    BuiltinConstant { name: "clrOrange", value: "0xFFA500", doc: None },
    BuiltinConstant { name: "clrGold", value: "0xFFD700", doc: None },
    BuiltinConstant { name: "clrYellow", value: "0xFFFF00", doc: None },
    BuiltinConstant { name: "clrChartreuse", value: "0x7FFF00", doc: None },
    BuiltinConstant { name: "clrLime", value: "0x00FF00", doc: None },
    BuiltinConstant { name: "clrSpringGreen", value: "0x00FF7F", doc: None },
    BuiltinConstant { name: "clrAqua", value: "0x00FFFF", doc: None },
    BuiltinConstant { name: "clrDeepSkyBlue", value: "0x00BFFF", doc: None },
    BuiltinConstant { name: "clrBlue", value: "0x0000FF", doc: None },
    BuiltinConstant { name: "clrMagenta", value: "0xFF00FF", doc: None },
    BuiltinConstant { name: "clrRed", value: "0xFF0000", doc: None },
    BuiltinConstant { name: "clrGray", value: "0x808080", doc: None },
    BuiltinConstant { name: "clrSlateGray", value: "0x708090", doc: None },
    BuiltinConstant { name: "clrPeru", value: "0xCD853F", doc: None },
    BuiltinConstant { name: "clrBlueViolet", value: "0x8A2BE2", doc: None },
    BuiltinConstant { name: "clrLightSlateGray", value: "0x778899", doc: None },
    BuiltinConstant { name: "clrDeepPink", value: "0xFF1493", doc: None },
    BuiltinConstant { name: "clrMediumTurquoise", value: "0x48D1CC", doc: None },
    BuiltinConstant { name: "clrDodgerBlue", value: "0x1E90FF", doc: None },
    BuiltinConstant { name: "clrTurquoise", value: "0x40E0D0", doc: None },
    BuiltinConstant { name: "clrRoyalBlue", value: "0x4169E1", doc: None },
    BuiltinConstant { name: "clrSlateBlue", value: "0x6A5ACD", doc: None },
    BuiltinConstant { name: "clrDarkKhaki", value: "0xBDB76B", doc: None },
    BuiltinConstant { name: "clrIndianRed", value: "0xCD5C5C", doc: None },
    BuiltinConstant { name: "clrMediumOrchid", value: "0xBA55D3", doc: None },
    BuiltinConstant { name: "clrGreenYellow", value: "0xADFF2F", doc: None },
    BuiltinConstant { name: "clrMediumAquamarine", value: "0x66CDAA", doc: None },
    BuiltinConstant { name: "clrDarkSeaGreen", value: "0x8FBC8F", doc: None },
    BuiltinConstant { name: "clrTomato", value: "0xFF6347", doc: None },
    BuiltinConstant { name: "clrRosyBrown", value: "0xBC8F8F", doc: None },
    BuiltinConstant { name: "clrOrchid", value: "0xDA70D6", doc: None },
    BuiltinConstant { name: "clrMediumPurple", value: "0x9370DB", doc: None },
    BuiltinConstant { name: "clrPaleVioletRed", value: "0xDB7093", doc: None },
    BuiltinConstant { name: "clrCoral", value: "0xFF7F50", doc: None },
    BuiltinConstant { name: "clrCornflowerBlue", value: "0x6495ED", doc: None },
    BuiltinConstant { name: "clrDarkGray", value: "0xA9A9A9", doc: None },
    BuiltinConstant { name: "clrSandyBrown", value: "0xF4A460", doc: None },
    BuiltinConstant { name: "clrMediumSlateBlue", value: "0x7B68EE", doc: None },
    BuiltinConstant { name: "clrTan", value: "0xD2B48C", doc: None },
    BuiltinConstant { name: "clrDarkSalmon", value: "0xE9967A", doc: None },
    BuiltinConstant { name: "clrBurlyWood", value: "0xDEB887", doc: None },
    BuiltinConstant { name: "clrHotPink", value: "0xFF69B4", doc: None },
    BuiltinConstant { name: "clrSalmon", value: "0xFA8072", doc: None },
    BuiltinConstant { name: "clrViolet", value: "0xEE82EE", doc: None },
    BuiltinConstant { name: "clrLightCoral", value: "0xF08080", doc: None },
    BuiltinConstant { name: "clrSkyBlue", value: "0x87CEEB", doc: None },
    BuiltinConstant { name: "clrLightSalmon", value: "0xFFA07A", doc: None },
    BuiltinConstant { name: "clrPlum", value: "0xDDA0DD", doc: None },
    BuiltinConstant { name: "clrKhaki", value: "0xF0E68C", doc: None },
    BuiltinConstant { name: "clrLightGreen", value: "0x90EE90", doc: None },
    BuiltinConstant { name: "clrAquamarine", value: "0x7FFFD4", doc: None },
    BuiltinConstant { name: "clrSilver", value: "0xC0C0C0", doc: None },
    BuiltinConstant { name: "clrLightSkyBlue", value: "0x87CEFA", doc: None },
    BuiltinConstant { name: "clrLightSteelBlue", value: "0xB0C4DE", doc: None },
    BuiltinConstant { name: "clrLightBlue", value: "0xADD8E6", doc: None },
    BuiltinConstant { name: "clrPaleGreen", value: "0x98FB98", doc: None },
    BuiltinConstant { name: "clrThistle", value: "0xD8BFD8", doc: None },
    BuiltinConstant { name: "clrPowderBlue", value: "0xB0E0E6", doc: None },
    BuiltinConstant { name: "clrPaleGoldenrod", value: "0xEEE8AA", doc: None },
    BuiltinConstant { name: "clrPaleTurquoise", value: "0xAFEEEE", doc: None },
    BuiltinConstant { name: "clrLightGray", value: "0xD3D3D3", doc: None },
    BuiltinConstant { name: "clrWheat", value: "0xF5DEB3", doc: None },
    BuiltinConstant { name: "clrNavajoWhite", value: "0xFFDEAD", doc: None },
    BuiltinConstant { name: "clrMoccasin", value: "0xFFE4B5", doc: None },
    BuiltinConstant { name: "clrLightPink", value: "0xFFB6C1", doc: None },
    BuiltinConstant { name: "clrGainsboro", value: "0xDCDCDC", doc: None },
    BuiltinConstant { name: "clrPeachPuff", value: "0xFFDAB9", doc: None },
    BuiltinConstant { name: "clrPink", value: "0xFFC0CB", doc: None },
    BuiltinConstant { name: "clrBisque", value: "0xFFE4C4", doc: None },
    BuiltinConstant { name: "clrLightGoldenrod", value: "0xFAFAD2", doc: None },
    BuiltinConstant { name: "clrBlanchedAlmond", value: "0xFFEBCD", doc: None },
    BuiltinConstant { name: "clrLemonChiffon", value: "0xFFFACD", doc: None },
    BuiltinConstant { name: "clrBeige", value: "0xF5F5DC", doc: None },
    BuiltinConstant { name: "clrAntiqueWhite", value: "0xFAEBD7", doc: None },
    BuiltinConstant { name: "clrPapayaWhip", value: "0xFFEFD5", doc: None },
    BuiltinConstant { name: "clrCornsilk", value: "0xFFF8DC", doc: None },
    BuiltinConstant { name: "clrLightYellow", value: "0xFFFFE0", doc: None },
    BuiltinConstant { name: "clrLightCyan", value: "0xE0FFFF", doc: None },
    BuiltinConstant { name: "clrLinen", value: "0xFAF0E6", doc: None },
    BuiltinConstant { name: "clrLavender", value: "0xE6E6FA", doc: None },
    BuiltinConstant { name: "clrMistyRose", value: "0xFFE4E1", doc: None },
    BuiltinConstant { name: "clrOldLace", value: "0xFDF5E6", doc: None },
    BuiltinConstant { name: "clrWhiteSmoke", value: "0xF5F5F5", doc: None },
    BuiltinConstant { name: "clrSeashell", value: "0xFFF5EE", doc: None },
    BuiltinConstant { name: "clrIvory", value: "0xFFFFF0", doc: None },
    BuiltinConstant { name: "clrHoneydew", value: "0xF0FFF0", doc: None },
    BuiltinConstant { name: "clrAliceBlue", value: "0xF0F8FF", doc: None },
    BuiltinConstant { name: "clrLavenderBlush", value: "0xFFF0F5", doc: None },
    BuiltinConstant { name: "clrMintCream", value: "0xF5FFFA", doc: None },
    BuiltinConstant { name: "clrSnow", value: "0xFFFAFA", doc: None },
    BuiltinConstant { name: "clrWhite", value: "0xFFFFFF", doc: None },
];

// =============================================================================
// BUILTIN GLOBALS (predefined variables)
// =============================================================================
pub static BUILTIN_GLOBALS: &[(&str, &str)] = &[
    ("_Symbol", "string"),
    ("_Point", "double"),
    ("_Digits", "int"),
    ("_Period", "int"),
    ("_StopFlag", "int"),
    ("_UninitReason", "int"),
    ("_LastError", "int"),
    ("_RandomSeed", "int"),
    ("_IsExpert", "int"),
    ("_IsIndicator", "int"),
    ("_IsScript", "int"),
    ("_AppliedTo", "int"),
];

// =============================================================================
// MQL5 KEYWORDS
// =============================================================================
pub static MQL5_KEYWORDS: &[&str] = &[
    // Storage & qualifiers
    "input", "sinput", "virtual", "override", "const", "static", "extern",
    // Type keywords
    "void", "bool", "char", "short", "int", "long", "float", "double", "string",
    "uchar", "ushort", "uint", "ulong", "datetime", "color",
    // OOP
    "class", "struct", "enum", "union", "interface", "template", "typename",
    "public", "private", "protected",
    // Memory / misc operators
    "new", "delete", "this", "sizeof",
    // Literals
    "true", "false", "NULL", "EMPTY_VALUE", "WRONG_VALUE",
    // Control flow
    "return", "if", "else", "for", "while", "do",
    "switch", "case", "default", "break", "continue",
    // Preprocessor
    "#property", "#include", "#define", "#import", "#resource",
    // Event handlers
    "OnInit", "OnDeinit", "OnTick", "OnTimer", "OnChartEvent",
    "OnCalculate", "OnStart", "OnTrade", "OnTradeTransaction",
    "OnTester", "OnTesterInit", "OnTesterDeinit", "OnTesterPass",
    "OnBookEvent",
];

// =============================================================================
// MQL5 PRIMITIVE TYPES
// =============================================================================
pub static MQL5_PRIMITIVE_TYPES: &[&str] = &[
    "int", "double", "float", "long", "ulong", "uint", "ushort", "uchar",
    "bool", "char", "short", "string", "datetime", "color", "void",
];

// =============================================================================
// LOOKUP FUNCTIONS
// =============================================================================
pub fn find_function(name: &str) -> Option<&'static BuiltinFunction> {
    BUILTIN_FUNCTIONS.iter().find(|f| f.name == name)
}

pub fn find_enum(name: &str) -> Option<&'static BuiltinEnum> {
    BUILTIN_ENUMS.iter().find(|e| e.name == name)
}

pub fn find_struct(name: &str) -> Option<&'static BuiltinStruct> {
    BUILTIN_STRUCTS.iter().find(|s| s.name == name)
}

pub fn find_constant(name: &str) -> Option<&'static BuiltinConstant> {
    BUILTIN_CONSTANTS.iter().find(|c| c.name == name)
}

pub fn is_builtin_type(name: &str) -> bool {
    MQL5_PRIMITIVE_TYPES.contains(&name) || find_enum(name).is_some() || find_struct(name).is_some()
}
