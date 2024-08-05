#[doc = "Register `ETPS_ETFLG` reader"]
pub type R = crate::R<EtpsEtflgSpec>;
#[doc = "Register `ETPS_ETFLG` writer"]
pub type W = crate::W<EtpsEtflgSpec>;
#[doc = "Field `ETPS_INTPRD` reader - 1:0\\]
ePWM Interrupt (EPWMx_INT) Period Select These bits determine how many selected ETSEL\\[INTSEL\\]
events need to occur before an interrupt is generated. To be generated, the interrupt must be enabled (ETSEL\\[INT\\]
= 1). If the interrupt status flag is set from a previous interrupt (ETFLG\\[INT\\]
= 1) then no interrupt will be generated until the flag is cleared via the ETCLR\\[INT\\]
bit. This allows for one interrupt to be pending while another is still being serviced. Once the interrupt is generated, the ETPS\\[INTCNT\\]
bits will automatically be cleared. Writing a INTPRD value that is the same as the current counter value will trigger an interrupt if it is enabled and the status flag is clear. Writing a INTPRD value that is less than the current counter value will result in an undefined state. If a counter event occurs at the same instant as a new zero or non-zero INTPRD value is written, the counter is incremented. 0 Disable the interrupt event counter. No interrupt will be generated and ETFRC\\[INT\\]
is ignored. 1h Generate an interrupt on the first event INTCNT = 01 (first event) 2h Generate interrupt on ETPS\\[INTCNT\\]
= 1,0 (second event) 3h Generate interrupt on ETPS\\[INTCNT\\]
= 1,1 (third event)"]
pub type EtpsIntprdR = crate::FieldReader;
#[doc = "Field `ETPS_INTPRD` writer - 1:0\\]
ePWM Interrupt (EPWMx_INT) Period Select These bits determine how many selected ETSEL\\[INTSEL\\]
events need to occur before an interrupt is generated. To be generated, the interrupt must be enabled (ETSEL\\[INT\\]
= 1). If the interrupt status flag is set from a previous interrupt (ETFLG\\[INT\\]
= 1) then no interrupt will be generated until the flag is cleared via the ETCLR\\[INT\\]
bit. This allows for one interrupt to be pending while another is still being serviced. Once the interrupt is generated, the ETPS\\[INTCNT\\]
bits will automatically be cleared. Writing a INTPRD value that is the same as the current counter value will trigger an interrupt if it is enabled and the status flag is clear. Writing a INTPRD value that is less than the current counter value will result in an undefined state. If a counter event occurs at the same instant as a new zero or non-zero INTPRD value is written, the counter is incremented. 0 Disable the interrupt event counter. No interrupt will be generated and ETFRC\\[INT\\]
is ignored. 1h Generate an interrupt on the first event INTCNT = 01 (first event) 2h Generate interrupt on ETPS\\[INTCNT\\]
= 1,0 (second event) 3h Generate interrupt on ETPS\\[INTCNT\\]
= 1,1 (third event)"]
pub type EtpsIntprdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ETPS_INTCNT` reader - 3:2\\]
ePWM Interrupt Event (EPWMx_INT) Counter Register These bits indicate how many selected ETSEL\\[INTSEL\\]
events have occurred. These bits are automatically cleared when an interrupt pulse is generated. If interrupts are disabled, ETSEL\\[INT\\]
= 0 or the interrupt flag is set, ETFLG\\[INT\\]
= 1, the counter will stop counting events when it reaches the period value ETPS\\[INTCNT\\]
= ETPS\\[INTPRD\\]. 0 No events have occurred. 1h 1 event has occurred. 2h 2 events have occurred. 3h 3 events have occurred."]
pub type EtpsIntcntR = crate::FieldReader;
#[doc = "Field `ETPS_INTCNT` writer - 3:2\\]
ePWM Interrupt Event (EPWMx_INT) Counter Register These bits indicate how many selected ETSEL\\[INTSEL\\]
events have occurred. These bits are automatically cleared when an interrupt pulse is generated. If interrupts are disabled, ETSEL\\[INT\\]
= 0 or the interrupt flag is set, ETFLG\\[INT\\]
= 1, the counter will stop counting events when it reaches the period value ETPS\\[INTCNT\\]
= ETPS\\[INTPRD\\]. 0 No events have occurred. 1h 1 event has occurred. 2h 2 events have occurred. 3h 3 events have occurred."]
pub type EtpsIntcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved3` reader - 7:4\\]
Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - 7:4\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETPS_SOCAPRD` reader - 9:8\\]
ePWM ADC Start-of-Conversion A Event (EPWMxSOCA) Period Select These bits determine how many selected ETSEL\\[SOCASEL\\]
events need to occur before an EPWMxSOCA pulse is generated. To be generated, the pulse must be enabled (ETSEL\\[SOCAEN\\]
= 1). The SOCA pulse will be generated even if the status flag is set from a previous start of conversion (ETFLG\\[SOCA\\]
= 1). Once the SOCA pulse is generated, the ETPS\\[SOCACNT\\]
bits will automatically be cleared. 0 Disable the SOCA event counter. No EPWMxSOCA pulse will be generated 1h Generate the EPWMxSOCA pulse on the first event: ETPS\\[SOCACNT\\]
= 0,1 2h Generate the EPWMxSOCA pulse on the second event: ETPS\\[SOCACNT\\]
= 1,0 3h Generate the EPWMxSOCA pulse on the third event: ETPS\\[SOCACNT\\]
= 1,1"]
pub type EtpsSocaprdR = crate::FieldReader;
#[doc = "Field `ETPS_SOCAPRD` writer - 9:8\\]
ePWM ADC Start-of-Conversion A Event (EPWMxSOCA) Period Select These bits determine how many selected ETSEL\\[SOCASEL\\]
events need to occur before an EPWMxSOCA pulse is generated. To be generated, the pulse must be enabled (ETSEL\\[SOCAEN\\]
= 1). The SOCA pulse will be generated even if the status flag is set from a previous start of conversion (ETFLG\\[SOCA\\]
= 1). Once the SOCA pulse is generated, the ETPS\\[SOCACNT\\]
bits will automatically be cleared. 0 Disable the SOCA event counter. No EPWMxSOCA pulse will be generated 1h Generate the EPWMxSOCA pulse on the first event: ETPS\\[SOCACNT\\]
= 0,1 2h Generate the EPWMxSOCA pulse on the second event: ETPS\\[SOCACNT\\]
= 1,0 3h Generate the EPWMxSOCA pulse on the third event: ETPS\\[SOCACNT\\]
= 1,1"]
pub type EtpsSocaprdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ETPS_SOCACNT` reader - 11:10\\]
ePWM ADC Start-of-Conversion A Event (EPWMxSOCA) Counter Register These bits indicate how many selected ETSEL\\[SOCASEL\\]
events have occurred: 0 No events have occurred. 1h 1 event has occurred. 2h 2 events have occurred. 3h 3 events have occurred."]
pub type EtpsSocacntR = crate::FieldReader;
#[doc = "Field `ETPS_SOCACNT` writer - 11:10\\]
ePWM ADC Start-of-Conversion A Event (EPWMxSOCA) Counter Register These bits indicate how many selected ETSEL\\[SOCASEL\\]
events have occurred: 0 No events have occurred. 1h 1 event has occurred. 2h 2 events have occurred. 3h 3 events have occurred."]
pub type EtpsSocacntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ETPS_SOCBPRD` reader - 13:12\\]
ePWM ADC Start-of-Conversion B Event (EPWMxSOCB) Period Select These bits determine how many selected ETSEL\\[SOCBSEL\\]
events need to occur before an EPWMxSOCB pulse is generated. To be generated, the pulse must be enabled (ETSEL\\[SOCBEN\\]
= 1). The SOCB pulse will be generated even if the status flag is set from a previous start of conversion (ETFLG\\[SOCB\\]
= 1). Once the SOCB pulse is generated, the ETPS\\[SOCBCNT\\]
bits will automatically be cleared. 0 Disable the SOCB event counter. No EPWMxSOCB pulse will be generated 1h Generate the EPWMxSOCB pulse on the first event: ETPS\\[SOCBCNT\\]
= 0,1 2h Generate the EPWMxSOCB pulse on the second event: ETPS\\[SOCBCNT\\]
= 1,0 3h Generate the EPWMxSOCB pulse on the third event: ETPS\\[SOCBCNT\\]
= 1,1"]
pub type EtpsSocbprdR = crate::FieldReader;
#[doc = "Field `ETPS_SOCBPRD` writer - 13:12\\]
ePWM ADC Start-of-Conversion B Event (EPWMxSOCB) Period Select These bits determine how many selected ETSEL\\[SOCBSEL\\]
events need to occur before an EPWMxSOCB pulse is generated. To be generated, the pulse must be enabled (ETSEL\\[SOCBEN\\]
= 1). The SOCB pulse will be generated even if the status flag is set from a previous start of conversion (ETFLG\\[SOCB\\]
= 1). Once the SOCB pulse is generated, the ETPS\\[SOCBCNT\\]
bits will automatically be cleared. 0 Disable the SOCB event counter. No EPWMxSOCB pulse will be generated 1h Generate the EPWMxSOCB pulse on the first event: ETPS\\[SOCBCNT\\]
= 0,1 2h Generate the EPWMxSOCB pulse on the second event: ETPS\\[SOCBCNT\\]
= 1,0 3h Generate the EPWMxSOCB pulse on the third event: ETPS\\[SOCBCNT\\]
= 1,1"]
pub type EtpsSocbprdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ETPS_SOCBCNT` reader - 15:14\\]
ePWM ADC Start-of-Conversion B Event (EPWMxSOCB) Counter Register These bits indicate how many selected ETSEL\\[SOCBSEL\\]
events have occurred: 0 No events have occurred. 1h 1 event has occurred. 2h 2 events have occurred. 3h 3 events have occurred"]
pub type EtpsSocbcntR = crate::FieldReader;
#[doc = "Field `ETPS_SOCBCNT` writer - 15:14\\]
ePWM ADC Start-of-Conversion B Event (EPWMxSOCB) Counter Register These bits indicate how many selected ETSEL\\[SOCBSEL\\]
events have occurred: 0 No events have occurred. 1h 1 event has occurred. 2h 2 events have occurred. 3h 3 events have occurred"]
pub type EtpsSocbcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ETFLG_INT` reader - 16:16\\]
Latched ePWM Interrupt (EPWMx_INT) Status Flag 0 Indicates no event occurred 1 Indicates that an ePWMx interrupt (EWPMx_INT) was generated. No further interrupts will be generated until the flag bit is cleared. Up to one interrupt can be pending while the ETFLG\\[INT\\]
bit is still set. If an interrupt is pending, it will not be generated until after the ETFLG\\[INT\\]
bit is cleared"]
pub type EtflgIntR = crate::BitReader;
#[doc = "Field `ETFLG_INT` writer - 16:16\\]
Latched ePWM Interrupt (EPWMx_INT) Status Flag 0 Indicates no event occurred 1 Indicates that an ePWMx interrupt (EWPMx_INT) was generated. No further interrupts will be generated until the flag bit is cleared. Up to one interrupt can be pending while the ETFLG\\[INT\\]
bit is still set. If an interrupt is pending, it will not be generated until after the ETFLG\\[INT\\]
bit is cleared"]
pub type EtflgIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 17:17\\]
Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - 17:17\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETFLG_SOCA` reader - 18:18\\]
Latched ePWM ADC Start-of-Conversion A (EPWMxSOCA) Status Flag Unlike the ETFLG\\[INT\\]
flag, the EPWMxSOCA output will continue to pulse even if the flag bit is set. 0 Indicates no event occurred 1 Indicates that a start of conversion pulse was generated on EPWMxSOCA. The EPWMxSOCA output will continue to be generated even if the flag bit is set"]
pub type EtflgSocaR = crate::BitReader;
#[doc = "Field `ETFLG_SOCA` writer - 18:18\\]
Latched ePWM ADC Start-of-Conversion A (EPWMxSOCA) Status Flag Unlike the ETFLG\\[INT\\]
flag, the EPWMxSOCA output will continue to pulse even if the flag bit is set. 0 Indicates no event occurred 1 Indicates that a start of conversion pulse was generated on EPWMxSOCA. The EPWMxSOCA output will continue to be generated even if the flag bit is set"]
pub type EtflgSocaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETFLG_SOCB` reader - 19:19\\]
Latched ePWM ADC Start-of-Conversion B (EPWMxSOCB) Status Flag 0 Indicates no EPWMxSOCB event occurred 1 Indicates that a start of conversion pulse was generated on EPWMxSOCB. The EPWMxSOCB output will continue to be generated even if the flag bit is set."]
pub type EtflgSocbR = crate::BitReader;
#[doc = "Field `ETFLG_SOCB` writer - 19:19\\]
Latched ePWM ADC Start-of-Conversion B (EPWMxSOCB) Status Flag 0 Indicates no EPWMxSOCB event occurred 1 Indicates that a start of conversion pulse was generated on EPWMxSOCB. The EPWMxSOCB output will continue to be generated even if the flag bit is set."]
pub type EtflgSocbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 31:20\\]
Reserved"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `Reserved2` writer - 31:20\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
ePWM Interrupt (EPWMx_INT) Period Select These bits determine how many selected ETSEL\\[INTSEL\\]
events need to occur before an interrupt is generated. To be generated, the interrupt must be enabled (ETSEL\\[INT\\]
= 1). If the interrupt status flag is set from a previous interrupt (ETFLG\\[INT\\]
= 1) then no interrupt will be generated until the flag is cleared via the ETCLR\\[INT\\]
bit. This allows for one interrupt to be pending while another is still being serviced. Once the interrupt is generated, the ETPS\\[INTCNT\\]
bits will automatically be cleared. Writing a INTPRD value that is the same as the current counter value will trigger an interrupt if it is enabled and the status flag is clear. Writing a INTPRD value that is less than the current counter value will result in an undefined state. If a counter event occurs at the same instant as a new zero or non-zero INTPRD value is written, the counter is incremented. 0 Disable the interrupt event counter. No interrupt will be generated and ETFRC\\[INT\\]
is ignored. 1h Generate an interrupt on the first event INTCNT = 01 (first event) 2h Generate interrupt on ETPS\\[INTCNT\\]
= 1,0 (second event) 3h Generate interrupt on ETPS\\[INTCNT\\]
= 1,1 (third event)"]
    #[inline(always)]
    pub fn etps_intprd(&self) -> EtpsIntprdR {
        EtpsIntprdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
ePWM Interrupt Event (EPWMx_INT) Counter Register These bits indicate how many selected ETSEL\\[INTSEL\\]
events have occurred. These bits are automatically cleared when an interrupt pulse is generated. If interrupts are disabled, ETSEL\\[INT\\]
= 0 or the interrupt flag is set, ETFLG\\[INT\\]
= 1, the counter will stop counting events when it reaches the period value ETPS\\[INTCNT\\]
= ETPS\\[INTPRD\\]. 0 No events have occurred. 1h 1 event has occurred. 2h 2 events have occurred. 3h 3 events have occurred."]
    #[inline(always)]
    pub fn etps_intcnt(&self) -> EtpsIntcntR {
        EtpsIntcntR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
ePWM ADC Start-of-Conversion A Event (EPWMxSOCA) Period Select These bits determine how many selected ETSEL\\[SOCASEL\\]
events need to occur before an EPWMxSOCA pulse is generated. To be generated, the pulse must be enabled (ETSEL\\[SOCAEN\\]
= 1). The SOCA pulse will be generated even if the status flag is set from a previous start of conversion (ETFLG\\[SOCA\\]
= 1). Once the SOCA pulse is generated, the ETPS\\[SOCACNT\\]
bits will automatically be cleared. 0 Disable the SOCA event counter. No EPWMxSOCA pulse will be generated 1h Generate the EPWMxSOCA pulse on the first event: ETPS\\[SOCACNT\\]
= 0,1 2h Generate the EPWMxSOCA pulse on the second event: ETPS\\[SOCACNT\\]
= 1,0 3h Generate the EPWMxSOCA pulse on the third event: ETPS\\[SOCACNT\\]
= 1,1"]
    #[inline(always)]
    pub fn etps_socaprd(&self) -> EtpsSocaprdR {
        EtpsSocaprdR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
ePWM ADC Start-of-Conversion A Event (EPWMxSOCA) Counter Register These bits indicate how many selected ETSEL\\[SOCASEL\\]
events have occurred: 0 No events have occurred. 1h 1 event has occurred. 2h 2 events have occurred. 3h 3 events have occurred."]
    #[inline(always)]
    pub fn etps_socacnt(&self) -> EtpsSocacntR {
        EtpsSocacntR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
ePWM ADC Start-of-Conversion B Event (EPWMxSOCB) Period Select These bits determine how many selected ETSEL\\[SOCBSEL\\]
events need to occur before an EPWMxSOCB pulse is generated. To be generated, the pulse must be enabled (ETSEL\\[SOCBEN\\]
= 1). The SOCB pulse will be generated even if the status flag is set from a previous start of conversion (ETFLG\\[SOCB\\]
= 1). Once the SOCB pulse is generated, the ETPS\\[SOCBCNT\\]
bits will automatically be cleared. 0 Disable the SOCB event counter. No EPWMxSOCB pulse will be generated 1h Generate the EPWMxSOCB pulse on the first event: ETPS\\[SOCBCNT\\]
= 0,1 2h Generate the EPWMxSOCB pulse on the second event: ETPS\\[SOCBCNT\\]
= 1,0 3h Generate the EPWMxSOCB pulse on the third event: ETPS\\[SOCBCNT\\]
= 1,1"]
    #[inline(always)]
    pub fn etps_socbprd(&self) -> EtpsSocbprdR {
        EtpsSocbprdR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
ePWM ADC Start-of-Conversion B Event (EPWMxSOCB) Counter Register These bits indicate how many selected ETSEL\\[SOCBSEL\\]
events have occurred: 0 No events have occurred. 1h 1 event has occurred. 2h 2 events have occurred. 3h 3 events have occurred"]
    #[inline(always)]
    pub fn etps_socbcnt(&self) -> EtpsSocbcntR {
        EtpsSocbcntR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Latched ePWM Interrupt (EPWMx_INT) Status Flag 0 Indicates no event occurred 1 Indicates that an ePWMx interrupt (EWPMx_INT) was generated. No further interrupts will be generated until the flag bit is cleared. Up to one interrupt can be pending while the ETFLG\\[INT\\]
bit is still set. If an interrupt is pending, it will not be generated until after the ETFLG\\[INT\\]
bit is cleared"]
    #[inline(always)]
    pub fn etflg_int(&self) -> EtflgIntR {
        EtflgIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Latched ePWM ADC Start-of-Conversion A (EPWMxSOCA) Status Flag Unlike the ETFLG\\[INT\\]
flag, the EPWMxSOCA output will continue to pulse even if the flag bit is set. 0 Indicates no event occurred 1 Indicates that a start of conversion pulse was generated on EPWMxSOCA. The EPWMxSOCA output will continue to be generated even if the flag bit is set"]
    #[inline(always)]
    pub fn etflg_soca(&self) -> EtflgSocaR {
        EtflgSocaR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Latched ePWM ADC Start-of-Conversion B (EPWMxSOCB) Status Flag 0 Indicates no EPWMxSOCB event occurred 1 Indicates that a start of conversion pulse was generated on EPWMxSOCB. The EPWMxSOCB output will continue to be generated even if the flag bit is set."]
    #[inline(always)]
    pub fn etflg_socb(&self) -> EtflgSocbR {
        EtflgSocbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
ePWM Interrupt (EPWMx_INT) Period Select These bits determine how many selected ETSEL\\[INTSEL\\]
events need to occur before an interrupt is generated. To be generated, the interrupt must be enabled (ETSEL\\[INT\\]
= 1). If the interrupt status flag is set from a previous interrupt (ETFLG\\[INT\\]
= 1) then no interrupt will be generated until the flag is cleared via the ETCLR\\[INT\\]
bit. This allows for one interrupt to be pending while another is still being serviced. Once the interrupt is generated, the ETPS\\[INTCNT\\]
bits will automatically be cleared. Writing a INTPRD value that is the same as the current counter value will trigger an interrupt if it is enabled and the status flag is clear. Writing a INTPRD value that is less than the current counter value will result in an undefined state. If a counter event occurs at the same instant as a new zero or non-zero INTPRD value is written, the counter is incremented. 0 Disable the interrupt event counter. No interrupt will be generated and ETFRC\\[INT\\]
is ignored. 1h Generate an interrupt on the first event INTCNT = 01 (first event) 2h Generate interrupt on ETPS\\[INTCNT\\]
= 1,0 (second event) 3h Generate interrupt on ETPS\\[INTCNT\\]
= 1,1 (third event)"]
    #[inline(always)]
    #[must_use]
    pub fn etps_intprd(&mut self) -> EtpsIntprdW<EtpsEtflgSpec> {
        EtpsIntprdW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
ePWM Interrupt Event (EPWMx_INT) Counter Register These bits indicate how many selected ETSEL\\[INTSEL\\]
events have occurred. These bits are automatically cleared when an interrupt pulse is generated. If interrupts are disabled, ETSEL\\[INT\\]
= 0 or the interrupt flag is set, ETFLG\\[INT\\]
= 1, the counter will stop counting events when it reaches the period value ETPS\\[INTCNT\\]
= ETPS\\[INTPRD\\]. 0 No events have occurred. 1h 1 event has occurred. 2h 2 events have occurred. 3h 3 events have occurred."]
    #[inline(always)]
    #[must_use]
    pub fn etps_intcnt(&mut self) -> EtpsIntcntW<EtpsEtflgSpec> {
        EtpsIntcntW::new(self, 2)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<EtpsEtflgSpec> {
        Reserved3W::new(self, 4)
    }
    #[doc = "Bits 8:9 - 9:8\\]
ePWM ADC Start-of-Conversion A Event (EPWMxSOCA) Period Select These bits determine how many selected ETSEL\\[SOCASEL\\]
events need to occur before an EPWMxSOCA pulse is generated. To be generated, the pulse must be enabled (ETSEL\\[SOCAEN\\]
= 1). The SOCA pulse will be generated even if the status flag is set from a previous start of conversion (ETFLG\\[SOCA\\]
= 1). Once the SOCA pulse is generated, the ETPS\\[SOCACNT\\]
bits will automatically be cleared. 0 Disable the SOCA event counter. No EPWMxSOCA pulse will be generated 1h Generate the EPWMxSOCA pulse on the first event: ETPS\\[SOCACNT\\]
= 0,1 2h Generate the EPWMxSOCA pulse on the second event: ETPS\\[SOCACNT\\]
= 1,0 3h Generate the EPWMxSOCA pulse on the third event: ETPS\\[SOCACNT\\]
= 1,1"]
    #[inline(always)]
    #[must_use]
    pub fn etps_socaprd(&mut self) -> EtpsSocaprdW<EtpsEtflgSpec> {
        EtpsSocaprdW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
ePWM ADC Start-of-Conversion A Event (EPWMxSOCA) Counter Register These bits indicate how many selected ETSEL\\[SOCASEL\\]
events have occurred: 0 No events have occurred. 1h 1 event has occurred. 2h 2 events have occurred. 3h 3 events have occurred."]
    #[inline(always)]
    #[must_use]
    pub fn etps_socacnt(&mut self) -> EtpsSocacntW<EtpsEtflgSpec> {
        EtpsSocacntW::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
ePWM ADC Start-of-Conversion B Event (EPWMxSOCB) Period Select These bits determine how many selected ETSEL\\[SOCBSEL\\]
events need to occur before an EPWMxSOCB pulse is generated. To be generated, the pulse must be enabled (ETSEL\\[SOCBEN\\]
= 1). The SOCB pulse will be generated even if the status flag is set from a previous start of conversion (ETFLG\\[SOCB\\]
= 1). Once the SOCB pulse is generated, the ETPS\\[SOCBCNT\\]
bits will automatically be cleared. 0 Disable the SOCB event counter. No EPWMxSOCB pulse will be generated 1h Generate the EPWMxSOCB pulse on the first event: ETPS\\[SOCBCNT\\]
= 0,1 2h Generate the EPWMxSOCB pulse on the second event: ETPS\\[SOCBCNT\\]
= 1,0 3h Generate the EPWMxSOCB pulse on the third event: ETPS\\[SOCBCNT\\]
= 1,1"]
    #[inline(always)]
    #[must_use]
    pub fn etps_socbprd(&mut self) -> EtpsSocbprdW<EtpsEtflgSpec> {
        EtpsSocbprdW::new(self, 12)
    }
    #[doc = "Bits 14:15 - 15:14\\]
ePWM ADC Start-of-Conversion B Event (EPWMxSOCB) Counter Register These bits indicate how many selected ETSEL\\[SOCBSEL\\]
events have occurred: 0 No events have occurred. 1h 1 event has occurred. 2h 2 events have occurred. 3h 3 events have occurred"]
    #[inline(always)]
    #[must_use]
    pub fn etps_socbcnt(&mut self) -> EtpsSocbcntW<EtpsEtflgSpec> {
        EtpsSocbcntW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Latched ePWM Interrupt (EPWMx_INT) Status Flag 0 Indicates no event occurred 1 Indicates that an ePWMx interrupt (EWPMx_INT) was generated. No further interrupts will be generated until the flag bit is cleared. Up to one interrupt can be pending while the ETFLG\\[INT\\]
bit is still set. If an interrupt is pending, it will not be generated until after the ETFLG\\[INT\\]
bit is cleared"]
    #[inline(always)]
    #[must_use]
    pub fn etflg_int(&mut self) -> EtflgIntW<EtpsEtflgSpec> {
        EtflgIntW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<EtpsEtflgSpec> {
        Reserved1W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Latched ePWM ADC Start-of-Conversion A (EPWMxSOCA) Status Flag Unlike the ETFLG\\[INT\\]
flag, the EPWMxSOCA output will continue to pulse even if the flag bit is set. 0 Indicates no event occurred 1 Indicates that a start of conversion pulse was generated on EPWMxSOCA. The EPWMxSOCA output will continue to be generated even if the flag bit is set"]
    #[inline(always)]
    #[must_use]
    pub fn etflg_soca(&mut self) -> EtflgSocaW<EtpsEtflgSpec> {
        EtflgSocaW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Latched ePWM ADC Start-of-Conversion B (EPWMxSOCB) Status Flag 0 Indicates no EPWMxSOCB event occurred 1 Indicates that a start of conversion pulse was generated on EPWMxSOCB. The EPWMxSOCB output will continue to be generated even if the flag bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn etflg_socb(&mut self) -> EtflgSocbW<EtpsEtflgSpec> {
        EtflgSocbW::new(self, 19)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<EtpsEtflgSpec> {
        Reserved2W::new(self, 20)
    }
}
#[doc = "Event-Trigger Pre-Scale Register/ Event-Trigger Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etps_etflg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etps_etflg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtpsEtflgSpec;
impl crate::RegisterSpec for EtpsEtflgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etps_etflg::R`](R) reader structure"]
impl crate::Readable for EtpsEtflgSpec {}
#[doc = "`write(|w| ..)` method takes [`etps_etflg::W`](W) writer structure"]
impl crate::Writable for EtpsEtflgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETPS_ETFLG to value 0"]
impl crate::Resettable for EtpsEtflgSpec {
    const RESET_VALUE: u32 = 0;
}
