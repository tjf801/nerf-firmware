
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
    
    LowerColumnStart(u8),
    UpperColumnStart(u8),
    /// Set the starting column. This is only for horizontal/vertical addressing mode.
    ColumnStart(u8),
    /// Set the starting page. This is only for page addressing mode.
    PageStart(Page),
    
    SetAddressMode(AddressMode),
    // /// Set column start and end address. This is only for horizontal/vertical addressing mode.
    // SetColumnAddress{start_col: u8, end_col: u8},
    // /// Set page start and end address. This is only for horizontal/vertical addressing mode.
    // SetPageAddress{start_page: Page, end_page: Page},
    // SetupVerticalScroll(...), // 0x28 | 0x29
    
    EnableScroll(bool),
    
    // StartLine(u8), 0x40-0x7F
    
    ChargePump(bool), // 0x8D
    
    // SegmentRemap(bool), // 0xA0 | 0xA1
    // SetupVerticalScrollArea(u8, u8), // 0xA3
    
    /// ratio is between 15-63, real value is ratio+1
    SetMultiplexRatio{ratio: u8}, // 0xA8
    
    // InternalIref(bool, bool), // 0xAD
    
    // ReverseComDir(bool), // 0xC0 | 0xC8
    
    // DisplayOffset(u8), // 0xD3, [0]
    
    
    // ComPinConfig(u8), // 0xDA
    
    // Timing & Driving Scheme Setting Command Table
    // =============================================
    
    /// ### Set Display Clock Divide Ratio/Oscillator Frequency.
    /// 
    /// both values are 0-15, real divide ratio is `divide_ratio+1`
    DisplayClockDiv{oscillator_freq: u8, divide_ratio: u8}, // 0xD5
    
    /// top nibble is phase2, bottom nibble is phase1
    PreChargePeriod(u8), // 0xD9, [0]
    
    // VcomhDeselect(VcomhLevel), // 0xDB, [0]
    
    /// ### Command for no operation (NOP).
    NoOp, // 0xE3
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

