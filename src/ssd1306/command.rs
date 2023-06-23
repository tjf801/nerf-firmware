
#[derive(Debug, Copy, Clone)]
pub enum Command {
    // Fundamental commands
    // ====================
    
    /// ### Set Contrast Control.
    /// 
    /// Double byte command to select 1 out of 256 contrast steps.
    /// Contrast increases as the value increases.
    /// (RESET = 0x7F)
    SetContrast{contrast: u8}, // 0x81
    
    /// ### Entire Display ON.
    /// 
    /// `false`: Resume to RAM content display, i.e. output follows RAM content.
    /// 
    /// `true`: Entire display ON. Output ignores RAM content.
    /// 
    /// (RESET = false)
    AllPixelsOn(bool), // false=0xA4, true=0xA5
    
    /// ### Set Normal/Inverse Display.
    /// 
    /// `false`: Normal display, i.e. pixels are ON if the RAM content is `1`.
    /// 
    /// `true`: Inverse display, i.e. pixels are ON if the RAM content is `0`.
    /// 
    /// (RESET = false)
    InvertDisplay(bool), // false=0xA6, true=0xA7
    
    /// ### Set Display ON/OFF.
    /// 
    /// `false`: Display OFF (sleep mode).
    /// 
    /// `true`: Display ON in normal mode.
    DisplayEnable(bool), // false=0xAE, true=0xAF
    
    
    // Scrolling commands
    // ==================
    
    /// ### Continuous Horizontal Scroll Setup.
    /// 
    /// `dir`: Direction of scrolling (right/left).
    /// 
    /// `interval`: Time interval between each scroll step (in terms of frame frequency).
    /// 
    /// TODO: `start_page` and `end_page`
    SetupHorizontalScroll {
        /// Direction of scrolling (right/left).
        direction: HorizontalScroll,
        
        /// Time interval between each scroll step (in terms of frame frequency).
        interval: ScrollInterval,
        
        /// "Define start page address". (TODO: what does this mean?)
        start_page: Page,
        
        /// "Define end page address".
        /// 
        /// `start_page` must be less than or equal to `end_page`.
        /// 
        /// TODO: make this unsafe somehow?
        end_page: Page
    },
    
    /// ### Continuous Vertical and Horizontal Scroll Setup.
    /// 
    /// `direction`: Direction of scrolling,
    /// i.e. Vertical and Right Horizontal Scroll, or Vertical and Left Horizontal Scroll.
    /// (Horizontal scroll by one column.)
    /// 
    /// **Note**: No continuous vertical scrolling is available.
    SetupVerticalAndHorizontalScroll {
        /// Direction of scrolling.
        /// (Vertical and Right Horizontal Scroll, or Vertical and Left Horizontal Scroll.)
        direction: VerticalHorizontalScroll,
        
        /// Time interval between each scroll step (in terms of frame frequency).
        interval: ScrollInterval,
        
        /// TODO: doc
        start_page: Page,
        
        /// TODO: doc
        end_page: Page,
        
        /// The vertical scrolling offset. (0-63)
        offset: u8
    },
    
    /// ### Activate/Deactivate Scrolling.
    /// 
    /// Starts or ends scrolling configured by the `SetupHorizontalScroll` and
    /// `VerticalHorizontalScrollSetup` commands.
    /// 
    /// `false`: Deactivate scrolling.
    /// 
    /// `true`: Activate scrolling.
    ///
    /// **Note:**
    /// 1. After deactivating scrolling, the RAM data needs to be rewritten.
    /// 2. Activation of scrolling must occur *directly* after the setup.  
    EnableScrolling(bool), // false=0x2E, true=0x2F
    
