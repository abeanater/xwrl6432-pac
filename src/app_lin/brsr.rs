#[doc = "Register `BRSR` reader"]
pub type R = crate::R<BrsrSpec>;
#[doc = "Register `BRSR` writer"]
pub type W = crate::W<BrsrSpec>;
#[doc = "Field `SCI_LIN_PSL` reader - 15:0\\]
PRESCALER P (Low Bits). SCI/LIN 24-bit Integer Prescaler Selection. These bits are used to select a baudrate for the SCI/LIN module. These bits are effective in LIN mode and SCI compatible mode. The SCI/LIN has an internally generated serial clock determined by the LIN module input clock and the prescalers P and M in this register. The SCI/LIN uses the 24-bit integer prescaler P value to select 1 of over 16,700,000 available baud rates. The additional 4-bit fractional prescaler M refines the baudate selection."]
pub type SciLinPslR = crate::FieldReader<u16>;
#[doc = "Field `SCI_LIN_PSL` writer - 15:0\\]
PRESCALER P (Low Bits). SCI/LIN 24-bit Integer Prescaler Selection. These bits are used to select a baudrate for the SCI/LIN module. These bits are effective in LIN mode and SCI compatible mode. The SCI/LIN has an internally generated serial clock determined by the LIN module input clock and the prescalers P and M in this register. The SCI/LIN uses the 24-bit integer prescaler P value to select 1 of over 16,700,000 available baud rates. The additional 4-bit fractional prescaler M refines the baudate selection."]
pub type SciLinPslW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SCI_LIN_PSH` reader - 23:16\\]
PRESCALER P (High Bits). SCI/LIN 24-bit Integer Prescaler Selection. These bits are used to select a baudrate for the SCI/LIN module. These bits are effective in LIN mode and SCI compatible mode. The SCI/LIN has an internally generated serial clock determined by the LIN module input clock and the prescalers P and M in this register. The SCI/LIN uses the 24-bit integer prescaler P value to select 1 of over 16,700,000 available baud rates. The additional 4-bit fractional prescaler M refines the baudate selection."]
pub type SciLinPshR = crate::FieldReader;
#[doc = "Field `SCI_LIN_PSH` writer - 23:16\\]
PRESCALER P (High Bits). SCI/LIN 24-bit Integer Prescaler Selection. These bits are used to select a baudrate for the SCI/LIN module. These bits are effective in LIN mode and SCI compatible mode. The SCI/LIN has an internally generated serial clock determined by the LIN module input clock and the prescalers P and M in this register. The SCI/LIN uses the 24-bit integer prescaler P value to select 1 of over 16,700,000 available baud rates. The additional 4-bit fractional prescaler M refines the baudate selection."]
pub type SciLinPshW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `M` reader - 27:24\\]
SCI/LIN 4-bit Fractional Divider Selection. (M) These bits are effective in LIN or SCI asynchronous mode. These bits are used to select a baud rate for the SCI/LIN module, and they are a fractional part for the baud rate specification. The M divider allows fine-tuning of the baud rate over the P prescaler with 15 additional intermediate values for each of the P integer values."]
pub type MR = crate::FieldReader;
#[doc = "Field `M` writer - 27:24\\]
SCI/LIN 4-bit Fractional Divider Selection. (M) These bits are effective in LIN or SCI asynchronous mode. These bits are used to select a baud rate for the SCI/LIN module, and they are a fractional part for the baud rate specification. The M divider allows fine-tuning of the baud rate over the P prescaler with 15 additional intermediate values for each of the P integer values."]
pub type MW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `U` reader - 30:28\\]
Superfractional Divider Selection. (U) These bits are an additional fractional part for the baudrate specification. These bits allow a super fine tuning of the fractional baudrate with 7 more intermediate values for each of the M fractional divider values. See the Superfractional Divider section for more details."]
pub type UR = crate::FieldReader;
#[doc = "Field `U` writer - 30:28\\]
Superfractional Divider Selection. (U) These bits are an additional fractional part for the baudrate specification. These bits allow a super fine tuning of the fractional baudrate with 7 more intermediate values for each of the M fractional divider values. See the Superfractional Divider section for more details."]
pub type UW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
PRESCALER P (Low Bits). SCI/LIN 24-bit Integer Prescaler Selection. These bits are used to select a baudrate for the SCI/LIN module. These bits are effective in LIN mode and SCI compatible mode. The SCI/LIN has an internally generated serial clock determined by the LIN module input clock and the prescalers P and M in this register. The SCI/LIN uses the 24-bit integer prescaler P value to select 1 of over 16,700,000 available baud rates. The additional 4-bit fractional prescaler M refines the baudate selection."]
    #[inline(always)]
    pub fn sci_lin_psl(&self) -> SciLinPslR {
        SciLinPslR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
PRESCALER P (High Bits). SCI/LIN 24-bit Integer Prescaler Selection. These bits are used to select a baudrate for the SCI/LIN module. These bits are effective in LIN mode and SCI compatible mode. The SCI/LIN has an internally generated serial clock determined by the LIN module input clock and the prescalers P and M in this register. The SCI/LIN uses the 24-bit integer prescaler P value to select 1 of over 16,700,000 available baud rates. The additional 4-bit fractional prescaler M refines the baudate selection."]
    #[inline(always)]
    pub fn sci_lin_psh(&self) -> SciLinPshR {
        SciLinPshR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
SCI/LIN 4-bit Fractional Divider Selection. (M) These bits are effective in LIN or SCI asynchronous mode. These bits are used to select a baud rate for the SCI/LIN module, and they are a fractional part for the baud rate specification. The M divider allows fine-tuning of the baud rate over the P prescaler with 15 additional intermediate values for each of the P integer values."]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Superfractional Divider Selection. (U) These bits are an additional fractional part for the baudrate specification. These bits allow a super fine tuning of the fractional baudrate with 7 more intermediate values for each of the M fractional divider values. See the Superfractional Divider section for more details."]
    #[inline(always)]
    pub fn u(&self) -> UR {
        UR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
PRESCALER P (Low Bits). SCI/LIN 24-bit Integer Prescaler Selection. These bits are used to select a baudrate for the SCI/LIN module. These bits are effective in LIN mode and SCI compatible mode. The SCI/LIN has an internally generated serial clock determined by the LIN module input clock and the prescalers P and M in this register. The SCI/LIN uses the 24-bit integer prescaler P value to select 1 of over 16,700,000 available baud rates. The additional 4-bit fractional prescaler M refines the baudate selection."]
    #[inline(always)]
    #[must_use]
    pub fn sci_lin_psl(&mut self) -> SciLinPslW<BrsrSpec> {
        SciLinPslW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
PRESCALER P (High Bits). SCI/LIN 24-bit Integer Prescaler Selection. These bits are used to select a baudrate for the SCI/LIN module. These bits are effective in LIN mode and SCI compatible mode. The SCI/LIN has an internally generated serial clock determined by the LIN module input clock and the prescalers P and M in this register. The SCI/LIN uses the 24-bit integer prescaler P value to select 1 of over 16,700,000 available baud rates. The additional 4-bit fractional prescaler M refines the baudate selection."]
    #[inline(always)]
    #[must_use]
    pub fn sci_lin_psh(&mut self) -> SciLinPshW<BrsrSpec> {
        SciLinPshW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
SCI/LIN 4-bit Fractional Divider Selection. (M) These bits are effective in LIN or SCI asynchronous mode. These bits are used to select a baud rate for the SCI/LIN module, and they are a fractional part for the baud rate specification. The M divider allows fine-tuning of the baud rate over the P prescaler with 15 additional intermediate values for each of the P integer values."]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> MW<BrsrSpec> {
        MW::new(self, 24)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Superfractional Divider Selection. (U) These bits are an additional fractional part for the baudrate specification. These bits allow a super fine tuning of the fractional baudrate with 7 more intermediate values for each of the M fractional divider values. See the Superfractional Divider section for more details."]
    #[inline(always)]
    #[must_use]
    pub fn u(&mut self) -> UW<BrsrSpec> {
        UW::new(self, 28)
    }
}
#[doc = "The BRSR register is used to configure the baud rate of the LIN module.\n\nYou can [`read`](crate::Reg::read) this register and get [`brsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrsrSpec;
impl crate::RegisterSpec for BrsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brsr::R`](R) reader structure"]
impl crate::Readable for BrsrSpec {}
#[doc = "`write(|w| ..)` method takes [`brsr::W`](W) writer structure"]
impl crate::Writable for BrsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRSR to value 0"]
impl crate::Resettable for BrsrSpec {
    const RESET_VALUE: u32 = 0;
}