    /// ### Set Vertical Scroll Area.
    /// 
    /// `top`: The number of rows in the top fixed area, between 0 and 32.
    /// This is referenced to the top of the GDDRAM, or row 0. [RESET = 0]
    /// 
    /// `bottom`: The number of rows in the (bottom) scrolling area.
    /// This is the number of rows to be used for vertical scrolling.
    /// The scroll area starts in the first row below the top fixed area. [RESET = 64]
    /// 
    /// **Note:**
    /// 1. `top + bottom <= MUX_RATIO`.
    /// 2. `bottom <= MUX_RATIO`.
    /// 3. ​`offset` (from `VerticalHorizontalScrollSetup`) `< bottom`.
    /// 4. Set Display Start Line (from `SetDisplayStartLine`) `<= bottom`.
    /// 5. The last row of the scroll area shifts to the first row of the scroll area.
    /// 6. For `MUX_RATIO=64` display:
    ///     * `top = 0`, `bottom = 64` : whole area scrolls
    ///     * `top = 0`, `bottom < 64` : top area scrolls
    ///     * `top + bottom < 64` : central area scrolls
    ///     * `top + bottom = 64` : bottom area scrolls
    SetupVerticalScrollArea { top: u8, bottom: u8 }, // 0xA3
    
    
    // Addressing Setting Commands
    // ===========================
    
    /// ### Set Lower Column Start Address for Page Addressing Mode.
    /// 
    /// Set the lower nibble of the column start address register for
    /// Page Addressing Mode using `.0[3:0]` as data bits. The initial
    /// display line register is reset to 0000b after RESET.
    /// 
    /// **NOTE:** This command is only for page addressing mode.
    LowerColumnStart(u8), // 0x00-0x0F
    
    /// ### Set Higher Column Start Address for Page Addressing Mode.
    /// 
    /// Set the higher nibble of the column start address register for
    /// Page Addressing Mode using `.0[3:0]` as data bits. The initial
    /// display line register is reset to 0000b after RESET.
    /// 
    /// **NOTE:** This command is only for page addressing mode.
    UpperColumnStart(u8), // 0x10-0x1F
    
    /// ### Set Column Start Address for Page Addressing Mode.
    /// 
    /// Convenience command to set the entire column start address register
    /// for Page Addressing Mode in a single command.
    /// 
    /// **NOTE:** This command is only for page addressing mode.
    ColumnStart(u8), // LowerColumnStart + UpperColumnStart
    
    /// ### Set Memory Addressing Mode.
    /// 
    /// `AddressMode::Horizontal`:
    /// 
    /// `AddressMode::Vertical`:
    /// 
    /// `AddressMode::Page`: (RESET)
    SetAddressMode(AddressMode), // 0x20
    
    /// ### Set Column Address.
    /// 
    /// Setup column start and end address
    /// 
    /// `start`: Column start address, range: 0-127 (RESET=0)
    /// 
    /// `end`: Column end address, range: 0-127 (RESET=127)
    /// 
    /// **NOTE:** This is only for horizontal/vertical addressing mode.
    SetColumnAddress{start_col: u8, end_col: u8}, // 0x21
    
    /// ### Set Page Address.
    /// 
    /// Setup page start and end address
    /// 
    /// `start_page`: Page start address (RESET=Page::Page0)
    /// 
    /// `end_page`: Page end address (RESET=Page::Page7)
    /// 
    /// **NOTE:** This is only for page addressing mode.
    SetPageAddress{start_page: Page, end_page: Page}, // 0x22
    
    /// ### Set Page Start Address for Page Addressing Mode.
    /// 
    /// Set GDDRAM Page Start Address (PAGE0~PAGE7) for Page Addressing Mode.
    /// 
    /// **NOTE:** This command is only for page addressing mode.
    PageStart(Page), // 0xB0-0xB7
    
    
    // Hardware Configuration (Panel resolution & layout related) Commands
    // ==================================================================
    
    /// ### Set Display Start Line
    /// 
    /// Set display RAM display start line register from 0-63.
    /// Display start line register is reset to 0 during RESET.
    SetStartLine(u8), // 0x40-0x7F
    
    /// ### Set Segment Remap.
    /// 
    /// `false`: column address 0 is mapped to SEG0 (RESET)
    /// 
    /// `true`: column address 127 is mapped to SEG0
    SegmentRemap(bool), // 0xA0 | 0xA1
    
    /// ### Set Multiplex Ratio.
    /// 
    /// Set MUX ratio to `ratio+1` MUX. `ratio` must be in range `15..64`. (RESET=63)
    SetMultiplexRatio{ratio: u8}, // 0xA8
    
    /// ### Set COM Output Scan Direction
    /// 
    /// TODO
    SetComScanDir(bool), // 0xC0 | 0xC8
    
    // SetupVerticalScroll(...), // 0x28 | 0x29
    EnableScroll(bool),
    // SetupVerticalScrollArea(u8, u8), // 0xA3
    // InternalIref(bool, bool), // 0xAD
    // DisplayOffset(u8), // 0xD3
    // ComPinConfig(u8), // 0xDA
    
    
    // Timing & Driving Scheme Setting Command Table
    // =============================================
    
    /// ### Set Display Clock Divide Ratio/Oscillator Frequency.
    /// 
    /// both values are 0-15, real divide ratio is `divide_ratio+1`
    DisplayClockDiv{oscillator_freq: u8, divide_ratio: u8}, // 0xD5
    
    /// top nibble is phase2, bottom nibble is phase1
    PreChargePeriod(u8), // 0xD9
    
    // VcomhDeselect(VcomhLevel), // 0xDB
    
    /// ### Command for no operation (NOP).
    NoOp, // 0xE3
    
    
    // Charge pump command table
    // =========================
    
    /// ### Charge Pump Setting
    /// 
    /// `false`: disable charge pump (RESET)
    /// 
    /// `true`: enable charge pump
    /// 
    /// **NOTE:** The charge pump must be enabled directly before the display is turned on.
    ChargePump(bool), // 0x8D
}

/// Which direction to scroll (left or right).
#[derive(Debug, Clone, Copy)]
pub enum HorizontalScroll {
    /// Scroll to the left
    Left = 1,
    
    /// Scroll to the right
    Right = 0,
    
    // cha cha real smooth.
}

/// Which direction to scroll (vertical+left or vertical+right).
#[derive(Debug, Clone, Copy)]
pub enum VerticalHorizontalScroll {
    /// Scroll vertically and to the left
    VerticalLeft = 0b01,
    
    /// Scroll vertically and to the right
    VerticalRight = 0b10,
}

/// How many frames to wait between each scroll step.
#[derive(Debug, Clone, Copy)]
pub enum ScrollInterval {
    /// 5 frames
    Frames5 = 0b000,
    /// 64 frames
    Frames64 = 0b001,
    /// 128 frames
    Frames128 = 0b010,
    /// 256 frames
    Frames256 = 0b011,
    /// 3 frames
    Frames3 = 0b100,
    /// 4 frames
    Frames4 = 0b101,
    /// 25 frames
    Frames25 = 0b110,
    /// 2 frames
    Frames2 = 0b111,
}


#[derive(Debug, Clone, Copy)]
pub enum AddressMode {
    /// Horizontal mode
    Horizontal = 0b00,
    /// Vertical mode
    Vertical = 0b01,
    /// Page mode (default)
    Page = 0b10,
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Page {
    Page0 = 0b000,
    Page1 = 0b001,
    Page2 = 0b010,
    Page3 = 0b011,
    Page4 = 0b100,
    Page5 = 0b101,
    Page6 = 0b110,
    Page7 = 0b111,
}
impl From<u8> for Page {
    fn from(val: u8) -> Page {
        match val >> 3 {
            0 => Page::Page0,
            1 => Page::Page1,
            2 => Page::Page2,
            3 => Page::Page3,
            4 => Page::Page4,
            5 => Page::Page5,
            6 => Page::Page6,
            7 => Page::Page7,
            _ => panic!("Page too high"),
        }
    }
}

